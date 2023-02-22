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
	crate::utils::DeError,
	std::{io, collections::HashMap},
	serde::*
};

pub const BINARY_GLTF_MAGIC: [u8; 4] = *b"glTF";
pub const CHUNK_TYPE_JSON:   [u8; 4] = *b"JSON";
pub const CHUNK_TYPE_BIN:    [u8; 4] = *b"BIN\0";

pub type OpenGlBufferTarget  = usize;
pub type OpenGlComponentType = usize;
pub type OpenGlConstant      = usize;

pub fn read(mut reader: impl io::Read) -> Result<Document, DeError> {
	let mut buf = [0u8; 4];
	reader.read_exact(&mut buf).map_err(DeError::Io)?;
	
	match buf {
		[b'{', ..] | [0xEF, 0xBB, 0xBF, ..] => {
			let mut buf = buf.to_vec();
			reader.read_to_end(&mut buf).map_err(DeError::Io)?;
			crate::json::deserialize(&mut buf.as_slice())
		}
		BINARY_GLTF_MAGIC => {
			let mut buf = [0u8; 8];
			reader.read_exact(&mut buf).map_err(DeError::Io)?;
			
			let version    = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
			let mut length = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) - 12;
			let mut chunks = Vec::new();
			let mut doc    = Option::<Document>::None;
			
			while length > 0 {
				reader.read_exact(&mut buf).map_err(DeError::Io)?;
				let chunk_length = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
				length -= chunk_length;
				
				match [buf[4], buf[5], buf[6], buf[7]] {
					CHUNK_TYPE_JSON => {
						let mut buf = vec![0u8; chunk_length as usize];
						reader.read_exact(&mut buf).map_err(DeError::Io)?;
						
						if doc.is_none() {
							doc = Some(crate::json::deserialize(&mut buf.as_slice())?);
						}
					},
					CHUNK_TYPE_BIN => {
						let mut buf = vec![0u8; chunk_length as usize];
						reader.read_exact(&mut buf).map_err(DeError::Io)?;
						chunks.push(buf);
					}
					_ => return Err(DeError::Io(io::Error::new(io::ErrorKind::InvalidData, "invalid chunk type")))
				}
			}
			
			match doc {
				Some(mut doc) => {
					doc.version = Some(version);
					doc.binary_buffers = chunks;
					Ok(doc)
				}
				None => Err(DeError::Io(io::Error::new(io::ErrorKind::InvalidData, "no JSON chunk present")))
			}
		}
		_ => Err(DeError::Io(io::Error::new(io::ErrorKind::InvalidData, "invalid file")))
	}
}

pub fn write(writer: impl io::Write, doc: &Document) -> Result<(), crate::utils::SerError> {
	if doc.binary_buffers.is_empty() {
		crate::json::serialize(writer, doc)
	} else {
		unimplemented!()
	}
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
	#[serde(skip)]
	pub version:             Option<u32>,
	#[serde(skip)]
	pub binary_buffers:      Vec<Vec<u8>>,
	pub scene:               usize,
	#[serde(default)]
	pub scenes:              Vec<Scene>,
	#[serde(default)]
	pub nodes:               Vec<Node>,
	#[serde(default)]
	pub meshes:              Vec<Mesh>,
	#[serde(default)]
	pub buffers:             Vec<Buffer>,
	#[serde(default)]
	pub buffer_views:        Vec<BufferView>,
	#[serde(default)]
	pub accessors:           Vec<Accessor>,
	#[serde(default)]
	pub cameras:             Vec<Camera>,
	#[serde(default)]
	pub textures:            Vec<Texture>,
	#[serde(default)]
	pub samplers:            Vec<Sampler>,
	#[serde(default)]
	pub extensions_used:     Vec<String>,
	#[serde(default)]
	pub extensions_required: Vec<String>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Scene {
	pub nodes: Vec<usize>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Node {
	pub name:        Option<String>,
	pub matrix:      Option<[f64; 16]>,
	pub translation: Option<[f64; 3]>,
	pub rotation:    Option<[f64; 4]>,
	pub scale:       Option<[f64; 3]>,
	pub mesh:        Option<usize>,
	pub skin:        Option<usize>,
	pub camera:      Option<usize>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Mesh {
	pub primitives: Vec<Primitive>,
	#[serde(default)]
	pub weights:    Vec<f64>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Primitive {
	pub mode:       MeshPrimitiveRenderingMode,
	pub indices:    usize,
	pub attributes: HashMap<String, usize>,
	pub material:   usize,
	#[serde(default)]
	pub targets:    Vec<HashMap<String, usize>>
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MeshPrimitiveRenderingMode {
	Points,
	Lines,
	Triangles
}

impl Default for MeshPrimitiveRenderingMode {
	fn default() -> Self {
		Self::Triangles
	}
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
	pub byte_length: usize,
	pub uri:         String
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
	pub buffer:      usize,
	pub byte_offset: usize,
	pub byte_length: usize,
	pub byte_stride: usize,
	pub target:      OpenGlBufferTarget
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
	pub buffer_view:    usize,
	pub byte_offset:    usize,
	pub r#type:         String,
	pub component_type: OpenGlComponentType,
	pub count:          usize,
	pub sparse:         Option<AccessorSparse>,
	pub min:            Vec<f64>,
	pub max:            Vec<f32>
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct AccessorSparse {
	pub count:   usize,
	pub values:  AccessorSparseValues,
	pub indices: AccessorSparseIndices
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessorSparseValues {
	pub buffer_view: usize
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessorSparseIndices {
	pub buffer_view:    usize,
	pub component_type: OpenGlComponentType
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
	pub pbr_metallic_roughness: MaterialPbrMetallicRoughness,
	pub normal_texture:         MaterialTexture,
	pub occlusion_texture:      MaterialTexture,
	pub emissive_texture:       MaterialTexture,
	pub emissive_factors:       [f64; 3]
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialPbrMetallicRoughness {
	pub base_color_texture:         MaterialTexture,
	pub base_color_factor:          [f64; 4],
	pub metallic_roughness_texture: MaterialTexture,
	pub metallic_factor:            f64,
	pub roughness_factor:           f64
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTexture {
	pub scale:     Option<f64>,
	pub strength:  Option<f64>,
	pub index:     usize,
	pub tex_coord: usize
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
	pub r#type:       CameraType,
	pub perspective:  Option<CameraPerspective>,
	pub orthographic: Option<CameraOrthographic>
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CameraType {
	Perspective,
	Orthographic,
	#[serde(other)]
	Other
}

impl Default for CameraType {
	fn default() -> Self {
		Self::Perspective
	}
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraPerspective {
	pub aspect_ratio: f64,
	pub yfov:         f64,
	pub zfar:         Option<f64>,
	pub znear:        f64
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct CameraOrthographic {
	pub xmag:         f64,
	pub ymag:         f64,
	pub zfar:         f64,
	pub znear:        f64
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Texture {
	pub source:  usize,
	pub sampler: usize
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Image {
	Uri { uri: String },
	Buffer { buffer_view: usize, mime_type: String }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
	pub mag_filter: OpenGlConstant,
	pub min_filter: OpenGlConstant,
	pub wrap_s:     OpenGlConstant,
	pub wrap_t:     OpenGlConstant
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
	pub inverse_bind_matrices: usize,
	pub joints:                Vec<usize>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Animation {
	pub channels: Vec<AnimationChannel>,
	pub samplers: Vec<AnimationSampler>
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AnimationChannel {
	pub target:  AnimationChannelTarget,
	pub sampler: usize
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AnimationChannelTarget {
	pub node: usize,
	pub path: AnimationChannelTargetPath
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimationChannelTargetPath {
	Translation,
	Rotation,
	Scale,
	Weights
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct AnimationSampler {
	pub input:         usize,
	pub interpolation: AnimationInterpolation,
	pub output:        usize
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AnimationInterpolation {
	Linear,
	Step,
	CatmullromSpline,
	CubicSpline
}

impl Default for AnimationInterpolation {
	fn default() -> Self {
		Self::Linear
	}
}

