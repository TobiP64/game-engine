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
	std::{io::{self, Result, SeekFrom, Error, ErrorKind}, convert::TryFrom}
};

/// The identifier of a KTX2 file.
pub const IDENTIFIER: [u8; 12] = *b"\xABKTX 22\xBB\r\n\x1A\n";

#[cfg(feature = "vk")]
pub type Format = vk::VkFormat;
#[cfg(not(feature = "vk"))]
pub type Format = u32;

pub struct ReadHeader;
pub struct ReadIndex;
pub struct ReadDataFormatDescriptor;
pub struct ReadKeyValueData;
pub struct ReadSupercompressionGlobalData;
pub struct ReadMipLevel;

/// A KTX2 header.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Header {
	pub format:                       Format,
	pub type_size:                    u32,
	pub pixel_width:                  u32,
	pub pixel_height:                 u32,
	pub pixel_depth:                  u32,
	pub number_of_array_elements:     u32,
	pub number_of_faces:              u32,
	pub number_of_mip_levels:         u32,
	pub supercompression_scheme:      SupercompressionScheme,
	pub bytes_of_images:              u64,
	pub bytes_of_uncompressed_images: u64
}

pub struct Index {
	pub data_format_descriptor_offset:         u32,
	pub bytes_of_data_format_descriptor:       u32,
	pub key_value_data_offset:                 u32,
	pub bytes_of_key_value_data:               u32,
	pub supercompression_global_data_offset:   u32,
	pub bytes_of_supercompression_global_data: u32,
	pub levels:                                Box<[Level]>
}

impl Default for Index {
	fn default() -> Self {
		Self {
			data_format_descriptor_offset:         0,
			bytes_of_data_format_descriptor:       0,
			key_value_data_offset:                 0,
			bytes_of_key_value_data:               0,
			supercompression_global_data_offset:   0,
			bytes_of_supercompression_global_data: 0,
			levels:                                Box::new([])
		}
	}
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Level {
	pub offset:                       u64,
	pub bytes_of_images:              u64,
	pub bytes_of_uncompressed_images: u64
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SupercompressionScheme {
	None      = 0x0,
	CrunchCrn = 0x1,
	Zlib      = 0x2,
	Zstd      = 0x3
}

pub struct KeyValueIter<'a> {
	buf: &'a [u8],
	idx: usize
}

impl<'a> Iterator for KeyValueIter<'a> {
	type Item = (&'a str, &'a [u8]);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.idx >= self.buf.len() {
			return None;
		}
		
		#[allow(clippy::cast_ptr_alignment)]
		let range = u32::from_le_bytes([
			self.buf[self.idx], self.buf[self.idx + 1], self.buf[self.idx + 2], self.buf[self.idx + 3]
		]) as usize;
		self.idx += 4;
		let off = self.idx;
		let ext = self.idx + range;
		
		while self.buf[self.idx] != 0 && self.idx < ext { self.idx += 1; }
		
		let str = std::str::from_utf8(&self.buf[off..self.idx]).unwrap();
		let buf = &self.buf[self.idx + 1..ext];
		self.idx = ext;
		Some((str, buf))
	}
}

// TODO use state pattern
pub trait Read: io::Read + io::Seek + Sized {
	fn ktx2_read_header(&mut self) -> Result<Header> {
		let mut buf = [0u8; 64];
		self.read_exact(&mut buf)?;
		
		if buf[..12] != IDENTIFIER {
			return Err(Error::new(ErrorKind::InvalidData, "invalid file identifier"));
		}
		
		let buf = &buf[12..];
		Ok(Header {
			format:                       {
				#[cfg(feature = "vk")]
				let v = Format::try_from(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]))
					.map_err(|_| Error::new(ErrorKind::InvalidData, "invalid format"))?;
				
				#[cfg(not(feature = "vk"))]
				let v = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
				
				v
			},
			type_size:                    u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
			pixel_width:                  u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
			pixel_height:                 u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]),
			pixel_depth:                  u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]),
			number_of_array_elements:     u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]),
			number_of_faces:              u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]),
			number_of_mip_levels:         u32::from_le_bytes([buf[28], buf[29], buf[30], buf[31]]),
			supercompression_scheme:      match u32::from_le_bytes([buf[32], buf[33], buf[34], buf[35]]) {
				0 => SupercompressionScheme::None,
				1 => SupercompressionScheme::CrunchCrn,
				2 => SupercompressionScheme::Zlib,
				3 => SupercompressionScheme::Zstd,
				_ => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid supercompression scheme"))
			},
			bytes_of_images:              u64::from_le_bytes([
				buf[36], buf[37], buf[38], buf[39], buf[40], buf[41], buf[42], buf[43]]),
			bytes_of_uncompressed_images: u64::from_le_bytes([
				buf[44], buf[45], buf[46], buf[47], buf[48], buf[49], buf[50], buf[51]])
		})
	}
	
	fn ktx2_read_index(&mut self, header: &Header) -> Result<Index> {
		let mut buf = [0u8; 24];
		self.read_exact(&mut buf)?;
		Ok(Index {
			data_format_descriptor_offset:         u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
			bytes_of_data_format_descriptor:       u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
			key_value_data_offset:                 u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
			bytes_of_key_value_data:               u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]),
			supercompression_global_data_offset:   u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]),
			bytes_of_supercompression_global_data: u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]),
			levels: (0..header.number_of_mip_levels).map(|_| {
					self.read_exact(&mut buf)?;
					Ok(Level {
						offset:                       u64::from_le_bytes([
							buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]),
						bytes_of_images:              u64::from_le_bytes([
							buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15]]),
						bytes_of_uncompressed_images: u64::from_le_bytes([
							buf[16], buf[17], buf[18], buf[19], buf[20], buf[21], buf[22], buf[23]])
					})
			}).collect::<io::Result<Vec<Level>>>()?
				.into_boxed_slice()
		})
	}
	
	fn ktx2_read_data_format_descriptor(&mut self, _header: &Header) -> Result<()> {
		unimplemented!() // TODO
	}
	
	fn ktx2_read_key_value_data<'a>(&mut self, index: &Index, buf: &'a mut [u8]) -> Result<KeyValueIter<'a>> {
		self.read_exact(&mut buf[..index.bytes_of_key_value_data as usize])?;
		Ok(KeyValueIter {
			buf: &mut buf[..index.bytes_of_key_value_data as usize],
			idx: 0
		})
	}
	
	fn ktx2_read_supercomression_global_data(&mut self, index: &Index, buf: &mut [u8]) -> Result<()> {
		self.seek(SeekFrom::Current(index.supercompression_global_data_offset as i64))?;
		self.read_exact(&mut buf[..index.bytes_of_supercompression_global_data as usize])
	}
	
	fn ktx2_read_mip_level(&mut self, index: &Index, level: usize, buf: &mut [u8]) -> Result<()> {
		self.seek(SeekFrom::Current(index.levels[level].offset as i64))?;
		self.read_exact(&mut buf[..index.levels[level].bytes_of_images as usize])
	}
}

impl<T: io::Read + io::Seek> Read for T {}

pub trait Write: io::Write + io::Seek {
	fn ktx2_write_header(&mut self, header: &Header) -> Result<()> {
		let mut buf = [0u8; 64];
		buf[..12].copy_from_slice(&IDENTIFIER);
		buf[12..16].copy_from_slice(&(header.format as u32).to_le_bytes());
		buf[16..20].copy_from_slice(&header.type_size.to_le_bytes());
		buf[20..24].copy_from_slice(&header.pixel_width.to_le_bytes());
		buf[24..28].copy_from_slice(&header.pixel_height.to_le_bytes());
		buf[28..32].copy_from_slice(&header.pixel_depth.to_le_bytes());
		buf[32..36].copy_from_slice(&header.number_of_array_elements.to_le_bytes());
		buf[36..40].copy_from_slice(&header.number_of_faces.to_le_bytes());
		buf[40..44].copy_from_slice(&header.number_of_mip_levels.to_le_bytes());
		buf[44..48].copy_from_slice(&(header.supercompression_scheme as u32).to_le_bytes());
		buf[48..56].copy_from_slice(&header.bytes_of_images.to_le_bytes());
		buf[56..64].copy_from_slice(&header.bytes_of_uncompressed_images.to_le_bytes());
		self.write_all(&buf)
	}
	
	fn ktx2_write_index(&mut self, index: &Index) -> Result<()> {
		let mut buf = [0u8; 24];
		buf[0..4].copy_from_slice(&index.data_format_descriptor_offset.to_le_bytes());
		buf[4..8].copy_from_slice(&index.bytes_of_data_format_descriptor.to_le_bytes());
		buf[8..12].copy_from_slice(&index.key_value_data_offset.to_le_bytes());
		buf[12..16].copy_from_slice(&index.bytes_of_key_value_data.to_le_bytes());
		buf[16..20].copy_from_slice(&index.supercompression_global_data_offset.to_le_bytes());
		buf[20..24].copy_from_slice(&index.bytes_of_supercompression_global_data.to_le_bytes());
		self.write_all(&buf)?;
		
		for level in &*index.levels {
			buf[0..8].copy_from_slice(&level.offset.to_le_bytes());
			buf[8..16].copy_from_slice(&level.bytes_of_images.to_le_bytes());
			buf[16..24].copy_from_slice(&level.bytes_of_uncompressed_images.to_le_bytes());
			self.write_all(&buf)?;
		}
		
		Ok(())
	}
	
	fn ktx2_write_data_format_descriptor(&mut self, index: &mut Index, buf: &[u8]) -> Result<()> {
		//index.data_format_descriptor_offset = self.stream_position().unwrap() as u32;
		index.bytes_of_data_format_descriptor = buf.len() as u32;
		self.write_all(buf)
	}
	
	fn ktx2_write_key_value_data<'a, I: IntoIterator<Item = (&'a str, &'a [u8])>>(&mut self, _index: &Index, iter: I) -> Result<()> {
		let _iter = iter.into_iter();
		unimplemented!()
	}
	
	fn ktx2_write_supercompression_global_data(&mut self, index: &mut Index, buf: &[u8]) -> Result<()> {
		//index.supercompression_global_data_offset = self.stream_position().unwrap() as u32;
		index.bytes_of_supercompression_global_data = buf.len() as u32;
		self.write_all(buf)
	}
	
	fn ktx2_write_mip_levels<'a, I: IntoIterator<Item = &'a [u8]>>(&'a mut self, _index: &Index, _iter: I) -> Result<()> {
		unimplemented!()
	}
}

impl<T: io::Write + io::Seek> Write for T {}

#[cfg(test)]
mod tests {

}