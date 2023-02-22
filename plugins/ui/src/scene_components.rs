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

use {::scene::*, math::Vec2};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Grid {
	pub subgrid:       bool,
	pub rows:          Vec<Val>,
	pub columns:       Vec<Val>,
	pub auto_flow:     GridAutoFlow,
	pub auto_rows:     Vec<Val>,
	pub auto_columns:  Vec<Val>,
	pub place_items:   Vec2<PlaceGridItem>,
	pub place_content: Vec2<PlaceGridContent>,
	pub gap:           Vec2<Val>
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct GridItem {
	pub start:      Vec2<GridItemStart>,
	pub end:        Vec2<GridItemEnd>,
	pub place_self: Vec2<PlaceGridItem>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GridItemStart {
	Auto,
	Absolute(isize)
}

impl Default for GridItemStart {
	fn default() -> Self {
		Self::Auto
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GridItemEnd {
	Relative(usize),
	Absolute(isize)
}

impl Default for GridItemEnd {
	fn default() -> Self {
		Self::Relative(1)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlaceGridItem {
	Stretch,
	Start,
	Center,
	End
}

impl Default for PlaceGridItem {
	fn default() -> Self {
		Self::Stretch
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlaceGridContent {
	Stretch,
	Start,
	Center,
	End,
	SpaceAround,
	SpaceBetween,
	SpaceEvenly
}

impl Default for PlaceGridContent {
	fn default() -> Self {
		Self::Stretch
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GridAutoFlow {
	Row,
	Column
}

impl Default for GridAutoFlow {
	fn default() -> Self {
		Self::Row
	}
}