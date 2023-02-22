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
	super::*,
	file_formats::ttf,
	std::io::{self, Read}
};
use engine_core::BoxedFuture;

pub type Font = dyn Source<(FontDescriptor, Box<dyn FontReader>)>;

pub trait FontReader: Send + Sync {
	fn get_glyph(&mut self, ch: char) -> Option<Glyph>;
	
	fn get_kerning(&mut self, ch1: char, ch2: char) -> Option<i16>;
}

#[derive(Debug, Copy, Clone)]
pub struct FontDescriptor {
	pub ascender:    i16,
	pub descender:   i16,
	pub line_gap:    i16,
	pub glyphs:      usize,
}

#[derive(Debug, Clone)]
pub struct Glyph {
	pub metrics: GlyphMetrics,
	pub path:    Vec<PathElement>
}

#[derive(Debug, Copy, Clone)]
pub struct GlyphMetrics {
	pub advance:  Vec2<u16>,
	pub bearing:  Vec2<i16>,
	pub bbox_min: Vec2<i16>,
	pub bbox_max: Vec2<i16>
}

#[derive(Debug, Copy, Clone)]
pub enum PathElement {
	Move(Vec2<f32>),
	Curve1(Vec2<f32>),
	Curve2(Vec2<f32>, Vec2<f32>),
	Curve3(Vec2<f32>, Vec2<f32>, Vec2<f32>)
}

/// A simple source that only contains a path. This is sufficient for most use cases.
#[derive(Debug, Copy, Clone)]
pub struct PathSource<T: AsRef<std::path::Path>>(pub T);

impl<T: AsRef<std::path::Path> + std::fmt::Debug + Send + Sync> Source<(FontDescriptor, Box<dyn FontReader>)> for PathSource<T> {
	fn open(&self, _read: bool, _write: bool) -> BoxedFuture<io::Result<(FontDescriptor, Box<dyn FontReader>)>> {
		Box::pin(async move {
			let mut data = Vec::new();
			std::fs::File::open(&self.0)?.read_to_end(&mut data)?;
			// this is safe as the data the slice refers to is on the heap
			let d = unsafe { &*(data.as_slice() as *const _) };
			let font = ttf::Font::from_data(d, 0).unwrap();
			
			Ok((FontDescriptor {
				ascender:  font.ascender(),
				descender: font.descender(),
				line_gap:  font.line_gap(),
				glyphs:    font.number_of_glyphs() as _
			}, Box::new(TtfReader(data, font)) as _))
		})
	}
}

pub struct TtfReader(Vec<u8>, ttf::Font<'static>);

impl FontReader for TtfReader {
	fn get_glyph(&mut self, ch: char) -> Option<Glyph> {
		let mut path = Path(Vec::new());
		let id = self.1.glyph_index(ch).ok()?;
		let hor = self.1.glyph_hor_metrics(id).unwrap_or(ttf::HorizontalMetrics {
			advance:           0,
			left_side_bearing: 0
		});
		let ver = self.1.glyph_ver_metrics(id).unwrap_or(ttf::VerticalMetrics {
			advance:          0,
			top_side_bearing: 0
		});
		let rect = self.1.outline_glyph(id, &mut path).ok();
		
		Some(Glyph {
			metrics: GlyphMetrics {
				advance:  Vec2(hor.advance, ver.advance),
				bearing:  Vec2(hor.left_side_bearing, ver.top_side_bearing),
				bbox_min: rect.map_or(Vec2::default(), |rect| Vec2(rect.x_min, rect.y_min)),
				bbox_max: rect.map_or(Vec2::default(), |rect| Vec2(rect.x_max, rect.y_max)),
			},
			path: path.0
		})
	}
	
	fn get_kerning(&mut self, ch1: char, ch2: char) -> Option<i16> {
		self.1.glyphs_kerning(
			self.1.glyph_index(ch1).ok()?,
			self.1.glyph_index(ch2).ok()?
		).ok()
	}
}

struct Path(Vec<PathElement>);

impl ttf::OutlineBuilder for Path {
	fn move_to(&mut self, x: f32, y: f32) {
		self.0.push(PathElement::Move(Vec2(x, y)));
	}
	
	fn line_to(&mut self, x: f32, y: f32) {
		self.0.push(PathElement::Curve1(Vec2(x, y)));
	}
	
	fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
		self.0.push(PathElement::Curve2(Vec2(x1, y1), Vec2(x, y)))
	}
	
	fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
		self.0.push(PathElement::Curve3(Vec2(x1, y1), Vec2(x2, y2), Vec2(x, y)));
	}
	
	fn close(&mut self) {}
}