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

use {super::*, core::mem::{self, *}};
use std::any::TypeId;

pub trait Component: Send + Sync + Unpin + 'static {}

impl<T: Send + Sync + Unpin + 'static> Component for T {}

// Optimally, `N` should be an associated constant, but MUh UNcOnSTraInED GENerIC coNsTaNt
pub trait ComponentBundle<const N: usize>: Sized {
	const TYPES: [TypeInfo; N];
	const ID:    TypeId;
	
	fn to_ptrs(&self) -> [*const u8; N];
	
	/// # Safety
	///
	/// The pointers must be valid.
	unsafe fn from_ptrs(ptrs: [*const u8; N]) -> Self;
	
	fn from_iter() -> ComponentBundleFromIter<Self, N> {
		ComponentBundleFromIter {
			bundle: MaybeUninit::uninit(),
			idx:    0
		}
	}
	
	fn into_iter(self) -> ComponentBundleIntoIter<Self, N> {
		ComponentBundleIntoIter {
			bundle: ManuallyDrop::new(self),
			idx:    0
		}
	}
}

pub trait Components<const N: usize>: Sized {
	type Iter<'a>: 'a + Iterator<Item = *mut u8> + Sized;
	const TYPES: [TypeInfo; N];
	const ID:    TypeId;
	
	fn len(&self) -> usize;
	
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
	
	fn to_ptrs<'a>(&'a self) -> [Self::Iter<'a>; N];
	
	/// # Safety
	///
	/// The pointers must be valid.
	unsafe fn from_ptrs(len: usize, ptrs: [impl IntoIterator<Item = *const u8>; N]) -> Self;
	
	fn from_iter() -> ComponentsFromIter<Self, N> {
		ComponentsFromIter {
			components: MaybeUninit::uninit(),
			idx:        0
		}
	}
	
	fn into_iter(self) -> ComponentsIntoIter<Self, N> {
		ComponentsIntoIter {
			components: ManuallyDrop::new(self),
			idx:        0
		}
	}
}

pub trait PackedComponents<const N: usize>: Sized {
	const TYPES: [TypeInfo; N];
	const ID:    TypeId;
	
	fn len(&self) -> usize;
	
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
	
	fn to_ptrs(&self) -> [*mut [u8]; N];
	
	/// # Safety
	///
	/// The pointers must be valid.
	unsafe fn from_ptrs(len: usize, ptrs: [*mut [u8]; N]) -> Self;
}

pub struct ComponentBundleFromIter<T: ComponentBundle<N>, const N: usize> {
	bundle: MaybeUninit<T>,
	idx:    usize
}

impl<T: ComponentBundle<N>, const N: usize> ComponentBundleFromIter<T, N> {
	pub fn get(self) -> T {
		if self.idx < N {
			panic!("iterator was not consumed entirely");
		}
		
		let v = unsafe { core::ptr::read(self.bundle.as_ptr()) };
		mem::forget(self);
		v
	}
}

impl<T: ComponentBundle<N>, const N: usize> Iterator for ComponentBundleFromIter<T, N> {
	type Item = (&'static TypeInfo, *mut u8);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.idx < N {
			self.idx += 1;
			Some((sort_types(&T::TYPES)[self.idx - 1], unsafe { &*self.bundle.as_ptr() }.to_ptrs()[self.idx - 1] as _))
		} else {
			None
		}
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = N - self.idx;
		(len, Some(len))
	}
}

impl<T: ComponentBundle<N>, const N: usize> ExactSizeIterator for ComponentBundleFromIter<T, N> {}

impl<T: ComponentBundle<N>, const N: usize> Drop for ComponentBundleFromIter<T, N> {
	fn drop(&mut self) {
		if self.idx == N {
			unsafe { core::ptr::drop_in_place(self.bundle.as_mut_ptr()); }
			return;
		}
		
		#[cfg(feature = "std")]
		if self.idx != 0 && !std::thread::panicking() {
			panic!("iterator was not consumed entirely");
		}
	}
}

pub struct ComponentBundleIntoIter<T: ComponentBundle<N>, const N: usize> {
	bundle: ManuallyDrop<T>,
	idx:    usize
}

impl<T: ComponentBundle<N>, const N: usize> ComponentBundleIntoIter<T, N> {
	pub fn get(self) -> T {
		if self.idx != 0 {
			mem::forget(self);
			panic!("iterator has already been partially consumed");
		}
		
		// SAFE: this is required to move `bundle` out of self, without triggering the destructor
		// of `Self`
		let bundle = ManuallyDrop::into_inner(unsafe { core::ptr::read(&self.bundle) });
		mem::forget(self);
		bundle
	}
}

impl<T: ComponentBundle<N>, const N: usize> Iterator for ComponentBundleIntoIter<T, N> {
	type Item = (&'static TypeInfo, *const u8);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.idx < N {
			self.idx += 1;
			Some((sort_types(&T::TYPES)[self.idx - 1], self.bundle.to_ptrs()[self.idx - 1] as _))
		} else {
			None
		}
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = N - self.idx;
		(len, Some(len))
	}
}

impl<T: ComponentBundle<N>, const N: usize> ExactSizeIterator for ComponentBundleIntoIter<T, N> {}

impl<T: ComponentBundle<N>, const N: usize> Drop for ComponentBundleIntoIter<T, N> {
	fn drop(&mut self) {
		if self.idx == 0 {
			unsafe { ManuallyDrop::drop(&mut self.bundle); }
			return;
		}
		
		#[cfg(feature = "std")]
		if self.idx != N && !std::thread::panicking() {
			panic!("iterator was not consumed entirely");
		}
	}
}

pub struct ComponentsFromIter<T: Components<N>, const N: usize> {
	components: MaybeUninit<T>,
	idx:        usize
}

impl<T: Components<N>, const N: usize> ComponentsFromIter<T, N> {
	pub fn get(self) -> T {
		if self.idx < N {
			panic!("iterator was not consumed entirely");
		}
		
		let v = unsafe { core::ptr::read(self.components.as_ptr()) };
		mem::forget(self);
		v
	}
}

impl<'a, T: Components<N>, const N: usize> Iterator for ComponentsFromIter<T, N> {
	type Item = (&'static TypeInfo, T::Iter<'static>);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.idx < N {
			self.idx += 1;
			Some((sort_types(&T::TYPES)[self.idx - 1], unsafe { mem::transmute_copy(&(&*self.components.as_ptr()).to_ptrs()[self.idx - 1]) }))
		} else {
			None
		}
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = N - self.idx;
		(len, Some(len))
	}
}

impl<T: Components<N>, const N: usize> ExactSizeIterator for ComponentsFromIter<T, N> {}

impl<T: Components<N>, const N: usize> Drop for ComponentsFromIter<T, N> {
	fn drop(&mut self) {
		if self.idx == N {
			unsafe { core::ptr::drop_in_place(self.components.as_mut_ptr()); }
			return;
		}
		
		#[cfg(feature = "std")]
		if self.idx != 0 && !std::thread::panicking() {
			panic!("iterator was not consumed entirely");
		}
	}
}

pub struct ComponentsIntoIter<T: Components<N>, const N: usize> {
	components: ManuallyDrop<T>,
	idx:        usize
}

impl<'a, T: Components<N>, const N: usize> Iterator for ComponentsIntoIter<T, N> {
	type Item = (&'static TypeInfo, T::Iter<'static>);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.idx < N {
			self.idx += 1;
			Some((sort_types(&T::TYPES)[self.idx - 1], unsafe { mem::transmute_copy(&(&*self.components).to_ptrs()[self.idx - 1]) }))
		} else {
			None
		}
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = N - self.idx;
		(len, Some(len))
	}
}

impl<T: Components<N>, const N: usize> ExactSizeIterator for ComponentsIntoIter<T, N> {}

impl<T: Components<N>, const N: usize> Drop for ComponentsIntoIter<T, N> {
	fn drop(&mut self) {
		if self.idx == 0 {
			unsafe { ManuallyDrop::drop(&mut self.components); }
			return;
		}
		
		#[cfg(feature = "std")]
		if self.idx != N && !std::thread::panicking() {
			panic!("iterator was not consumed entirely");
		}
	}
}

fn sort_types<const N: usize>(types: &[TypeInfo; N]) -> [&TypeInfo; N] {
	const __INFO__: TypeInfo = TypeInfo::of::<()>();
	let mut sort = [&__INFO__; N];
	
	for i in 0..N {
		sort[i] = &types[i];
	}
	
	sort.sort_unstable();
	sort
}

mod impls {
	#![allow(clippy::int_plus_one, unused_parens, non_snake_case, const_evaluatable_unchecked)]
	use super::*;
	
	impl ComponentBundle<0> for () {
		const TYPES: [TypeInfo; 0] = [];
		const ID:    TypeId        = TypeId::of::<Self>();
		
		fn to_ptrs(&self) -> [*const u8; 0] {
			[]
		}
		
		unsafe fn from_ptrs(_ptrs: [*const u8; 0]) -> Self {}
	}
	
	impl Components<0> for () {
		type Iter<'a> = crate::utils::EmptyIter<*mut u8>;
		const TYPES: [TypeInfo; 0] = [];
		const ID:    TypeId        = TypeId::of::<Self>();
		
		fn len(&self) -> usize {
			0
		}
		
		fn to_ptrs<'a>(&'a self) -> [Self::Iter<'a>; 0] {
			[]
		}
		
		unsafe fn from_ptrs(_len: usize, _ptrs: [impl IntoIterator<Item = *const u8>; 0]) -> Self {}
	}
	
	impl PackedComponents<0> for () {
		const TYPES: [TypeInfo; 0] = [];
		const ID:    TypeId        = TypeId::of::<Self>();
		
		fn len(&self) -> usize {
			0
		}
		
		fn to_ptrs(&self) -> [*mut [u8]; 0] {
			[]
		}
		
		unsafe fn from_ptrs(_len: usize, _ptrs: [*mut [u8]; 0]) -> Self {}
	}
	
	fn to_sorted_ptrs<T, const N: usize>(types: &[TypeInfo; N], mut ptrs: [T; N]) -> [T; N] {
		unsafe {
			let mut sort: [(TypeId, T); N] = mem::MaybeUninit::uninit().assume_init();
			
			for i in 0..N {
				(&mut sort[i] as *mut (TypeId, T)).write((types[i].id, ptrs[i]));
			}
			
			sort.sort_unstable_by_key(|(ty, _)| *ty);
			
			for i in 0..N {
				(&mut ptrs[i] as *mut T).write((&sort[i].1 as *const T).read());
			}
			
			mem::forget(sort);
			ptrs
		}
	}
	
	fn from_sorted_ptrs<T, const N: usize>(types: &[TypeInfo; N], ptrs: [T; N]) -> [T; N] {
		unsafe {
			let mut sort: [(TypeId, usize); N] = mem::MaybeUninit::uninit().assume_init();
			
			for i in 0..N {
				sort[i] = (types[i].id, i);
			}
			
			sort.sort_unstable_by_key(|(ty, _)| *ty);
			
			let mut new: [T; N] = mem::MaybeUninit::uninit().assume_init();
			
			for i in 0..N {
				new[sort[i].1] = ptrs[i];
			}
			
			new
		}
	}
	
	pub struct ComponentsIter<'a>(Box<dyn Iterator<Item = *mut u8> + 'a>);
	
	impl<'a> Iterator for ComponentsIter<'a> {
		type Item = *mut u8;
		
		fn next(&mut self) -> Option<Self::Item> {
			self.0.next()
		}
	}
	
	macro_rules! impls {
		() => {};
		( $head:ident, $head2:ident, $( $tail:ident, )* ) => {
			impls!(@impl $head, $head2, $( $tail, )* );
			impls!( $( $tail, )* );
		};
		(@count $ident:ident ) => { 1 };
		(@impl $( $ident:ident, $ident2:ident, )* ) => {
			impl< $( $ident: Component, )* > ComponentBundle<{ 0 $( + impls!(@count $ident) )* }> for ( $( $ident, )* ) {
				const TYPES: [TypeInfo; { 0 $( + impls!(@count $ident) )* }] = [ $( TypeInfo::of::<$ident>(), )* ];
				const ID: TypeId = TypeId::of::<Self>();
				
				fn to_ptrs(&self) -> [*const u8; { 0 $( + impls!(@count $ident) )* }] {
					let ( $( $ident, )* ) = self;
					
					to_sorted_ptrs(&Self::TYPES, [
						$(
							$ident as *const _ as _,
						)*
					])
				}
				
				unsafe fn from_ptrs(ptrs: [*const u8; { 0 $( + impls!(@count $ident) )* }]) -> Self {
					let [ $( $ident, )* ] = from_sorted_ptrs(&Self::TYPES, ptrs);
					
					(
						$(
							core::ptr::read($ident as *const _),
						)*
					)
				}
			}
			
			impl< $( $ident2, )* $( $ident: Component, )* > Components<{ 0 $( + impls!(@count $ident) )* }> for ( $( $ident2, )* )
				where
					$( for<'a> &'a $ident2: core::iter::IntoIterator<Item = &'a $ident, IntoIter: ExactSizeIterator>, )*
					$( for<'a>     $ident2: core::iter::FromIterator<&'a $ident>, )*
			{
				type Iter<'b> = ComponentsIter<'b>;
				const TYPES: [TypeInfo; { 0 $( + impls!(@count $ident) )* }] = [ $( TypeInfo::of::<$ident>(), )* ];
				const ID: TypeId = TypeId::of::<( $( $ident, )* )>();
				
				fn len(&self) -> usize {
					self.0.into_iter().len()
				}
				
				fn to_ptrs<'b>(&'b self) -> [Self::Iter<'b>; { 0 $( + impls!(@count $ident) )* }] {
					let ( $( $ident, )* ) = self;
					
					to_sorted_ptrs(&Self::TYPES, [
						$(
							ComponentsIter(Box::new($ident.into_iter().map(|v| v as *const _ as _))),
						)*
					])
				}
				
				unsafe fn from_ptrs(_len: usize, _ptrs: [impl IntoIterator<Item = *const u8>; { 0 $( + impls!(@count $ident) )* }]) -> Self {
					unimplemented!()
				}
			}
			
			impl<$( $ident2: core::ops::Deref<Target = [$ident]>, )* $( $ident: Component, )* > PackedComponents<{ 0 $( + impls!(@count $ident) )* }> for ( $( $ident2, )* ) {
				const TYPES: [TypeInfo; { 0 $( + impls!(@count $ident) )* }] = [ $( TypeInfo::of::<$ident>(), )* ];
				const ID: TypeId = TypeId::of::<( $( $ident, )* )>();
				
				fn len(&self) -> usize {
					self.0.len()
				}
				
				fn to_ptrs(&self) -> [*mut [u8]; { 0 $( + impls!(@count $ident) )* }] {
					let ( $( $ident, )* ) = self;
					
					to_sorted_ptrs(&Self::TYPES, [
						$(
							core::ptr::slice_from_raw_parts_mut($ident.as_mut_ptr() as _, $ident.len() * core::mem::size_of::<$ident>()),
						)*
					])
				}
				
				unsafe fn from_ptrs(_len: usize, _ptrs: [*mut [u8]; { 0 $( + impls!(@count $ident) )* }]) -> Self {
					unimplemented!()
				}
			}
		};
	}
	
	impls!(C0, I0, C1, I1, C2, I2, C3, I3, C4, I4, C5, I5, C6, I6, C7, I7, C8, I8, C9, I9, C10, I10, C11, I11, C12, I12, C13, I13, C14, I14, C15, I15, );
}