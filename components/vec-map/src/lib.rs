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

#![feature(allocator_api, raw_vec_internals)]
#![warn(clippy::all)]

extern crate alloc;

use {
	core::{
		ops::{Index, IndexMut},
		ops::{Deref, DerefMut, RangeBounds},
		iter::FromIterator,
		fmt
	},
	alloc::alloc::Allocator
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VecMap<K: Ord, V, A: Allocator = std::alloc::Global>(Vec<(K, V), A>);

impl<K: Ord, V> VecMap<K, V> {
	#[inline]
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self(Vec::with_capacity(capacity))
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		self.0.capacity()
	}

	#[inline]
	pub fn reserve(&mut self, additional: usize) {
		self.0.reserve(additional);
	}

	#[inline]
	pub fn reserve_exact(&mut self, additional: usize) {
		self.0.reserve_exact(additional);
	}

	#[inline]
	pub fn shrink_to_fit(&mut self) {
		self.0.shrink_to_fit()
	}

	#[inline]
	pub fn into_boxed_slice(self) -> Box<[(K, V)]> {
		self.0.into_boxed_slice()
	}

	#[inline]
	pub fn truncate(&mut self, len: usize) {
		self.0.truncate(len)
	}

	#[inline]
	pub fn as_slice(&self) -> &[(K, V)] {
		&self.0
	}

	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut [(K, V)] {
		&mut self.0
	}

	#[inline]
	pub fn as_ptr(&self) -> *const (K, V) {
		self.0.as_ptr()
	}

	#[inline]
	pub fn as_mut_ptr(&mut self) -> *mut (K, V) {
		self.0.as_mut_ptr()
	}

	/// # Safety
	///
	/// Might cause undefined behaviour.
	#[inline]
	pub unsafe fn set_len(&mut self, new_len: usize) {
		self.0.set_len(new_len)
	}

	pub fn insert(&mut self, key: K, val: V) -> Option<V> {
		match self.0.binary_search_by_key(&&key, |(k, _)| k) {
			Ok(i) => Some(core::mem::replace(&mut self.0[i].1, val)),
			Err(i) => {
				self.0.insert(i, (key, val));
				None
			}
		}
	}

	pub fn contains(&self, key: &K) -> bool {
		self.0.binary_search_by_key(&key, |(k, _)| k).is_ok()
	}

	pub fn get(&self, key: &K) -> Option<&V> {
		match self.0.binary_search_by_key(&key, |(k, _)| k) {
			Ok(i) => Some(unsafe { &self.0.get_unchecked(i).1 }),
			Err(_) => None
		}
	}

	pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		match self.0.binary_search_by_key(&key, |(k, _)| k) {
			Ok(i) => Some(unsafe { &mut self.0.get_unchecked_mut(i).1 }),
			Err(_) => None
		}
	}

	pub fn remove(&mut self, key: &K) -> Option<V> {
		let i = self.0.binary_search_by_key(&key, |(k, _)| k).ok()?;
		Some(self.0.remove(i).1)
	}

	#[inline]
	pub fn retain(&mut self, f: impl FnMut(&(K, V)) -> bool) {
		self.0.retain(f);
	}

	#[inline]
	pub fn pop(&mut self) -> Option<(K, V)> {
		self.0.pop()
	}

	/*#[inline]
	pub fn append(&mut self, other: &mut Self) {
		unsafe {
			self.append_elements(other.as_slice() as _);
			other.set_len(0);
		}
	}

	#[inline]
	unsafe fn append_elements(&mut self, other: *const [T]) {
		let count = (*other).len();
		self.reserve(count);
		let len = self.len();
		ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr().add(len), count);
		self.len += count;
	}*/

	#[inline]
	pub fn drain(&mut self, range: impl RangeBounds<usize>) -> std::vec::Drain<'_, (K, V)> {
		self.0.drain(range)
	}

	#[inline]
	pub fn clear(&mut self) {
		self.0.clear()
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
	/*
	#[inline]
	pub fn split_off(&mut self, at: usize) -> Self {
		assert!(at <= self.len(), "`at` out of bounds");

		let other_len = self.len - at;
		let mut other = Self::with_capacity(other_len, self.buf.clone());

		// Unsafely `set_len` and copy items to `other`.
		unsafe {
			self.set_len(at);
			other.set_len(other_len);

			ptr::copy_nonoverlapping(self.as_ptr().add(at),
									 other.as_mut_ptr(),
									 other.len());
		}
		other
	}

	pub fn resize_with<F>(&mut self, new_len: usize, f: F)
		where F: FnMut() -> T
	{
		let len = self.len();
		if new_len > len {
			self.extend_with(new_len - len, ExtendFunc(f));
		} else {
			self.truncate(new_len);
		}
	}*/

	pub fn inner(&mut self) -> &mut Vec<(K, V)> {
		&mut self.0
	}

	pub fn get_or_insert(&mut self, key: K, val: V) -> &mut V {
		self.get_or_insert_with(key, || val)
	}

	pub fn get_or_insert_with(&mut self, key: K, val: impl FnOnce() -> V) -> &mut V {
		match self.0.binary_search_by_key(&&key, |(k, _)| k) {
			Ok(i) => &mut self.0[i].1,
			Err(i) => {
				self.0.insert(i, (key, val()));
				&mut self.0[i].1
			}
		}
	}

	pub fn get_index_of(&self, key: &K) -> Result<usize, usize> {
		self.0.binary_search_by_key(&key, |(k, _)| k)
	}
}

/*impl<T: Clone, A: Alloc> CustomVec<T, A> {/*
	/// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
	///
	/// If `new_len` is greater than `len`, the `Vec` is extended by the
	/// difference, with each additional slot filled with `value`.
	/// If `new_len` is less than `len`, the `Vec` is simply truncated.
	///
	/// This method requires [`Clone`] to be able clone the passed value. If
	/// you need more flexibility (or want to rely on [`Default`] instead of
	/// [`Clone`]), use [`resize_with`].
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec!["hello"];
	/// vec.resize(3, "world");
	/// assert_eq!(vec, ["hello", "world", "world"]);
	///
	/// let mut vec = vec![1, 2, 3, 4];
	/// vec.resize(2, 0);
	/// assert_eq!(vec, [1, 2]);
	/// ```
	///
	/// [`Clone`]: ../../std/clone/trait.Clone.html
	/// [`Default`]: ../../std/default/trait.Default.html
	/// [`resize_with`]: #method.resize_with
	pub fn resize(&mut self, new_len: usize, value: T) {
		let len = self.len();

		if new_len > len {
			self.extend_with(new_len - len, ExtendElement(value))
		} else {
			self.truncate(new_len);
		}
	}

	/// Clones and appends all elements in a slice to the `Vec`.
	///
	/// Iterates over the slice `other`, clones each element, and then appends
	/// it to this `Vec`. The `other` vector is traversed in-order.
	///
	/// Note that this function is same as [`extend`] except that it is
	/// specialized to work with slices instead. If and when Rust gets
	/// specialization this function will likely be deprecated (but still
	/// available).
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![1];
	/// vec.extend_from_slice(&[2, 3, 4]);
	/// assert_eq!(vec, [1, 2, 3, 4]);
	/// ```
	///
	/// [`extend`]: #method.extend
	pub fn extend_from_slice(&mut self, other: &[T]) {
		self.spec_extend(other.iter())
	}*/
}*/

impl<K: Ord, V> Default for VecMap<K, V> {
	fn default() -> Self {
		Self::new()
	}
}

impl<'a, K: Ord, V> Index<&'a K> for VecMap<K, V> {
	type Output = V;

	#[inline]
	fn index(&self, index: &'a K) -> &Self::Output {
		let i = self.0.binary_search_by_key(&index, |(k, _)| k)
			.expect("element not present");
		&self.0[i].1
	}
}

impl<'a, K: Ord, V> IndexMut<&'a K> for VecMap<K, V> {
	#[inline]
	fn index_mut(&mut self, index: &'a K) -> &mut Self::Output {
		let i = self.0.binary_search_by_key(&index, |(k, _)| k)
			.expect("element not present");
		&mut self.0[i].1
	}
}

impl<K: Ord, V> Deref for VecMap<K, V> {
	type Target = [(K, V)];

	fn deref(&self) -> &[(K, V)] {
		&*self.0
	}
}

impl<K: Ord, V> DerefMut for VecMap<K, V> {
	fn deref_mut(&mut self) -> &mut [(K, V)] {
		&mut self.0
	}
}

impl<K: Ord, V> Extend<(K, V)> for VecMap<K, V> {
	fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
		self.0.extend(iter)
	}
}

impl<K: Ord, V> FromIterator<(K, V)> for VecMap<K, V>  {
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
		let mut vec = Self::default();
		vec.extend(iter);
		vec
	}
}

impl<K: Ord, V> IntoIterator for VecMap<K, V> {
	type Item     = (K, V);
	type IntoIter = <Vec<(K, V)> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a, K: Ord, V> IntoIterator for &'a VecMap<K, V> {
	type Item     = &'a (K, V);
	type IntoIter = <&'a Vec<(K, V)> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl<'a, K: Ord, V> IntoIterator for &'a mut VecMap<K, V> {
	type Item     = &'a mut (K, V);
	type IntoIter = <&'a mut Vec<(K, V)> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter_mut()
	}
}

impl<K: Ord + fmt::Debug, V: fmt::Debug> fmt::Debug for VecMap<K, V> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_slice().fmt(f)
	}
}
