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

use core::{
	sync::atomic::{self, *},
	alloc::AllocRef,
	borrow::Borrow,
	ptr::null_mut,
	convert::TryFrom,
	fmt, ops
};

// TODO after reading the value, check if parent ptr did not change to ensure the node wasn't freed while iterating

const BLOCK_LEN: usize = 16;

pub struct ConcurrentTreeMap<K, V, A: AllocRef = std::alloc::Global> {
	root:  AtomicPtr<Node<K, V>>,
	// the cache is a cyclic linked list of unused nodes
	cache: AtomicPtr<Node<K, V>>,
	len:   AtomicUsize,
	alloc: A
}

impl<K, V> ConcurrentTreeMap<K, V, std::alloc::Global> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<K, V, A: AllocRef> ConcurrentTreeMap<K, V, A> {
	fn root(&self) -> Option<&Node<K, V>> {
		unsafe { self.root.load(Ordering::SeqCst).as_ref() }
	}
	
	fn search_tree<Q: Ord + ?Sized>(&self, key: &Q) -> Result<&Node<K, V>, Option<&Node<K, V>>> where K: Borrow<Q> {
		use core::cmp::Ordering::*;
		
		let mut root = self.root().ok_or(None)?;
		
		loop {
			match root.key.borrow().cmp(key) {
				Less    => root = root.left().ok_or(Some(root))?,
				Greater => root = root.right().ok_or(Some(root))?,
				Equal   => return Ok(root)
			}
		}
	}
	
	pub fn new_in(alloc: A) -> Self {
		Self { root: AtomicPtr::default(), cache: AtomicPtr::default(), len: AtomicUsize::default(), alloc }
	}
	
	pub fn len(&self) -> usize {
		self.len.load(Ordering::SeqCst)
	}
	
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
	
	pub fn clear(&self) {
		self.len.store(0, Ordering::SeqCst);
		let root = self.root.swap(null_mut(), Ordering::SeqCst);
		
	}
	
	pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V> where K: Borrow<Q> {
		self.get_key_value(key).map(|(_, v)| v)
	}
	
	pub fn get_key_value<Q: Ord + ?Sized>(&self, k: &Q) -> Option<(&K, &V)> where K: Borrow<Q> {
		unimplemented!()
	}
	
	pub fn first_key_value(&self) -> Option<(&K, &V)> {
		self.first_entry().map(OccupiedEntry::into_key_value)
	}
	
	pub fn first_entry(&self) -> Option<OccupiedEntry<'_, K, V, A>> {
		loop {
			if let Some(node) = self.root()?.left_edge().upgrade() {
				return Some(OccupiedEntry { root: self, node });
			}
		}
	}
	
	pub fn pop_first(&self) -> Option<(K, V)> {
		self.first_entry().map(OccupiedEntry::remove_entry)
	}
	
	pub fn last_key_value(&self) -> Option<(&K, &V)> {
		self.last_entry().map(OccupiedEntry::into_key_value)
	}
	
	pub fn last_entry(&self) -> Option<OccupiedEntry<'_, K, V, A>> {
		loop {
			if let Some(node) = self.root()?.right_edge().upgrade() {
				return Some(OccupiedEntry { root: self, node });
			}
		}
	}
	
	pub fn pop_last(&self) -> Option<(K, V)> {
		self.last_entry().map(OccupiedEntry::remove_entry)
	}
	
	pub fn contains_key<Q: Ord + ?Sized>(&self, key: &Q) -> bool where K: Borrow<Q> {
		self.get(key).is_some()
	}
	
	pub fn insert(&self, key: K, value: V) -> Option<V> where K: Ord {
		self.entry(key).insert(value)
	}
	
	pub fn remove<Q: Ord + ?Sized>(&self, key: &Q) -> Option<V> where K: Borrow<Q> {
		self.remove_entry(key).map(|(_, v)| v)
	}
	
	pub fn remove_entry<Q: Ord + ?Sized>(&self, key: &Q) -> Option<(K, V)> where K: Borrow<Q> {
		unimplemented!()
	}
	
	pub fn append(&self, other: &Self) {
		atomic::fence(Ordering::SeqCst);
		let other_root = other.root.swap(null_mut(), Ordering::Acquire);
		let other_cache = other.cache.swap(null_mut(), Ordering::Acquire);
		
		if other_root.is_null() {
			return;
		} else if self.root.compare_and_swap(null_mut(), other_root, Ordering::SeqCst).is_null() {
			if let Some(mut node) = unsafe { other.cache.compare_and_exchange(
				null_mut(), other_cache, Ordering::Release, Ordering::Acquire).as_ref() }
			{
				while let Err(next) = unsafe { node.right.compare_and_exchange(
					null_mut(), other_cache, Ordering::Release, Ordering::Acquire).as_ref() } {
					node = next;
				}
			}
			
			return;
		}
		
		unimplemented!()
		
		/*
		// First, we merge `self` and `other` into a sorted sequence in linear time.
		let self_iter = mem::take(self).into_iter();
		let other_iter = mem::take(other).into_iter();
		let iter = MergeIter { left: self_iter.peekable(), right: other_iter.peekable() };
		
		// Second, we build a tree from the sorted sequence in linear time.
		self.from_sorted_iter(iter);*/
	}
	
	/*pub fn range<T: ?Sized, R>(&self, range: R) -> Range<'_, K, V>
		where
			T: Ord,
			K: Borrow<T>,
			R: RangeBounds<T>,
	{
		if let Some(root) = &self.root {
			let (f, b) = root.node_as_ref().range_search(range);
			
			Range { front: Some(f), back: Some(b) }
		} else {
			Range { front: None, back: None }
		}
	}
	
	pub fn range_mut<T: ?Sized, R>(&mut self, range: R) -> RangeMut<'_, K, V>
		where
			T: Ord,
			K: Borrow<T>,
			R: RangeBounds<T>,
	{
		if let Some(root) = &mut self.root {
			let (f, b) = root.node_as_valmut().range_search(range);
			
			RangeMut { front: Some(f), back: Some(b), _marker: PhantomData }
		} else {
			RangeMut { front: None, back: None, _marker: PhantomData }
		}
	}*/
	
	pub fn entry(&self, key: K) -> Entry<'_, K, V, A> where K: Ord {
		loop {
			return match self.search_tree(&key) {
				Ok(node) => if let Some(node) = node.upgrade() {
					Entry::Occupied(OccupiedEntry { root: self, node })
				} else {
					continue
				},
				Err(Some(node)) => if let Some(node) = node.upgrade() {
					Entry::Vacant(VacantEntry { key, root: self, node: Some(node) })
				} else {
					continue;
				},
				Err(None) => Entry::Vacant(VacantEntry { key, root: self, node: None })
			};
		}
	}
	
	/*fn from_sorted_iter<I: Iterator<Item = (K, V)>>(&mut self, iter: I) {
		let root = Self::ensure_is_owned(&mut self.root);
		let mut cur_node = root.node_as_mut().last_leaf_edge().into_node();
		// Iterate through all key-value pairs, pushing them into nodes at the right level.
		for (key, value) in iter {
			// Try to push key-value pair into the current leaf node.
			if cur_node.len() < node::CAPACITY {
				cur_node.push(key, value);
			} else {
				// No space left, go up and push there.
				let mut open_node;
				let mut test_node = cur_node.forget_type();
				loop {
					match test_node.ascend() {
						Ok(parent) => {
							let parent = parent.into_node();
							if parent.len() < node::CAPACITY {
								// Found a node with space left, push here.
								open_node = parent;
								break;
							} else {
								// Go up again.
								test_node = parent.forget_type();
							}
						}
						Err(_) => {
							// We are at the top, create a new root node and push there.
							open_node = root.push_internal_level();
							break;
						}
					}
				}
				
				// Push key-value pair and new right subtree.
				let tree_height = open_node.height() - 1;
				let mut right_tree = node::Root::new_leaf();
				for _ in 0..tree_height {
					right_tree.push_internal_level();
				}
				open_node.push(key, value, right_tree);
				
				// Go down to the right-most leaf again.
				cur_node = open_node.forget_type().last_leaf_edge().into_node();
			}
			
			self.length += 1;
		}
		Self::fix_right_edge(root)
	}
	
	fn fix_right_edge(root: &mut node::Root<K, V>) {
		// Handle underfull nodes, start from the top.
		let mut cur_node = root.node_as_mut();
		while let Internal(internal) = cur_node.force() {
			// Check if right-most child is underfull.
			let mut last_edge = internal.last_edge();
			let right_child_len = last_edge.reborrow().descend().len();
			if right_child_len < node::MIN_LEN {
				// We need to steal.
				let mut last_kv = match last_edge.left_kv() {
					Ok(left) => left,
					Err(_) => unreachable!(),
				};
				last_kv.bulk_steal_left(node::MIN_LEN - right_child_len);
				last_edge = last_kv.right_edge();
			}
			
			// Go further down.
			cur_node = last_edge.descend();
		}
	}
	
	pub fn split_off<Q: ?Sized + Ord>(&mut self, key: &Q) -> Self
		where
			K: Borrow<Q>,
	{
		if self.is_empty() {
			return Self::new();
		}
		
		let total_num = self.len();
		let left_root = self.root.as_mut().unwrap(); // unwrap succeeds because not empty
		
		let mut right = Self::new();
		let right_root = Self::ensure_is_owned(&mut right.root);
		for _ in 0..left_root.height() {
			right_root.push_internal_level();
		}
		
		{
			let mut left_node = left_root.node_as_mut();
			let mut right_node = right_root.node_as_mut();
			
			loop {
				let mut split_edge = match search::search_node(left_node, key) {
					// key is going to the right tree
					Found(handle) => handle.left_edge(),
					GoDown(handle) => handle,
				};
				
				split_edge.move_suffix(&mut right_node);
				
				match (split_edge.force(), right_node.force()) {
					(Internal(edge), Internal(node)) => {
						left_node = edge.descend();
						right_node = node.first_edge().descend();
					}
					(Leaf(_), Leaf(_)) => {
						break;
					}
					_ => {
						unreachable!();
					}
				}
			}
		}
		
		left_root.fix_right_border();
		right_root.fix_left_border();
		
		if left_root.height() < right_root.height() {
			self.length = left_root.node_as_ref().calc_length();
			right.length = total_num - self.len();
		} else {
			right.length = right_root.node_as_ref().calc_length();
			self.length = total_num - right.len();
		}
		
		right
	}
	
	pub fn drain_filter<F>(&mut self, pred: F) -> DrainFilter<'_, K, V, F>
		where
			F: FnMut(&K, &mut V) -> bool,
	{
		DrainFilter { pred, inner: self.drain_filter_inner() }
	}
	
	pub(super) fn drain_filter_inner(&mut self) -> DrainFilterInner<'_, K, V> {
		if let Some(root) = self.root.as_mut() {
			let (root, dormant_root) = DormantMutRef::new(root);
			let front = root.node_as_mut().first_leaf_edge();
			DrainFilterInner {
				length: &mut self.length,
				dormant_root: Some(dormant_root),
				cur_leaf_edge: Some(front),
			}
		} else {
			DrainFilterInner { length: &mut self.length, dormant_root: None, cur_leaf_edge: None }
		}
	}
	
	pub fn into_keys(self) -> IntoKeys<K, V> {
		IntoKeys { inner: self.into_iter() }
	}
	
	pub fn into_values(self) -> IntoValues<K, V> {
		IntoValues { inner: self.into_iter() }
	}*/
}

impl<K, V> Default for ConcurrentTreeMap<K, V, std::alloc::Global> {
	fn default() -> Self {
		Self::new_in(std::alloc::Global)
	}
}

pub struct Node<K, V> {
	parent: AtomicPtr<Self>,
	left:   AtomicPtr<Self>,
	right:  AtomicPtr<Self>,
	cache:  *const [Self; BLOCK_LEN],
	refs:   AtomicUsize,
	key:    K,
	val:    V
	// TODO use a pointer as the value
	// val: AtomicPtr<V>
}

impl<K, V> Node<K, V> {
	fn as_key_value(&self) -> (&K, &V) {
		(&self.key, &self.val)
	}
	
	fn as_key_value_mut(&mut self) -> (&mut K, &mut V) {
		(&mut self.key, &mut self.val)
	}
	
	fn into_key_value(self) -> (K, V) {
		let Self { key, val, .. } = self;
		(key, val)
	}
	
	fn parent(&self) -> Option<&Self> {
		unsafe { self.parent.load(Ordering::SeqCst).as_ref() }
	}
	
	fn right(&self) -> Option<&Self> {
		unsafe { self.right.load(Ordering::SeqCst).as_ref() }
	}
	
	fn left(&self) -> Option<&Self> {
		unsafe { self.left.load(Ordering::SeqCst).as_ref() }
	}
	
	fn root(&self) -> &Self {
		let mut node = self;
		while let Some(next) = node.parent() { node = next; }
		node
	}
	
	fn right_edge(&self) -> &Self {
		let mut node = self;
		while let Some(next) = node.right() { node = next; }
		node
	}
	
	fn left_edge(&self) -> &Self {
		let mut node = self;
		while let Some(next) = node.left() { node = next; }
		node
	}
	
	fn upgrade(&self) -> Option<NodeRef<K, V>> {
		self.refs.fetch_add(1, Ordering::SeqCst);
		if let Some(parent) = self.parent() {
			if parent.right.load(Ordering::SeqCst) as *const _ == self as *const _
				|| parent.left.load(Ordering::SeqCst) as *const _ == self as *const _ {
				return Some(NodeRef(self))
			}
		}
		
		self.refs.fetch_sub(1, Ordering::SeqCst);
		None
	}
}

pub struct NodeRef<'a, K, V>(&'a Node<K, V>);

impl<'a, K, V> TryFrom<&'a AtomicPtr<Node<K, V>>> for NodeRef<'a, K, V> {
	type Error = ();
	
	fn try_from(value: &'a AtomicPtr<Node<K, V>>) -> Result<Self, Self::Error> {
		unsafe { value.load(Ordering::SeqCst).as_ref() }
			.filter(|node| node.refs.fetch_add(1, Ordering::SeqCst) != 0)
			.map(Self)
			.ok_or(())
	}
}

impl<'a, K, V> Drop for NodeRef<'a, K, V> {
	fn drop(&mut self) {
		let refs = self.0.refs.fetch_sub(1, Ordering::SeqCst);
		
		if refs == 0 {
			panic!("invalid reference count, expecting a value > 0");
		} else if refs == 1 {
			unimplemented!()
		}
	}
}

pub struct KeyRef<'a, K, V>(NodeRef<'a, K, V>);

impl<'a, K, V> ops::Deref for KeyRef<'a, K, V> {
	type Target = K;
	
	fn deref(&self) -> &Self::Target {
		&self.0.0.key
	}
}

pub struct ValRef<'a, K, V>(NodeRef<'a, K, V>);

impl<'a, K, V> ops::Deref for ValRef<'a, K, V> {
	type Target = V;
	
	fn deref(&self) -> &Self::Target {
		&self.0.0.val
	}
}

pub enum Entry<'a, K: 'a, V: 'a, A: AllocRef> {
	Vacant(VacantEntry<'a, K, V, A>),
	Occupied(OccupiedEntry<'a, K, V, A>),
}

impl<'a, K: Ord, V, A: AllocRef> Entry<'a, K, V, A> {
	pub fn insert(self, val: V) -> Option<V> {
		match self {
			Self::Occupied(mut entry) => Some(entry.insert(val)),
			Self::Vacant(entry) => {
				entry.insert(val);
				None
			},
		}
	}
	
	pub fn or_insert(self, default: V) -> &'a V {
		match self {
			Self::Occupied(entry) => entry.into_ref(),
			Self::Vacant(entry) => entry.insert(default),
		}
	}
	
	pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a V {
		match self {
			Self::Occupied(entry) => entry.into_ref(),
			Self::Vacant(entry) => entry.insert(default()),
		}
	}
	
	#[inline]
	pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a V {
		match self {
			Self::Occupied(entry) => entry.into_ref(),
			Self::Vacant(entry) => {
				let value = default(entry.key());
				entry.insert(value)
			}
		}
	}
	
	pub fn or_default(self) -> &'a V where V: Default {
		match self {
			Self::Occupied(entry) => entry.into_ref(),
			Self::Vacant(entry) => entry.insert(Default::default()),
		}
	}
	
	pub fn key(&self) -> &K {
		match *self {
			Self::Occupied(ref entry) => entry.key(),
			Self::Vacant(ref entry) => entry.key(),
		}
	}
	
	pub fn and_modify<F>(self, f: F) -> Self
		where
			F: FnOnce(&V),
	{
		match self {
			Self::Occupied(mut entry) => {
				f(entry.get());
				Self::Occupied(entry)
			}
			Self::Vacant(entry) => Self::Vacant(entry),
		}
	}
}

impl<K: fmt::Debug + Ord, V: fmt::Debug, A: AllocRef> fmt::Debug for Entry<'_, K, V, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
			Self::Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
		}
	}
}

pub struct VacantEntry<'a, K: 'a, V: 'a, A: AllocRef> {
	key:  K,
	root: &'a ConcurrentTreeMap<K, V, A>,
	node: Option<NodeRef<'a, K, V>>
}

impl<'a, K: Ord, V, A: AllocRef> VacantEntry<'a, K, V, A> {
	pub fn key(&self) -> &K {
		&self.key
	}
	
	pub fn into_key(self) -> K {
		self.key
	}
	
	pub fn insert(self, value: V) -> &'a V {
		match self.node {
			Some(node)
		}
	}
}

impl<K: fmt::Debug + Ord, V, A: AllocRef> fmt::Debug for VacantEntry<'_, K, V, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("VacantEntry")
			.field(self.key())
			.finish()
	}
}

pub struct OccupiedEntry<'a, K: 'a, V: 'a, A: AllocRef> {
	root: &'a ConcurrentTreeMap<K, V, A>,
	node: NodeRef<'a, K, V>
}

impl<'a, K, V, A: AllocRef> OccupiedEntry<'a, K, V, A> {
	pub fn key(&self) -> &K {
		unimplemented!()
	}
	
	pub fn remove_entry(self) -> (K, V) {
		unimplemented!()
	}
	
	pub fn get(&self) -> &V {
		unimplemented!()
	}
	
	pub fn into_ref(self) -> &'a V {
		unimplemented!()
	}
	
	pub fn into_key_value(self) -> (&'a K, &'a V) {
		unimplemented!()
	}
	
	pub fn insert(&self, value: V) -> V {
		//mem::replace(&mut self.node.0.val, ManuallyDrop::new(value))
		unimplemented!()
	}
	
	pub fn remove(self) -> V {
		self.remove_entry().1
	}
}

impl<K: fmt::Debug + Ord, V: fmt::Debug, A: AllocRef> fmt::Debug for OccupiedEntry<'_, K, V, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("OccupiedEntry")
			.field("key", self.key())
			.field("value", self.get())
			.finish()
	}
}