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

use core::{marker::PhantomData, stream::Stream, pin::Pin, task::{Context, Poll}, ops, cmp};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct EmptyStream<T>(PhantomData<T>);

impl<T> Stream for EmptyStream<T> {
	type Item = T;
	
	fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		Poll::Ready(None)
	}
}

pub fn align<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Rem<Output = T> + cmp::Eq + Copy>(
	offset: T, align: T
) -> T {
	#[allow(clippy::eq_op)]
	let zero = offset - offset;
	offset + match offset % align {
		v if v == zero => zero,
		v => align - v
	}
}

pub fn print_types(f: &mut core::fmt::Formatter, mut types: impl Iterator<Item = crate::archetype::TypeInfo>) -> core::fmt::Result {
	if let Some(ty) = types.next() {
		f.write_str(ty.name)?;
		
		for ty in types {
			f.write_str(", ")?;
			f.write_str(ty.name)?;
		}
	}
	
	Ok(())
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct EmptyIter<T>(pub core::marker::PhantomData<T>);

impl<T> Iterator for EmptyIter<T> {
	type Item = T;
	
	fn next(&mut self) -> Option<Self::Item> {
		None
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(0))
	}
}

impl<T> ExactSizeIterator for EmptyIter<T> {}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct __ForceExactSizeIterator__<I: Iterator>(pub I, pub usize);

impl<I: Iterator> Iterator for __ForceExactSizeIterator__<I> {
	type Item = I::Item;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.1, Some(self.1))
	}
}

impl<I: Iterator> ExactSizeIterator for __ForceExactSizeIterator__<I> {}