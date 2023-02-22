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

use serde::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Svg {
	pub x:        String,
	pub y:        String,
	pub width:    String,
	pub height:   String,
	#[serde(rename = "$body")]
	pub elements: Vec<Element>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Element {
	G(G),
	Circle(Circle),
	Ellipse(Ellipse),
	Image(Image),
	Line(Line),
	Path(Path),
	Polygon(Polygon),
	Polyline(Polyline),
	Rect(Rect),
	Text(Text),
	TextPath
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct G {
	#[serde(rename = "$body")]
	pub elements: Vec<Element>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rect {
	pub x:      String,
	pub y:      String,
	pub width:  String,
	pub height: String,
	pub rx:     String,
	pub ry:     String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Circle {
	pub cx:     String,
	pub cy:     String,
	pub r:      String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ellipse {
	pub cx:     String,
	pub cy:     String,
	pub rx:     String,
	pub ry:     String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
	pub x1:          String,
	pub y1:          String,
	pub x2:          String,
	pub y2:          String,
	pub path_length: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Polyline {
	pub path_length: String,
	pub points:      String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Polygon {
	pub path_length: String,
	pub points:      String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
	pub d:           String,
	pub path_length: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
	pub length_adjust: String,
	pub x:             String,
	pub y:             String,
	pub dx:            String,
	pub dy:            String,
	pub rotate:        String,
	pub text_length:   String,
	#[serde(rename = "$body")]
	pub body:          String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
	pub x:                     String,
	pub y:                     String,
	pub width:                 String,
	pub height:                String,
	pub preserve_aspect_ratio: String,
	pub href:                  String,
}