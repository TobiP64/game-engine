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
	std::{io, fmt, marker::PhantomData},
	serde::{*, de::{self, *}, ser::*, forward_to_deserialize_any}
};

pub fn serialize<W: io::Write, T: Serialize>(writer: W, value: &T) -> Result<(), SerError> {
	value.serialize(&mut Serializer::new(writer))
}

pub fn deserialize<'de, T: Deserialize<'de>, R: io::BufRead + fmt::Debug>(reader: R) -> Result<T, DeError> {
	T::deserialize(&mut Deserializer::new(reader))
}

pub fn deserialize_slice<'de, T: Deserialize<'de>>(slice: &[u8]) -> Result<T, DeError> {
	deserialize(io::BufReader::new(slice))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
	String(String),
	Number(f64),
	Object(std::collections::HashMap<String, Self>),
	Array(Vec<Self>),
	Bool(bool),
	Null
}

impl Value {
	pub fn as_string(&self) -> Option<&String> {
		match self {
			Self::String(v) => Some(v),
			_ => None
		}
	}
	
	pub fn as_number(&self) -> Option<f64> {
		match self {
			Self::Number(v) => Some(*v),
			_ => None
		}
	}
	
	pub fn as_object(&self) -> Option<&std::collections::HashMap<String, Self>> {
		match self {
			Self::Object(v) => Some(v),
			_ => None
		}
	}
	
	pub fn as_array(&self) -> Option<&[Self]> {
		match self {
			Self::Array(v) => Some(v),
			_ => None
		}
	}
	
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Self::Bool(v) => Some(*v),
			_ => None
		}
	}
	
	pub fn is_null(&self) -> bool {
		matches!(self, Self::Null)
	}
}

impl Default for Value {
	fn default() -> Self {
		Self::Null
	}
}

impl From<String> for Value {
	fn from(v: String) -> Self {
		Self::String(v)
	}
}

impl From<f64> for Value {
	fn from(v: f64) -> Self {
		Self::Number(v)
	}
}

impl From<std::collections::HashMap<String, Self>> for Value {
	fn from(v: std::collections::HashMap<String, Self>) -> Self {
		Self::Object(v)
	}
}

impl From<Vec<Self>> for Value {
	fn from(v: Vec<Self>) -> Self {
		Self::Array(v)
	}
}

impl From<bool> for Value {
	fn from(v: bool) -> Self {
		Self::Bool(v)
	}
}

impl From<()> for Value {
	fn from(_: ()) -> Self {
		Self::Null
	}
}

pub struct Serializer<T: io::Write> {
	writer: T,
	first:  bool
}

impl<T: io::Write> Serializer<T> {
	pub fn new(writer: T) -> Self {
		Self { writer, first: false }
	}
}

impl<'a, T: io::Write> serde::Serializer for &'a mut Serializer<T> {
	type Ok                     = ();
	type Error                  = SerError;
	type SerializeSeq           = Self;
	type SerializeTuple         = Self;
	type SerializeTupleStruct   = Self;
	type SerializeTupleVariant  = Self;
	type SerializeMap           = Self;
	type SerializeStruct        = Self;
	type SerializeStructVariant = Self;
	
	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(match v {
			true  => b"true",
			false => b"false"
		}).map_err(Into::into)
	}
	
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "{}", v).map_err(Into::into)
	}
	
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "\"{}\"", v).map_err(Into::into)
	}
	
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		write!(&mut self.writer, "\"{}\"", v.replace("\"", "\\\"")).map_err(Into::into)
	}
	
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		let mut s = self.serialize_seq(Some(v.len()))?;
		v.iter().try_for_each(|b| SerializeSeq::serialize_element(&mut s, b))?;
		SerializeSeq::end(s)
	}
	
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"null").map_err(Into::into)
	}
	
	fn serialize_some<S: ?Sized + Serialize>(self, value: &S) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"null").map_err(Into::into)
	}
	
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}
	
	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(variant)
	}
	
	fn serialize_newtype_struct<S: ?Sized + Serialize>(self, _name: &'static str, value: &S) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_newtype_variant<S: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &S) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"{\"")?;
		self.writer.write_all(variant.as_bytes())?;
		self.writer.write_all(b"\":")?;
		value.serialize(&mut*self)?;
		self.writer.write_all(b"}")?;
		Ok(())
	}
	
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		self.first = true;
		self.writer.write_all(b"[")?;
		Ok(self)
	}
	
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		self.serialize_seq(Some(len))
	}
	
	fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		self.serialize_seq(Some(len))
	}
	
	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		self.writer.write_all(b"{\"")?;
		self.writer.write_all(variant.as_bytes())?;
		self.writer.write_all(b"\":[")?;
		Ok(self)
	}
	
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		self.first = true;
		self.writer.write_all(b"{")?;
		Ok(self)
	}
	
	fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		self.serialize_map(Some(len))
	}
	
	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		self.writer.write_all(b"{\"")?;
		self.writer.write_all(variant.as_bytes())?;
		self.writer.write_all(b"\":{")?;
		Ok(self)
	}
	
	fn is_human_readable(&self) -> bool {
		true
	}
}

impl<'a, T: io::Write> SerializeSeq for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_element<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		if !self.first {
			self.writer.write_all(b",")?;
		} else {
			self.first = false;
		}
		
		value.serialize(&mut**self)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"]")?;
		Ok(())
	}
}

impl<'a, T: io::Write> SerializeTuple for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_element<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(self, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self)
	}
}

impl<'a, T: io::Write> SerializeTupleStruct for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(self, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self)
	}
}

impl<'a, T: io::Write> SerializeTupleVariant for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(self, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"]}")?;
		Ok(())
	}
}

impl<'a, T: io::Write> SerializeMap for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_key<S: ?Sized + Serialize>(&mut self, key: &S) -> Result<(), Self::Error> {
		if !self.first {
			self.writer.write_all(b",")?;
		} else {
			self.first = false;
		}
		
		key.serialize(&mut**self)
	}
	
	fn serialize_value<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		self.writer.write_all(b":")?;
		value.serialize(&mut**self)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"}")?;
		Ok(())
	}
}

impl<'a, T: io::Write> SerializeStruct for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, key: &'static str, value: &S) -> Result<(), Self::Error> {
		SerializeMap::serialize_entry(self, key, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeMap::end(self)
	}
}

impl<'a, T: io::Write> SerializeStructVariant for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, key: &'static str, value: &S) -> Result<(), Self::Error> {
		SerializeMap::serialize_entry(self, key, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"}}")?;
		Ok(())
	}
}

#[derive(Copy, Clone, Debug)]
pub enum Expected {
	MapComma,
	MapColon,
	MapEnd,
	SeqComma,
	SeqEnd,
	Key,
	Value,
	EnumVariant
}

impl std::fmt::Display for Expected {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}

#[derive(Debug)]
pub struct Deserializer<T: io::BufRead> {
	reader: T,
	first:  bool
}

impl<T: io::BufRead> Deserializer<T> {
	pub fn new(mut reader: T) -> Self {
		// consume magic bytes for unicode text files
		if reader.fill_buf().unwrap_or(&[]).starts_with(&UNICODE_FILE_MAGIC_BYTES) {
			reader.consume(3);
		}
		
		Self { reader, first: true }
	}
	
	fn peek(&mut self) -> io::Result<u8> {
		match self.reader.fill_buf()? {
			[ch, ..] => Ok(*ch),
			_ => Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""))
		}
	}
	
	fn next(&mut self) -> io::Result<u8> {
		loop {
			match self.reader.fill_buf()? {
				[ch, ..] if ch.is_ascii_whitespace() => self.reader.consume(1),
				[ch, ..] => {
					let ch = *ch;
					self.reader.consume(1);
					return Ok(ch)
				}
				[] => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""))
			}
		}
	}
}

macro_rules! visit_number {
	( $( $de_fn:ident, $visit_fn:ident; )* ) => {
		$( visit_number!($de_fn, $visit_fn); )*
	};
    ($de_fn:ident, $visit_fn:ident) => {
		fn $de_fn<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
			V: Visitor<'de> {
			struct NumberVisitor<'de, V: Visitor<'de>>(V, PhantomData<&'de ()>);
			
			impl<'de, V: Visitor<'de>> Visitor<'de> for NumberVisitor<'de, V> {
				type Value = V::Value;
				
				fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
					self.0.expecting(formatter)
				}
				
				fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
					self.0.$visit_fn(v as _)
				}
			}
			
			self.deserialize_any(NumberVisitor(visitor, PhantomData))
		}
    };
}

impl<'a, 'de, T: io::BufRead> serde::Deserializer<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		return match self.next()? {
			b'"' => {
				let mut buf = Vec::new();
				self.reader.read_until(b'"', &mut buf)?;
				match buf.as_slice() {
					[.., b'"'] => {
						buf.truncate(buf.len() - 1);
						visitor.visit_string(String::from_utf8(buf).map_err(|e| e.utf8_error())?)
					}
					_ => Err(DeError::invalid_token("EOF", Expected::Value))
				}
			}
			b'{' => {
				self.first = true;
				visitor.visit_map(self)
			}
			b'[' => {
				self.first = true;
				visitor.visit_seq(self)
			}
			b't'  => {
				let mut buf = [0u8; 3];
				let _ = self.reader.read(&mut buf)?;
				match &buf {
					b"rue" => visitor.visit_bool(true),
					_ => Err(DeError::invalid_token("\u{FFFD}", Expected::Value))
				}
			}
			b'f'  => {
				let mut buf = [0u8; 4];
				let _ = self.reader.read(&mut buf)?;
				match &buf {
					b"alse" => visitor.visit_bool(true),
					_ => Err(DeError::invalid_token("\u{FFFD}", Expected::Value))
				}
			}
			b'n' => {
				let mut buf = [0u8; 3];
				let _ = self.reader.read(&mut buf)?;
				match &buf {
					b"ull" => visitor.visit_unit(),
					_ => Err(DeError::invalid_token("\u{FFFD}", Expected::Value))
				}
			}
			b'0'..=b'9' | b'-' => {
				unimplemented!("number parsing")
			}
			ch => Err(DeError::invalid_token(ch, Expected::Value))
		}
	}
	
	visit_number!(
		deserialize_i8, visit_i8;
		deserialize_i16, visit_i16;
		deserialize_i32, visit_i32;
		deserialize_i64, visit_i64;
		deserialize_u8, visit_u8;
		deserialize_u16, visit_u16;
		deserialize_u32, visit_u32;
		deserialize_u64, visit_u64;
		deserialize_f32, visit_f32;
	);
	
	fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		struct CharVisitor<'de, V: Visitor<'de>>(V, PhantomData<&'de ()>);
		
		impl<'de, V: Visitor<'de>> Visitor<'de> for CharVisitor<'de, V> {
			type Value = V::Value;
			
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				self.0.expecting(formatter)
			}
			
			fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
				E: de::Error, {
				match v.chars().count() {
					1 => self.0.visit_char(v.chars().next().unwrap()),
					_ => Err(E::invalid_type(Unexpected::Str(v.as_str()), &self))
				}
			}
			
			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
				E: de::Error, {
				match v.chars().count() {
					1 => self.0.visit_char(v.chars().next().unwrap()),
					_ => Err(E::invalid_type(Unexpected::Str(v), &self))
				}
			}
		}
		
		self.deserialize_any(CharVisitor(visitor, PhantomData))
	}
	
	fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_byte_buf(visitor)
	}
	
	fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.peek()? {
			b'[' => visitor.visit_byte_buf(Vec::<u8>::deserialize(self)?),
			_ => self.deserialize_any(visitor)
		}
	}
	
	fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.peek()? {
			b'n' => {
				self.reader.consume(1);
				let mut buf = [0u8; 3];
				let _ = self.reader.read(&mut buf)?;
				match &buf {
					b"ull" => visitor.visit_none(),
					_ => Err(DeError::invalid_token("", Expected::Value))
				}
			}
			_ => visitor.visit_some(self)
		}
	}
	
	fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V, ) -> Result<V::Value, Self::Error> {
		visitor.visit_newtype_struct(self)
	}
	
	fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		match self.peek()? {
			b'{' => {
				self.reader.consume(1);
				visitor.visit_enum(self)
			},
			b'"' => visitor.visit_enum(String::deserialize(self)?.into_deserializer()),
			ch => Err(DeError::invalid_token(ch, Expected::EnumVariant))
		}
	}
	
	fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if self.next()? != b'"' {
			return Err(DeError::invalid_token("", Expected::Key))
		}
		
		let mut buf = Vec::new();
		self.reader.read_until(b'"', &mut buf)?;
		let mut s = String::from_utf8(buf)?;
		
		if !s.ends_with('"') {
			Err(DeError::invalid_token("EOF", Expected::Key))
		} else {
			s.truncate(s.len() - 1);
			visitor.visit_string(s)
		}
	}
	
	fn is_human_readable(&self) -> bool {
		true
	}
	
	forward_to_deserialize_any!(f64 bool str string unit unit_struct seq tuple tuple_struct map struct ignored_any);
}

impl<'a, 'de, T: io::BufRead> MapAccess<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		let ch = self.next()?;
		
		if ch == b'}' {
			return Ok(None)
		} else if !self.first && ch != b',' {
			return Err(DeError::invalid_token(ch, Expected::MapComma))
		}
		
		self.first = false;
		seed.deserialize(&mut**self).map(Some)
	}
	
	fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
		match self.next()? {
			b':' => seed.deserialize(&mut**self),
			ch => Err(DeError::invalid_token(ch, Expected::MapColon))
		}
	}
}

impl<'a, 'de, T: io::BufRead> SeqAccess<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	fn next_element_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error> {
		let ch = self.next()?;
		
		if ch == b'}' {
			return Ok(None)
		} else if !self.first && ch != b',' {
			return Err(DeError::invalid_token(ch, Expected::SeqComma))
		}
		
		self.first = false;
		seed.deserialize(&mut**self).map(Some)
	}
}

impl<'a, 'de, T: io::BufRead> EnumAccess<'de> for &'a mut Deserializer<T> {
	type Error   = DeError;
	type Variant = Self;
	
	fn variant_seed<V: DeserializeSeed<'de>>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
		match MapAccess::next_key_seed(&mut self, seed)? {
			Some(key) => Ok((key, self)),
			None => Err(DeError::Custom("expected an enum variant".to_string()))
		}
	}
}

impl<'a, 'de, T: io::BufRead> VariantAccess<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	fn unit_variant(self) -> Result<(), Self::Error> {
		Err(DeError::Custom("expected a string".to_string()))
	}
	
	fn newtype_variant_seed<S: DeserializeSeed<'de>>(self, seed: S) -> Result<S::Value, Self::Error> {
		seed.deserialize(self)
	}
	
	fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		serde::Deserializer::deserialize_seq(self, visitor)
	}
	
	fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		serde::Deserializer::deserialize_map(self, visitor)
	}
}

#[cfg(test)]
mod tests {
	use serde::*;
	use super::*;
	
	#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
	struct Struct {
		a: bool,
		b: String,
		c: Struct2
	}
	
	#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
	struct Struct2 {
		a: String
	}
	
	#[test]
	fn full() {
		let v = Struct { a: true, b: "test".to_string(), c: Struct2 { a: "test".to_string() } };
		let mut buf = Vec::<u8>::new();
		serialize(&mut buf, &v).unwrap();
		assert_eq!(v, deserialize_slice(buf.as_slice()).unwrap());
	}
	
	#[test]
	fn simple() {
		let v = super::deserialize::<Struct, _>(&mut r#"
		{
			"a": true,
			"b": "test",
			"c": {
				"a": "test"
			}
		}
		"#.as_bytes()).unwrap();
		
		assert_eq!(v, Struct { a: true, b: "test".to_string(), c: Struct2 { a: "test".to_string() } });
	}
}