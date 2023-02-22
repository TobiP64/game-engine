// MIT License
//
// Copyright (c) 2019-2023 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use {
	std::{
		sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}},
		any::{TypeId, Any},
		collections::BTreeMap
	},
	custom_sync::mpbc
};

pub type ObjectId = usize;
//type Callback<Args> = dyn Fn<Args, Output = Box<dyn std::future::Future<Output = ()>>>;
type ObjectStorage<T> = BTreeMap<ObjectId, Arc<RwLock<T>>>;
//type ObjectStorage2<T> = BTreeMap<ObjectId, (Arc<RwLock<T>>, RwLock<Vec<(TypeId, ObjectId)>>)>;

struct RegistryEntry {
	type_name: &'static str,
	elements:  Box<dyn Any + Send + Sync>,
	listeners: Vec<Box<dyn Fn(&Registry, TypeId, ObjectId, bool) + Send + Sync>>,
	channel:   mpbc::Sender<(TypeId, ObjectId, bool)>
}

impl RegistryEntry {
	pub fn new<T: Any + Send + Sync>() -> Self {
		let channel = mpbc::channel(0).0;
		let cloned = channel.clone();
		RegistryEntry {
			type_name: std::any::type_name::<T>(),
			elements:  Box::new(ObjectStorage::<T>::new()),
			listeners: vec![Box::new(move |_, ty, id, insert| cloned.send((ty, id, insert)))],
			channel
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TryGetError {
	ReadFailed,
	InvalidType,
	DowncastFailed,
	InvalidId,
	NoEntries
}

pub struct RegistryIter<'a, T> {
	_guard: std::sync::RwLockReadGuard<'a, RegistryEntry>,
	iter:   std::collections::btree_map::Iter<'a, ObjectId, Arc<RwLock<T>>>
}

impl<'a, T> Iterator for RegistryIter<'a, T> {
	type Item = (ObjectId, &'a Arc<RwLock<T>>);
	
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(id, obj)| (*id, obj))
	}
}

#[derive(Clone, Default)]
pub struct Registry(Arc<(RwLock<BTreeMap<TypeId, RegistryEntry>>, AtomicUsize)>);

impl Registry {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn insert<T: Any + Send + Sync>(&self, v: T) -> ObjectId {
		self.insert_(v, None)
	}
	
	pub fn insert_with_name<T: Any + Send + Sync>(&self, v: T, name: &str) -> ObjectId {
		self.insert_(v, Some(name))
	}
	
	pub fn insert_<T: Any + Send + Sync>(&self, v: T, name: Option<&str>) -> ObjectId {
		let id = self.0.1.fetch_add(1, Ordering::Relaxed);
		
		let mut guard = self.0.0.write()
			.expect("failed to write registry");
		
		let entry = guard.entry(TypeId::of::<T>())
			.or_insert_with(RegistryEntry::new::<T>);
		
		entry.elements
			.downcast_mut::<ObjectStorage<T>>()
			.expect("failed to downcast type storage")
			.insert(id, Arc::new(RwLock::new(v)));
		
		entry.listeners.iter().for_each(|f| f(self, TypeId::of::<T>(), id, true));
		
		match name {
			None => log::trace!("[REGISTRY] registered {} with id #{}", std::any::type_name::<T>(), id),
			Some(name) => log::trace!("[REGISTRY] registered {} with id #{} and name '{}'",
				std::any::type_name::<T>(), id, name)
		}
		id
	}
	
	pub fn get<T: Any + Send + Sync>(&self, id: ObjectId) -> Arc<RwLock<T>> {
		self.0.0.read()
			.expect("failed to read registry")
			.get(&TypeId::of::<T>())
			.expect("entry with type not present")
			.elements
			.downcast_ref::<ObjectStorage<T>>()
			.expect("failed to downcast type storage")
			.get(&id)
			.expect("entry with id not present")
			.clone()
	}
	
	pub fn get_all<T: Any + Send + Sync>(&self) -> RegistryIter<T> {
		unimplemented!()
	}
	
	pub fn get_first<T: Any + Send + Sync>(&self) -> (ObjectId, Arc<RwLock<T>>) {
		/*self.get_all::<T>().next()
			.map(|(id, obj)| (id, obj.clone()))
			.expect("no entries with type present")*/
		self.0.0.read()
			.expect("failed to read registry")
			.get(&TypeId::of::<T>())
			.expect("entry with type not present")
			.elements
			.downcast_ref::<ObjectStorage<T>>()
			.expect("failed to downcast type storage")
			.first_key_value()
			.map(|(id, obj)| (*id, obj.clone()))
			.expect("no entries with type present")
	}
	
	pub fn try_get<T: Any + Send + Sync>(&self, id: ObjectId) -> Result<Arc<RwLock<T>>, TryGetError> {
		Ok(self.0.0.read()
			.map_err(|_| TryGetError::ReadFailed)?
			.get(&TypeId::of::<T>())
			.ok_or(TryGetError::InvalidType)?
			.elements
			.downcast_ref::<ObjectStorage<T>>()
			.ok_or(TryGetError::DowncastFailed)?
			.get(&id)
			.ok_or(TryGetError::InvalidId)?
			.clone())
	}
	
	pub fn remove<T: Any + Send + Sync>(&self, id: ObjectId) -> Arc<RwLock<T>> {
		let mut guard = self.0.0.write()
			.expect("failed to write registry");
		
		let entry = guard.get_mut(&TypeId::of::<T>())
			.expect("entry with type not present");
		
		let v = entry.elements
			.downcast_mut::<ObjectStorage<T>>()
			.expect("failed to downcast type storage")
			.remove(&id)
			.expect("entry with id not present");
		
		entry.listeners.iter().for_each(|f| f(self, TypeId::of::<T>(), id, false));
		
		log::trace!("[REGISTRY] unregistered {} with id #{}", std::any::type_name::<T>(), id);
		v
	}
	
	pub fn add_listener<T: Any + Send + Sync, F: Fn(&Self, TypeId, ObjectId, bool) + Send + Sync + 'static>(&self, listener: F) {
		self.0.0.write()
			.expect("failed to write registry")
			.entry(TypeId::of::<T>())
			.or_insert_with(RegistryEntry::new::<T>)
			.listeners
			.push(Box::new(listener));
	}
	
	pub fn get_receiver<T: Any + Send + Sync>(&self) -> mpbc::Receiver<(TypeId, ObjectId, bool)> {
		self.0.0.write()
			.expect("failed to write registry")
			.entry(TypeId::of::<T>())
			.or_insert_with(RegistryEntry::new::<T>)
			.channel
			.receiver()
	}
}

impl std::fmt::Debug for Registry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let guard = self.0.0.read().unwrap();
		
		let mut f = f.debug_list();
		
		for (ty, entry) in guard.iter() {
			#[derive(Debug)]
			struct Entry { type_id: TypeId, type_name: &'static str }
			
			f.entry(&Entry {
				type_id:   *ty,
				type_name: entry.type_name,
			});
		}
		
		f.finish()
	}
}

#[cfg(test)]
mod tests {
	use std::{sync::{Arc, Mutex, mpsc::TryRecvError}, any::TypeId};
	
	#[test]
	fn test_one_element() {
		let registry = super::Registry::new();
		let id = registry.insert(123usize);
		assert_eq!(*registry.get::<usize>(id).read().unwrap(), 123usize);
		assert_eq!(*registry.remove::<usize>(id).read().unwrap(), 123usize);
	}
	
	#[test]
	fn test_listener() {
		let registry = super::Registry::new();
		let event = Arc::new(Mutex::new(None));
		let cloned = event.clone();
		
		registry.add_listener::<usize, _>(
			move |_, ty, id, insert| *cloned.lock().unwrap() = Some((ty, id, insert)));
		
		let id = registry.insert(123usize);
		assert_eq!(event.lock().unwrap().take(), Some((TypeId::of::<usize>(), id, true)));
		
		registry.remove::<usize>(id);
		assert_eq!(event.lock().unwrap().take(), Some((TypeId::of::<usize>(), id, false)));
	}
	
	#[test]
	fn test_channel() {
		let registry = super::Registry::new();
		let recv = registry.get_receiver::<usize>();
		
		let id = registry.insert(123usize);
		assert_eq!(recv.try_recv(), Ok((TypeId::of::<usize>(), id, true)));
		assert_eq!(recv.try_recv(), Err(TryRecvError::Empty));
		
		registry.remove::<usize>(id);
		assert_eq!(recv.try_recv(), Ok((TypeId::of::<usize>(), id, false)));
		assert_eq!(recv.try_recv(), Err(TryRecvError::Empty));
	}
}