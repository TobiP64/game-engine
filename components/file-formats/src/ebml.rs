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
	crate::utils::*,
	std::{io::{self, Write, Read}, fmt},
	serde::{*, de::*, ser::*}
};

pub type ElementId = u32;

pub fn serialize<W: io::Write, T: Serialize>(writer: W, value: &T) -> Result<(), SerError> {
	value.serialize(&mut Serializer::new(writer))
}

pub fn deserialize<'de, T: Deserialize<'de>, R: io::Read + fmt::Debug>(reader: R) -> Result<T, DeError> {
	T::deserialize(&mut Deserializer::new(reader))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
	Signed(i64),
	Unsigned(u64),
	Float(f64),
	String(String),
	Date(i64),
	Master(Vec<(u32, Self)>),
	Binary(Vec<u8>)
}

pub struct Serializer<T: io::Write>(T);

impl<T: io::Write> Serializer<T> {
	pub fn new(writer: T) -> Self {
		Self(writer)
	}
}

impl<'a, T: io::Write> serde::Serializer for &'a mut Serializer<T> {
	type Ok                     = ();
	type Error                  = SerError;
	type SerializeSeq           = MasterElementSerializer<&'a mut T>;
	type SerializeTuple         = MasterElementSerializer<&'a mut T>;
	type SerializeTupleStruct   = MasterElementSerializer<&'a mut T>;
	type SerializeTupleVariant  = VariantSerializer<&'a mut T>;
	type SerializeMap           = MasterElementSerializer<&'a mut T>;
	type SerializeStruct        = MasterElementSerializer<&'a mut T>;
	type SerializeStructVariant = VariantSerializer<&'a mut T>;
	
	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.0.write_all(&[0b1000_0001, if v { 1 } else { 0 }])?;
		Ok(())
	}
	
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		self.0.write_all(&[0x81, v as _])?;
		Ok(())
	}
	
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		match v.to_be_bytes() {
			[0,  v0] => self.0.write_all(&[0x81, v0]),
			[v0, v1] => self.0.write_all(&[0x82, v0, v1])
		}?;
		Ok(())
	}
	
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		match v.to_be_bytes() {
			[0,  0,  0,  v0] => self.0.write_all(&[0x81, v0]),
			[0,  0,  v0, v1] => self.0.write_all(&[0x82, v0, v1]),
			[0,  v0, v1, v2] => self.0.write_all(&[0x83, v0, v1, v2]),
			[v0, v1, v2, v3] => self.0.write_all(&[0x84, v0, v1, v2, v3])
		}?;
		Ok(())
	}
	
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		match v.to_be_bytes() {
			[0,  0,  0,  0,  0,  0,  0,  v0] => self.0.write_all(&[0x81, v0]),
			[0,  0,  0,  0,  0,  0,  v0, v1] => self.0.write_all(&[0x82, v0, v1]),
			[0,  0,  0,  0,  0,  v0, v1, v2] => self.0.write_all(&[0x83, v0, v1, v2]),
			[0,  0,  0,  0,  v0, v1, v2, v3] => self.0.write_all(&[0x84, v0, v1, v2, v3]),
			[0,  0,  0,  v0, v1, v2, v3, v4] => self.0.write_all(&[0x85, v0, v1, v2, v3, v4]),
			[0,  0,  v0, v1, v2, v3, v4, v5] => self.0.write_all(&[0x86, v0, v1, v2, v3, v4, v5]),
			[0,  v0, v1, v2, v3, v4, v5, v6] => self.0.write_all(&[0x87, v0, v1, v2, v3, v4, v5, v6]),
			[v0, v1, v2, v3, v4, v5, v6, v7] => self.0.write_all(&[0x88, v0, v1, v2, v3, v4, v5, v6, v7])
		}?;
		Ok(())
	}
	
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.0.write_all(&[0x81, v])?;
		Ok(())
	}
	
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		match v.to_be_bytes() {
			[0,  v0] => self.0.write_all(&[0x81, v0]),
			[v0, v1] => self.0.write_all(&[0x82, v0, v1])
		}?;
		Ok(())
	}
	
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		match v.to_be_bytes() {
			[0,  0,  0,  v0] => self.0.write_all(&[0x81, v0]),
			[0,  0,  v0, v1] => self.0.write_all(&[0x82, v0, v1]),
			[0,  v0, v1, v2] => self.0.write_all(&[0x83, v0, v1, v2]),
			[v0, v1, v2, v3] => self.0.write_all(&[0x84, v0, v1, v2, v3])
		}?;
		Ok(())
	}
	
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		match v.to_be_bytes() {
			[0,  0,  0,  0,  0,  0,  0,  v0] => self.0.write_all(&[0x81, v0]),
			[0,  0,  0,  0,  0,  0,  v0, v1] => self.0.write_all(&[0x82, v0, v1]),
			[0,  0,  0,  0,  0,  v0, v1, v2] => self.0.write_all(&[0x83, v0, v1, v2]),
			[0,  0,  0,  0,  v0, v1, v2, v3] => self.0.write_all(&[0x84, v0, v1, v2, v3]),
			[0,  0,  0,  v0, v1, v2, v3, v4] => self.0.write_all(&[0x85, v0, v1, v2, v3, v4]),
			[0,  0,  v0, v1, v2, v3, v4, v5] => self.0.write_all(&[0x86, v0, v1, v2, v3, v4, v5]),
			[0,  v0, v1, v2, v3, v4, v5, v6] => self.0.write_all(&[0x87, v0, v1, v2, v3, v4, v5, v6]),
			[v0, v1, v2, v3, v4, v5, v6, v7] => self.0.write_all(&[0x88, v0, v1, v2, v3, v4, v5, v6, v7])
		}?;
		Ok(())
	}
	
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		self.0.write_all(&[0b1000_0100])?;
		self.0.write_all(&v.to_be_bytes())?;
		Ok(())
	}
	
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		self.0.write_all(&[0b1000_1000])?;
		self.0.write_all(&v.to_be_bytes())?;
		Ok(())
	}
	
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(&v.to_string())
	}
	
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		write_len(&mut self.0, v.len() as _)?;
		self.0.write_all(v.as_bytes())?;
		Ok(())
	}
	
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		write_len(&mut self.0, v.len() as _)?;
		self.0.write_all(v)?;
		Ok(())
	}
	
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.0.write_all(&[0x80])?;
		Ok(())
	}
	
	fn serialize_some<S: ?Sized + Serialize>(self, value: &S) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		self.0.write_all(&[0x80])?;
		Ok(())
	}
	
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}
	
	fn serialize_unit_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
		let mut buf = Vec::new();
		IdSerializer(&mut buf).serialize_u32(variant_index)?;
		buf.push(0x80);
		write_len(&mut self.0, buf.len() as _)?;
		self.0.write_all(&buf)?;
		Ok(())
	}
	
	fn serialize_newtype_struct<S: ?Sized + Serialize>(self, _name: &'static str, value: &S) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_newtype_variant<S: ?Sized + Serialize>(self, _name: &'static str, variant_index: u32, _variant: &'static str, value: &S) -> Result<Self::Ok, Self::Error> {
		let mut buf = Vec::new();
		IdSerializer(&mut buf).serialize_u32(variant_index)?;
		value.serialize(&mut Serializer::new(&mut buf))?;
		write_len(&mut self.0, buf.len() as _)?;
		self.0.write_all(&buf)?;
		Ok(())
	}
	
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Ok(MasterElementSerializer { writer: &mut self.0, buf: Vec::new() })
	}
	
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		self.serialize_seq(Some(len))
	}
	
	fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		self.serialize_seq(Some(len))
	}
	
	fn serialize_tuple_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Ok(VariantSerializer { inner: MasterElementSerializer { writer: &mut self.0, buf: Vec::new() }, variant: variant_index })
	}
	
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Ok(MasterElementSerializer { writer: &mut self.0, buf: Vec::new() })
	}
	
	fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		self.serialize_map(Some(len))
	}
	
	fn serialize_struct_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		Ok(VariantSerializer { inner: MasterElementSerializer { writer: &mut self.0, buf: Vec::new() }, variant: variant_index })
	}
}

pub struct MasterElementSerializer<T: io::Write> {
	writer: T,
	buf: Vec<u8>
}

impl<T: io::Write> SerializeSeq for MasterElementSerializer<T> {
	type Ok = ();
	type Error = SerError;
	
	fn serialize_element<E: ?Sized + Serialize>(&mut self, value: &E) -> Result<(), Self::Error> {
		value.serialize(&mut SeqSerializer(&mut self.buf))
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeMap::end(self)
	}
}

impl<T: io::Write> SerializeTuple for MasterElementSerializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_element<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(self, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self)
	}
}

impl<T: io::Write> SerializeTupleStruct for MasterElementSerializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(self, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self)
	}
}

impl<T: io::Write> SerializeMap for MasterElementSerializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_key<S: ?Sized + Serialize>(&mut self, key: &S) -> Result<(), Self::Error> {
		key.serialize(IdSerializer(&mut self.buf))
	}
	
	fn serialize_value<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		value.serialize(&mut Serializer::new(&mut self.buf))
	}
	
	fn end(mut self) -> Result<Self::Ok, Self::Error> {
		write_len(&mut self.writer, self.buf.len() as _)?;
		self.writer.write_all(&self.buf)?;
		Ok(())
	}
}

impl<T: io::Write> SerializeStruct for MasterElementSerializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, key: &'static str, value: &S) -> Result<(), Self::Error> {
		SerializeMap::serialize_entry(self, key, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeMap::end(self)
	}
}

pub struct VariantSerializer<T: io::Write> {
	inner:   MasterElementSerializer<T>,
	variant: u32
}

impl<T: io::Write> SerializeTupleVariant for VariantSerializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(&mut self.inner, value)
	}
	
	fn end(mut self) -> Result<Self::Ok, Self::Error> {
		let mut buf = Vec::new();
		serde::Serializer::serialize_u32(IdSerializer(&mut buf), self.variant)?;
		write_len(&mut buf, self.inner.buf.len() as _)?;
		buf.write_all(&self.inner.buf)?;
		write_len(&mut self.inner.writer, buf.len() as _)?;
		self.inner.writer.write_all(&buf)?;
		Ok(())
	}
}

impl<T: io::Write> SerializeStructVariant for VariantSerializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<V: ?Sized + Serialize>(&mut self, key: &'static str, value: &V) -> Result<(), Self::Error> {
		SerializeMap::serialize_entry(&mut self.inner, key, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeTupleVariant::end(self)
	}
}

struct IdSerializer<T: io::Write>(T);

impl<T: io::Write> serde::Serializer for IdSerializer<T> {
	type Ok                     = ();
	type Error                  = SerError;
	type SerializeSeq           = Impossible<(), SerError>;
	type SerializeTuple         = Impossible<(), SerError>;
	type SerializeTupleStruct   = Impossible<(), SerError>;
	type SerializeTupleVariant  = Impossible<(), SerError>;
	type SerializeMap           = Impossible<(), SerError>;
	type SerializeStruct        = Impossible<(), SerError>;
	type SerializeStructVariant = Impossible<(), SerError>;
	
	fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
		match v {
			v if v & 0x80 == 0 => self.0.write_all(&[0x80 | v]),
			_                  => self.0.write_all(&[0x40, v])
		}?;
		Ok(())
	}
	
	fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
		let b = v.to_be_bytes();
		match v {
			v if v & 0xFF80 == 0 => self.0.write_all(&[0x80 | b[1]]),
			v if v & 0xC000 == 0 => self.0.write_all(&[0x40 | b[0], b[1]]),
			_                    => self.0.write_all(&[0x20, b[0], b[1]])
		}?;
		Ok(())
	}
	
	fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
		let b = v.to_be_bytes();
		match v {
			v if v & 0xFFFF_FF80 == 0 => self.0.write_all(&[0x80 | b[3]]),
			v if v & 0xFFFF_C000 == 0 => self.0.write_all(&[0x40 | b[2], b[3]]),
			v if v & 0xFFE0_0000 == 0 => self.0.write_all(&[0x20 | b[1], b[2], b[3]]),
			v if v & 0xF000_0000 == 0 => self.0.write_all(&[0x10 | b[0], b[1], b[2], b[3]]),
			_                         => return Err(SerError::UnsupportedType)
		}?;
		Ok(())
	}
	
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		self.serialize_u32(v.parse().expect("string must be a number"))
	}
	
	fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i8(self,  _v: i8)  -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	
	fn serialize_some<S: ?Sized + Serialize>(self, _value: &S) -> Result<Self::Ok, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_newtype_struct<S: ?Sized + Serialize>(self, _name: &'static str, _value: &S) -> Result<Self::Ok, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_newtype_variant<S: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &S) -> Result<Self::Ok, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		Err(SerError::UnsupportedType)
	}
}

pub struct SeqSerializer<T>(T);

impl<'a, T: 'a + io::Write> serde::Serializer for &'a mut SeqSerializer<T> {
	type Ok                     = ();
	type Error                  = SerError;
	type SerializeSeq           = Impossible<(), SerError>;
	type SerializeTuple         = Impossible<(), SerError>;
	type SerializeTupleStruct   = Impossible<(), SerError>;
	type SerializeTupleVariant  = SeqSerializer<MasterElementSerializer<&'a mut T>>;
	type SerializeMap           = Impossible<(), SerError>;
	type SerializeStruct        = Impossible<(), SerError>;
	type SerializeStructVariant = SeqSerializer<MasterElementSerializer<&'a mut T>>;
	
	fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_some<V: ?Sized + Serialize>(self, _value: &V) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> { Err(SerError::UnsupportedType) }
	
	fn serialize_unit_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
		serde::Serializer::serialize_u32(IdSerializer(&mut self.0), variant_index)?;
		self.0.write_all(&[0x80])?;
		Ok(())
	}
	
	fn serialize_newtype_struct<V: ?Sized + Serialize>(self, _name: &'static str, _value: &V) -> Result<Self::Ok, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_newtype_variant<V: ?Sized + Serialize>(self, _name: &'static str, variant_index: u32, _variant: &'static str, value: &V) -> Result<Self::Ok, Self::Error> {
		IdSerializer(&mut self.0).serialize_u32(variant_index)?;
		value.serialize(&mut Serializer::new(&mut self.0))?;
		Ok(())
	}
	
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_tuple_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		IdSerializer(&mut self.0).serialize_u32(variant_index)?;
		Ok(SeqSerializer(MasterElementSerializer { writer: &mut self.0, buf: Vec::new() }))
	}
	
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		Err(SerError::UnsupportedType)
	}
	
	fn serialize_struct_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		IdSerializer(&mut self.0).serialize_u32(variant_index)?;
		Ok(SeqSerializer(MasterElementSerializer { writer: &mut self.0, buf: Vec::new() }))
	}
}

impl<T: io::Write> SerializeTupleVariant for SeqSerializer<MasterElementSerializer<T>> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<V: ?Sized + Serialize>(&mut self, value: &V) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(&mut self.0, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self.0)
	}
}

impl<T: io::Write> SerializeStructVariant for SeqSerializer<MasterElementSerializer<T>> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<V: ?Sized + Serialize>(&mut self, key: &'static str, value: &V) -> Result<(), Self::Error> {
		SerializeMap::serialize_entry(&mut self.0, key, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeMap::end(self.0)
	}
}

pub struct Deserializer<T: io::Read>(T);

impl<T: io::Read> Deserializer<T> {
	pub fn new(reader: T) -> Self {
		Self(reader)
	}
	
	fn read_len(&mut self) -> io::Result<u64> {
		read_len(&mut self.0)
	}
	
	fn deserialize_fixed<I, const N: usize>(&mut self, f: fn([u8; N]) -> I) -> io::Result<I> {
		let len = self.read_len()? as usize;
		
		if len > N {
			return Err(io::Error::new(io::ErrorKind::InvalidData, "integer too long"));
		}
		
		let mut buf = [0u8; N];
		self.0.read_exact(&mut buf[..len])?;
		Ok(f(buf))
	}
}

impl<'a, 'de, T: io::Read> serde::Deserializer<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_byte_buf(visitor)
	}
	
	fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_bool(self.deserialize_fixed(u64::from_be_bytes)? != 0)
	}
	
	fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i8(self.deserialize_fixed(i8::from_be_bytes)?)
	}
	
	fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i16(self.deserialize_fixed(i16::from_be_bytes)?)
	}
	
	fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i32(self.deserialize_fixed(i32::from_be_bytes)?)
	}
	
	fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i64(self.deserialize_fixed(i64::from_be_bytes)?)
	}
	
	fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u8(self.deserialize_fixed(u8::from_be_bytes)?)
	}
	
	fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u16(self.deserialize_fixed(u16::from_be_bytes)?)
	}
	
	fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u32(self.deserialize_fixed(u32::from_be_bytes)?)
	}
	
	fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u64(self.deserialize_fixed(u64::from_be_bytes)?)
	}
	
	fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_f32(self.deserialize_fixed(f32::from_be_bytes)?)
	}
	
	fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_f64(self.deserialize_fixed(f64::from_be_bytes)?)
	}
	
	fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let mut buf = vec![0u8; self.read_len()? as usize];
		self.0.read_exact(&mut buf)?;
		
		match String::from_utf8(buf)? {
			s if s.chars().count() == 0 => visitor.visit_char(s.chars().next().unwrap()),
			s => visitor.visit_str(&s)
		}
	}
	
	fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_string(visitor)
	}
	
	fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let mut buf = vec![0u8; self.read_len()? as usize];
		self.0.read_exact(&mut buf)?;
		visitor.visit_string(String::from_utf8(buf)?)
	}
	
	fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_byte_buf(visitor)
	}
	
	fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let mut buf = vec![0u8; self.read_len()? as usize];
		self.0.read_exact(&mut buf)?;
		visitor.visit_byte_buf(buf)
	}
	
	fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_some(self)
	}
	
	fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.read_len()? {
			0 => visitor.visit_unit(),
			len => {
				let mut buf = vec![0u8; len as usize];
				self.0.read_exact(&mut buf)?;
				visitor.visit_byte_buf(buf)
			}
		}
	}
	
	fn deserialize_unit_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_unit(visitor)
	}
	
	fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_newtype_struct(self)
	}
	
	fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let len = self.read_len()?;
		visitor.visit_seq(Deserializer((&mut self.0).take(len)))
	}
	
	fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_seq(visitor)
	}
	
	fn deserialize_tuple_struct<V: Visitor<'de>>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_seq(visitor)
	}
	
	fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let len = self.read_len()?;
		visitor.visit_map(Deserializer((&mut self.0).take(len)))
	}
	
	fn deserialize_struct<V: Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_map(visitor)
	}
	
	fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		let len = self.read_len()?;
		visitor.visit_enum(Deserializer((&mut self.0).take(len)))
	}
	
	fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u32(read_id(&mut self.0)?)
	}
	
	fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_any(visitor)
	}
}

impl<'de, T: io::Read> MapAccess<'de> for Deserializer<io::Take<T>> {
	type Error = DeError;
	
	fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		(self.0.limit() > 0)
			.then(|| seed.deserialize(self))
			.transpose()
	}
	
	fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
		seed.deserialize(self)
	}
}

impl<'de, T: io::Read> SeqAccess<'de> for Deserializer<io::Take<T>> {
	type Error = DeError;
	
	fn next_element_seed<E: DeserializeSeed<'de>>(&mut self, seed: E) -> Result<Option<E::Value>, Self::Error> {
		match self.0.limit() {
			0 => Ok(None),
			_ => seed.deserialize(SeqDeserializer(&mut self.0)).map(Some)
		}
	}
}

impl<'de, T: io::Read> EnumAccess<'de> for Deserializer<io::Take<T>> {
	type Error   = DeError;
	type Variant = Self;
	
	fn variant_seed<V: DeserializeSeed<'de>>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
		Ok((seed.deserialize(&mut self)?, self))
	}
}

impl<'de, T: io::Read> VariantAccess<'de> for Deserializer<T> {
	type Error = DeError;
	
	fn unit_variant(mut self) -> Result<(), Self::Error> {
		match self.read_len()? {
			0 => Ok(()),
			l => Err(DeError::InvalidLength(l as _, "0".to_string()))
		}
	}
	
	fn newtype_variant_seed<S: DeserializeSeed<'de>>(mut self, seed: S) -> Result<S::Value, Self::Error> {
		seed.deserialize(&mut self)
	}
	
	fn tuple_variant<V: Visitor<'de>>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		serde::Deserializer::deserialize_tuple(&mut self, len, visitor)
	}
	
	fn struct_variant<V: Visitor<'de>>(mut self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		serde::Deserializer::deserialize_map(&mut self, visitor)
	}
}

struct SeqDeserializer<'a, T: io::Read>(&'a mut io::Take<T>);

impl<'a, 'de, T: io::Read> serde::Deserializer<'de> for SeqDeserializer<'a, T> {
	type Error = DeError;
	
	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_enum(self)
	}
	
	fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		serde::Deserializer::deserialize_identifier(&mut Deserializer(self.0), visitor)
	}
	
	forward_to_deserialize_any!(
		bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char enum str string bytes byte_buf option unit
		unit_struct newtype_struct seq tuple tuple_struct map struct ignored_any
	);
}

impl<'a, 'de, T: io::Read> EnumAccess<'de> for SeqDeserializer<'a, T> {
	type Error   = DeError;
	type Variant = Deserializer<&'a mut io::Take<T>>;
	
	fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
		let key = seed.deserialize(SeqDeserializer(&mut*self.0))?;
		Ok((key, Deserializer(self.0)))
	}
}

pub fn read_id(mut reader: impl io::Read) -> io::Result<ElementId> {
	let mut buf = [0u8; 4];
	reader.read_exact(&mut buf[3..])?;
	
	let len = match buf[3] {
		v if v & 0b1000_0000 != 0 => 1,
		v if v & 0b0100_0000 != 0 => 2,
		v if v & 0b0010_0000 != 0 => 3,
		v if v & 0b0001_0000 != 0 => 4,
		_  => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid id"))
	};
	
	buf[4 - len] = buf[3] & (!0 >> len);
	reader.read_exact(&mut buf[4 - len + 1..])?;
	Ok(u32::from_be_bytes(buf))
}

pub fn read_len(mut reader: impl io::Read) -> io::Result<u64> {
	let mut buf = [0u8; 8];
	reader.read_exact(&mut buf[7..])?;
	
	let len = match buf[7] {
		v if v & 0b1000_0000 != 0 => 1,
		v if v & 0b0100_0000 != 0 => 2,
		v if v & 0b0010_0000 != 0 => 3,
		v if v & 0b0001_0000 != 0 => 4,
		v if v & 0b0000_1000 != 0 => 5,
		v if v & 0b0000_0100 != 0 => 6,
		v if v & 0b0000_0010 != 0 => 7,
		v if v & 0b0000_0001 != 0 => 8,
		_  => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid data size"))
	};
	
	buf[8 - len] = buf[7] & (!0 >> len);
	reader.read_exact(&mut buf[8 - len + 1..])?;
	Ok(u64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]))
}

pub fn read_element(mut reader: impl io::Read) -> io::Result<(ElementId, u64)> {
	let mut buf = [0u8; 8];
	reader.read_exact(&mut buf[3..4])?;
	
	let len = match buf[3] {
		v if v & 0b1000_0000 != 0 => 1,
		v if v & 0b0100_0000 != 0 => 2,
		v if v & 0b0010_0000 != 0 => 3,
		v if v & 0b0001_0000 != 0 => 4,
		_  => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid id"))
	};
	
	buf[4 - len] = buf[3] & (!0 >> len);
	reader.read_exact(&mut buf[4 - len + 1..4])?;
	let id = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
	reader.read_exact(&mut buf[7..])?;
	
	let len = match buf[7] {
		v if v & 0b1000_0000 != 0 => 1,
		v if v & 0b0100_0000 != 0 => 2,
		v if v & 0b0010_0000 != 0 => 3,
		v if v & 0b0001_0000 != 0 => 4,
		v if v & 0b0000_1000 != 0 => 5,
		v if v & 0b0000_0100 != 0 => 6,
		v if v & 0b0000_0010 != 0 => 7,
		v if v & 0b0000_0001 != 0 => 8,
		_  => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid data size"))
	};
	
	buf[8 - len] = buf[7] & (!0 >> len);
	reader.read_exact(&mut buf[8 - len + 1..])?;
	let len = u64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]);
	Ok((id, len as _))
}

pub fn write_id(mut writer: impl io::Write, id: ElementId) -> io::Result<()> {
	let b = id.to_be_bytes();
	match id {
		v if v & 0xFFFF_FF80 == 0 => writer.write_all(&[0x80 | b[3]]),
		v if v & 0xFFFF_C000 == 0 => writer.write_all(&[0x40 | b[2], b[3]]),
		v if v & 0xFFE0_0000 == 0 => writer.write_all(&[0x20 | b[1], b[2], b[3]]),
		v if v & 0xF000_0000 == 0 => writer.write_all(&[0x10 | b[0], b[1], b[2], b[3]]),
		_                         => unreachable!()
	}?;
	Ok(())
}

pub fn write_len(mut writer: impl io::Write, len: u64) -> io::Result<()> {
	let mut b = len.to_be_bytes();
	let off = 8 - match len {
		v if v & 0xFFFF_FFFF_FFFF_FF80 == 0 => 1,
		v if v & 0xFFFF_FFFF_FFFF_C000 == 0 => 2,
		v if v & 0xFFFF_FFFF_FFE0_0000 == 0 => 3,
		v if v & 0xFFFF_FFFF_F000_0000 == 0 => 4,
		v if v & 0xFFFF_FF80_0000_0000 == 0 => 5,
		v if v & 0xFFFF_C000_0000_0000 == 0 => 6,
		v if v & 0xFFE0_0000_0000_0000 == 0 => 7,
		v if v & 0xF000_0000_0000_0000 == 0 => 8,
		_ => unreachable!()
	};
	
	b[off] |= 1 << off;
	writer.write_all(&b[off..])
}

pub fn write_element(mut writer: impl io::Write, id: ElementId, data: &[u8]) -> io::Result<()> {
	let mut buf = [0u8; 12];
	let mut len = 0;
	
	let l = match id {
		v if v & 0xFFFF_FF80 == 0 => 1,
		v if v & 0xFFFF_C000 == 0 => 2,
		v if v & 0xFFE0_0000 == 0 => 3,
		v if v & 0xF000_0000 == 0 => 4,
		_ => unreachable!()
	};
	
	buf[..l].copy_from_slice(&u32::to_be_bytes(id)[4 - l..]);
	buf[0] |= 1 >> (l - 1);
	len += l;
	
	let l = match data.len() as u64 {
		v if v & 0xFFFF_FFFF_FFFF_FF80 == 0 => 1,
		v if v & 0xFFFF_FFFF_FFFF_C000 == 0 => 2,
		v if v & 0xFFFF_FFFF_FFE0_0000 == 0 => 3,
		v if v & 0xFFFF_FFFF_F000_0000 == 0 => 4,
		v if v & 0xFFFF_FF80_0000_0000 == 0 => 5,
		v if v & 0xFFFF_C000_0000_0000 == 0 => 6,
		v if v & 0xFFE0_0000_0000_0000 == 0 => 7,
		v if v & 0xF000_0000_0000_0000 == 0 => 8,
		_ => unreachable!()
	};
	
	buf[len..len + l].copy_from_slice(&u64::to_be_bytes(data.len() as _)[8 - l..]);
	buf[len] |= 1 >> (l - 1);
	len += l;
	writer.write_all(&buf[..len])?;
	writer.write_all(data)?;
	Ok(())
}