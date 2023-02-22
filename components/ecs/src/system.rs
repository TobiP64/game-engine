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

use {crate::*, core::{any::TypeId, future::Future, stream::Stream}};

pub type BoxedFuture<'a, T> = core::pin::Pin<Box<dyn core::future::Future<Output = T> + Send + 'a>>;

pub struct SystemInfo<'a> {
	pub id:                TypeId,
	pub name:              &'a str,
	pub archetypes_access: Option<&'a [u64]>,
	pub components_read:   &'a [TypeId],
	pub components_write:  &'a [TypeId],
	pub resources_read:    &'a [TypeId],
	pub resources_write:   &'a [TypeId]
}

pub trait IntoSystem {
	type System: System;
	
	fn into(self) -> Self::System;
}

pub trait System: Send + Sync + 'static {
	type In  = ();
	type Out = ();
	
	fn info(&self) -> SystemInfo;
	
	fn run(&self, world: &World, resources: &Resources, input: Self::In) -> BoxedFuture<Self::Out>;
}

pub trait Executor<C, T> where <Self::Iter as Iterator>::Item: Future<Output = ()> {
	type Iter:   Iterator<Item: Future<Output = ()>>;
	type Stream: Stream<Item = Self::Iter>;
	
	fn new(context: C, function: T) -> Self;
	
	fn exec_iter(&mut self) -> Self::Iter;
	
	fn exec_stream(&mut self) -> Option<Self::Stream>;
}

pub trait Fetch {
	type Context;
	type Item;
	type Iter:   Iterator<Item = Self::Item>;
	type Stream: Stream<Item = Self::Iter>;
	
	/// If true, this resolver yields values endlessly
	const UNBOUND: bool;
	
	fn new(context: Self::Context) -> Self;
	
	fn iter(&mut self) -> Self::Iter;
	
	fn stream(&mut self) -> Self::Stream;
}

pub trait OutputResolver<T> {
	type Context;
	
	fn new(context: Self::Context) -> Self;
	
	fn resolve(&mut self, item: T);
}

pub trait SystemParam: Sized {
	type Fetch: Fetch<Item = Self>;
}

mod impls {
	#![allow(unused_parens, non_snake_case)]
	
	use {
		super::*,
		core::{task::*, pin::Pin}
	};
	
	pub struct ParamIter<T>(T);
	
	macro_rules! impls {
		( $head:ident, ) => {};
		( $head:ident $(, $tail:ident )*, ) => {
			impl<$head: SystemParam $(, $tail: SystemParam )* > SystemParam for ( $head $(, $tail )* )
				where
					<$head::Fetch as Fetch>::Context: Copy
					$(, $tail::Fetch: Fetch<Context = <$head::Fetch as Fetch>::Context> )*
			{
				type Fetch = ( $head::Fetch $(, $tail::Fetch )* );
			}
			
			impl<$head: Fetch<Context: Copy> $(, $tail: Fetch<Context = $head::Context> )* > Fetch for ( $head $(, $tail )* ) {
				type Context = $head::Context;
				type Item    = ( $head::Item $(, $tail::Item )* );
				type Iter    = ParamIter<( $head::Iter $(, $tail::Iter )* )>;
				type Stream  = ParamIter<( $head::Stream $(, $tail::Stream )* )>;
				
				/// If true, this fetch yields values endlessly
				const UNBOUND: bool = $head::UNBOUND $(& $tail::UNBOUND )*;
				
				fn new(context: Self::Context) -> Self {
					( $head::new(context) $(, $tail::new(context) )* )
				}
				
				fn iter(&mut self) -> Self::Iter {
					let ( $head $(, $tail )* ) = self;
					ParamIter(( $head.iter() $(, $tail.iter() )* ))
				}
				
				fn stream(&mut self) -> Self::Stream {
					let ( $head $(, $tail )* ) = self;
					ParamIter(( $head.stream() $(, $tail.stream() )* ))
				}
			}
			
			impl< $head: Iterator $(, $tail: Iterator )* > Iterator for ParamIter<( $head $(, $tail )* )> {
				type Item = ( $head::Item $(, $tail::Item )* );
				
				fn next(&mut self) -> Option<Self::Item> {
					let Self(( $head $(, $tail )* )) = self;
					Some(( $head.next()? $(, $tail.next()? )* ))
				}
			}
			
			impl< $head: Stream $(, $tail: Stream )* > Stream for ParamIter<( $head $(, $tail )* )> {
				type Item = ParamIter<( $head::Item $(, $tail::Item )* )>;
				
				fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
					unimplemented!()
				}
			}
			
			impls!( $( $tail, )* );
		};
	}
	
	impls!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, );
}