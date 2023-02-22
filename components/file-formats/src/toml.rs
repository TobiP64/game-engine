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

#![allow(unused_variables, dead_code, unused_mut)]

use {
	crate::utils::*,
	std::{io, fmt, collections::HashMap},
	serde::{*, de::*, ser::*, forward_to_deserialize_any}
};

pub fn serialize<W: io::Write, T: Serialize>(writer: W, value: &T) -> Result<(), SerError> {
	value.serialize(DummySerializer)?.serialize(writer)
}

pub fn deserialize<'de, T: Deserialize<'de>, R: io::BufRead + fmt::Debug>(reader: R) -> Result<T, DeError> {
	T::deserialize(Value::deserialize(reader)?)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
	String(String),
	Integer(i64),
	Float(f64),
	Bool(bool),
	DateTime(u64, u32),
	Array(Vec<Self>),
	Table(HashMap<String, Self>)
}

impl Value {
	pub fn as_string(&self) -> Option<&str> {
		match self {
			Self::String(v) => Some(v),
			_ => None
		}
	}
	
	pub fn as_integer(&self) -> Option<i64> {
		match self {
			Self::Integer(v) => Some(*v),
			_ => None
		}
	}
	
	pub fn as_float(&self) -> Option<f64> {
		match self {
			Self::Float(v) => Some(*v),
			_ => None
		}
	}
	
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Self::Bool(v) => Some(*v),
			_ => None
		}
	}
	
	pub fn as_datetime(&self) -> Option<(u64, u32)> {
		match self {
			Self::DateTime(secs, nanos) => Some((*secs, *nanos)),
			_ => None
		}
	}
	
	pub fn as_array(&self) -> Option<&[Self]> {
		match self {
			Self::Array(v) => Some(v),
			_ => None
		}
	}
	
	pub fn as_table(&self) -> Option<&HashMap<String, Self>> {
		match self {
			Self::Table(v) => Some(v),
			_ => None
		}
	}
	
	pub fn serializer<T: Serialize + ?Sized>(v: &T) -> Result<Self, SerError> {
		v.serialize(DummySerializer)
	}
	
	pub fn deserializer<'de, T: Deserialize<'de>>(self) -> Result<T, DeError> {
		T::deserialize(self)
	}
	
	pub fn serialize(&self, writer: impl io::Write) -> Result<(), SerError> {
		/*match self {
			Self::Table(v) => {
			
			}
			Self::Array(v) if v.iter().all(|v| matches!(v, Self::Table(_))) => {
				// array of tables
			}
			Self::Array(v) => {
				// normal array
			}
		}*/
		
		unimplemented!();
	}
	
	pub fn deserialize(mut reader: impl io::BufRead) -> Result<Self, DeError> {
		let mut map = std::collections::HashMap::new();
		
		loop {
			match reader.fill_buf()? {
				[ch,   ..] if ch.is_ascii_whitespace() => reader.consume(1),
				[b'[', ..] => {
					reader.consume(1);
					
					let array = matches!(reader.fill_buf()?, [b'[', ..]);
					
					if array {
						reader.consume(1);
					}
					
					unimplemented!()
				}
				[ch, ..] => {
				
				}
				[] => break //return Err(DeError::invalid_token("EOF", "?"))
			}
		}
		
		Ok(Self::Table(map))
	}
}

impl From<String> for Value {
	fn from(v: String) -> Self {
		Self::String(v)
	}
}

impl From<i64> for Value {
	fn from(v: i64) -> Self {
		Self::Integer(v)
	}
}

impl From<f64> for Value {
	fn from(v: f64) -> Self {
		Self::Float(v)
	}
}

impl From<bool> for Value {
	fn from(v: bool) -> Self {
		Self::Bool(v)
	}
}

impl From<std::time::SystemTime> for Value {
	fn from(v: std::time::SystemTime) -> Self {
		let d = v.duration_since(std::time::UNIX_EPOCH).unwrap();
		Self::DateTime(d.as_secs(), (d.as_nanos() % 1_000_000) as _)
	}
}

impl From<Vec<Self>> for Value {
	fn from(v: Vec<Self>) -> Self {
		Self::Array(v)
	}
}

impl From<HashMap<String, Self>> for Value {
	fn from(v: HashMap<String, Self>) -> Self {
		Self::Table(v)
	}
}

impl std::iter::FromIterator<Self> for Value {
	fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
		Self::Array(iter.into_iter().collect())
	}
}

impl std::iter::FromIterator<(String, Self)> for Value {
	fn from_iter<T: IntoIterator<Item = (String, Self)>>(iter: T) -> Self {
		Self::Table(iter.into_iter().collect())
	}
}

struct DummySerializer;

impl Serializer for DummySerializer {
	type Ok                     = Value;
	type Error                  = SerError;
	type SerializeSeq           = ValueWrapper<Vec<Value>>;
	type SerializeTuple         = ValueWrapper<Vec<Value>>;
	type SerializeTupleStruct   = ValueWrapper<Vec<Value>>;
	type SerializeTupleVariant  = ValueWrapper<(&'static str, Vec<Value>)>;
	type SerializeMap           = ValueWrapper<(HashMap<String, Value>, Option<String>)>;
	type SerializeStruct        = ValueWrapper<(HashMap<String, Value>, Option<String>)>;
	type SerializeStructVariant = ValueWrapper<(&'static str, HashMap<String, Value>)>;
	
	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Bool(v))
	}
	
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Integer(v as _))
	}
	
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		Ok(Value::String(v.to_string()))
	}
	
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		Ok(Value::String(v.to_string()))
	}
	
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		let mut s = self.serialize_seq(Some(v.len()))?;
		v.iter().try_for_each(|b| SerializeSeq::serialize_element(&mut s, b))?;
		SerializeSeq::end(s)
	}
	
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}
	
	fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Table(HashMap::new()))
	}
	
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}
	
	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
		Ok(Value::String(variant.to_string()))
	}
	
	fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Table(std::iter::once((variant.to_string(), value.serialize(self)?)).collect()))
	}
	
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Ok(ValueWrapper(Vec::new()))
	}
	
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Ok(ValueWrapper(Vec::new()))
	}
	
	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Ok(ValueWrapper(Vec::new()))
	}
	
	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Ok(ValueWrapper((variant, Vec::new())))
	}
	
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Ok(ValueWrapper((HashMap::new(), None)))
	}
	
	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		Ok(ValueWrapper((HashMap::new(), None)))
	}
	
	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		Ok(ValueWrapper((variant, HashMap::new())))
	}
}

impl SerializeSeq for ValueWrapper<Vec<Value>> {
	type Ok    = Value;
	type Error = SerError;
	
	fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		self.0.push(value.serialize(DummySerializer)?);
		Ok(())
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Array(self.0))
	}
}

impl SerializeTuple for ValueWrapper<Vec<Value>> {
	type Ok    = Value;
	type Error = SerError;
	
	fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(self, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self)
	}
}

impl SerializeTupleStruct for ValueWrapper<Vec<Value>> {
	type Ok    = Value;
	type Error = SerError;
	
	fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		SerializeSeq::serialize_element(self, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self)
	}
}

impl SerializeTupleVariant for ValueWrapper<(&'static str, Vec<Value>)> {
	type Ok    = Value;
	type Error = SerError;
	
	fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		self.0.1.push(value.serialize(DummySerializer)?);
		Ok(())
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Table(std::iter::once((self.0.0.to_string(), Value::Array(self.0.1))).collect()))
	}
}

impl SerializeMap for ValueWrapper<(HashMap<String, Value>, Option<String>)> {
	type Ok    = Value;
	type Error = SerError;
	
	fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
		unimplemented!()
	}
	
	fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		self.0.0.insert(self.0.1.take().unwrap(), value.serialize(DummySerializer)?);
		Ok(())
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Table(self.0.0))
	}
}

impl SerializeStruct for ValueWrapper<(HashMap<String, Value>, Option<String>)> {
	type Ok    = Value;
	type Error = SerError;
	
	fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		SerializeMap::serialize_entry(self, key, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeMap::end(self)
	}
}


impl SerializeStructVariant for ValueWrapper<(&'static str, HashMap<String, Value>)> {
	type Ok    = Value;
	type Error = SerError;
	
	fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		self.0.1.insert(key.to_string(), value.serialize(DummySerializer)?);
		Ok(())
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Value::Table(std::iter::once((self.0.0.to_string(), Value::Table(self.0.1))).collect()))
	}
}

impl<'de> serde::Deserializer<'de> for Value {
	type Error = DeError;
	
	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self {
			Self::String(v)             => visitor.visit_string(v),
			Self::Integer(v)            => visitor.visit_i64(v),
			Self::Float(v)              => visitor.visit_f64(v),
			Self::Bool(v)               => visitor.visit_bool(v),
			Self::DateTime(_secs, _nanos) => unimplemented!(),
			Self::Array(v)              => visitor.visit_seq(ValueWrapper(v.into_iter())),
			Self::Table(v)              => visitor.visit_map(ValueWrapper(v.into_iter().peekable()))
		}
	}
	
	forward_to_deserialize_any!(bool i64 f64 str string bytes byte_buf unit unit_struct
		newtype_struct seq tuple tuple_struct map struct ignored_any);
	
	fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Integer(v) = self { visitor.visit_i8(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Integer(v) = self { visitor.visit_i16(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Integer(v) = self { visitor.visit_i32(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Integer(v) = self { visitor.visit_u8(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Integer(v) = self { visitor.visit_u16(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Integer(v) = self { visitor.visit_u32(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Integer(v) = self { visitor.visit_u64(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if let Self::Float(v) = self { visitor.visit_f32(v as _) } else { self.deserialize_any(visitor) }
	}
	
	fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self {
			Self::String(s) if s.len() == 1 => visitor.visit_char(s.chars().next().unwrap()),
			_ => self.deserialize_any(visitor)
		}
	}
	
	fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_some(self)
	}
	
	fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		match self {
			Self::String(v)                => visitor.visit_enum(v.into_deserializer()),
			Self::Integer(v) if v >= 0     => visitor.visit_enum(variants[v as usize].into_deserializer()),
			Self::Table(v) if v.len() == 1 => visitor.visit_enum(Self::Table(v)),
			_                              => self.deserialize_any(visitor)
		}
	}
	
	fn deserialize_identifier<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
		unreachable!()
	}
}

impl<'de> EnumAccess<'de> for Value {
	type Error   = DeError;
	type Variant = Value;
	
	fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
		match self {
			Self::Table(v) => {
				let (key, val) = v.into_iter().next().unwrap();
				let key = seed.deserialize(IntoDeserializer::<DeError>::into_deserializer(key))?;
				Ok((key, val))
			}
			_ => unreachable!()
		}
	}
}

impl<'de> VariantAccess<'de> for Value {
	type Error = DeError;
	
	fn unit_variant(self) -> Result<(), Self::Error> {
		Err(DeError::Custom("expected a string".to_string()))
	}
	
	fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
		seed.deserialize(self)
	}
	
	fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		serde::Deserializer::deserialize_seq(self, visitor)
	}
	
	fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		serde::Deserializer::deserialize_map(self, visitor)
	}
}

struct ValueWrapper<T>(T);

impl<'de> MapAccess<'de> for ValueWrapper<std::iter::Peekable<<std::collections::HashMap<String, Value> as IntoIterator>::IntoIter>> {
	type Error = DeError;
	
	fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		self.0.peek()
			.map(|(v, _)| seed.deserialize(v.as_str().into_deserializer()))
			.transpose()
	}
	
	fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
		seed.deserialize(self.0.next().unwrap().1)
	}
}

impl<'de> SeqAccess<'de> for ValueWrapper<<Vec<Value> as IntoIterator>::IntoIter> {
	type Error = DeError;
	
	fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
		self.0.next()
			.map(|v| seed.deserialize(v))
			.transpose()
	}
}

#[derive(Debug)]
pub struct Deserializer<T: io::BufRead> {
	reader: T
}

impl<T: io::BufRead> Deserializer<T> {
	pub fn new(mut reader: T) -> Self {
		// consume magic bytes for unicode text files
		if reader.fill_buf().unwrap_or(&[]).starts_with(&UNICODE_FILE_MAGIC_BYTES) {
			reader.consume(3);
		}
		
		Self { reader }
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

/*pub struct Serializer<T: io::Write> {
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
		match v {
			true => self.writer.write_all(b"true"),
			false => self.writer.write_all(b"false"),
		}.map_err(Into::into)
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
		write!(&mut self.writer, "\"{}\"", v).map_err(Into::into)
	}
	
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		let mut s = self.serialize_seq(Some(v.len()))?;
		v.iter().try_for_each(|b| SerializeSeq::serialize_element(&mut s, &b))?;
		SerializeSeq::end(s)
	}
	
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}
	
	fn serialize_some<S: ?Sized + Serialize>(self, value: &S) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"{}").map_err(Into::into)
	}
	
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}
	
	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
		variant.serialize(self)
	}
	
	fn serialize_newtype_struct<S: ?Sized + Serialize>(self, _name: &'static str, value: &S) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_newtype_variant<S: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, value: &S) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}
	
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		self.first = true;
		self.writer.write_all(b"[ ")?;
		Ok(self)
	}
	
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		self.serialize_seq(Some(len))
	}
	
	fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		self.serialize_seq(Some(len))
	}
	
	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		self.writer.write_all(b"{ ")?;
		self.writer.write_all(variant.as_bytes())?;
		self.writer.write_all(b" = [ ")?;
		Ok(self)
	}
	
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		self.first = true;
		self.writer.write_all(b"{ ")?;
		Ok(self)
	}
	
	fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		self.serialize_map(Some(len))
	}
	
	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		self.serialize_map(Some(len))
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
			self.first = false;
			self.writer.write_all(b", ")?;
		}
		
		value.serialize(&mut**self)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b" ]")?;
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
		SerializeSeq::end(self)
	}
}

impl<'a, T: io::Write> SerializeMap for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_key<S: ?Sized + Serialize>(&mut self, key: &S) -> Result<(), Self::Error> {
		self.writer.write_all(b"\n")?;
		key.serialize(&mut**self)
	}
	
	fn serialize_value<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<(), Self::Error> {
		self.writer.write_all(b" = ")?;
		value.serialize(&mut**self)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"\n")?;
		Ok(())
	}
}

impl<'a, T: io::Write> SerializeStruct for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, key: &'static str, value: &S) -> Result<(), Self::Error> {
		self.writer.write_all(b"\n")?;
		key.serialize(&mut**self)?;
		self.writer.write_all(b" = ")?;
		value.serialize(&mut**self)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"\n")?;
		Ok(())
	}
}

impl<'a, T: io::Write> SerializeStructVariant for &'a mut Serializer<T> {
	type Ok    = ();
	type Error = SerError;
	
	fn serialize_field<S: ?Sized + Serialize>(&mut self, key: &'static str, value: &S) -> Result<(), Self::Error> {
		SerializeStruct::serialize_field(self, key, value)
	}
	
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeStruct::end(self)
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
	Uninit,
	Default
}

#[derive(Debug)]
pub struct Deserializer<T: io::BufRead> {
	reader: T,
	first:  bool,
	depth:  usize,
	is_map: bool,
	state:  State
}

impl<T: io::BufRead> Deserializer<T> {
	pub fn from_reader(mut reader: T) -> Self {
		// consume magic bytes for unicode text files
		if reader.fill_buf().unwrap_or(&[]).starts_with(&crate::utils::UNICODE_FILE_MAGIC_BYTES) {
			reader.consume(3);
		}
		
		Self { reader, is_map: true, first: true, state: State::Uninit, depth: 0 }
	}
	
	fn next(&mut self, exp: &str) -> io::Result<u8> {
		match self.reader.fill_buf()? {
			[ch, ..] => {
				let ch = *ch;
				self.reader.consume(1);
				Ok(ch)
			},
			[] => Err(io::Error::new(io::ErrorKind::UnexpectedEof, exp.to_string()))
		}
	}
}

impl<'a, 'de, T: io::BufRead> serde::Deserializer<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	#[allow(clippy::unused_io_amount)]
	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		if self.state == State::Uninit {
			self.state = State::Default;
			return visitor.visit_map(self);
		}
		
		match self.next("expected a value")? {
			mut ch if ch.is_ascii_digit() || [b'+', b'-', b'i', b'n'].contains(&ch) => {
				let mut sign = true;
				
				if ch == b'-' || ch == b'+' {
					if ch == b'-' { sign = false; }
					ch = self.reader.fill_buf()?[0];
					self.reader.consume(1);
				}
				
				match ch {
					b'i' => {
						let mut buf = [0u8; 2];
						self.reader.read(&mut buf)?;
						return match &buf {
							b"nf" => visitor.visit_f64(if sign { f64::INFINITY } else { f64::NEG_INFINITY }),
							_ => Err(DeError::invalid_token("", "`inf`"))
						}
					}
					b'n' => {
						let mut buf = [0u8; 2];
						self.reader.read(&mut buf)?;
						return match &buf {
							b"an" => visitor.visit_f64(if sign { f64::NAN } else { -f64::NAN }),
							_ => Err(DeError::invalid_token("", "`nan`"))
						}
					}
					_ch => {
						/*let radix = if ch == b'0' {
							match self.next("a number")? {
								'b' => 2,
								'o' => 8,
								'x' => 16,
								c   => {
									ch = c;
									10
								}
							}
						} else {
							10
						};
						
						let mut val = 0u64;
						
						'int: {
						
						}
						
						unimplemented!()
						
						
						
						loop {
							match ch {
								ch if ch.is_ascii_whitespace() => break,
								'_' => (),
								ch @ 'e' | ch @ 'E' | ch @ '.' => {
									float = true;
									self.buf.push(ch);
								}
								ch => self.buf.push(ch)
							}
							
							match self.next()? {
								ch if ch.is_ascii_whitespace() => break,
								'_' => (),
								ch @ 'e' | ch @ 'E' | ch @ '.' => {
									float = true;
									self.buf.push(ch);
								}
								ch => self.buf.push(ch)
							}
						}
						
						match (sign, float) {
							(false, false) => visitor.visit_u64(unimplemented!()),
							(true,  false) => visitor.visit_i64(unimplemented!()),
							(_,     true)  => visitor.visit_f64(unimplemented!()),
						}*/
						
						unimplemented!("number parsing")
					}
				}
			}
			b't' => {
				let mut buf = [0u8; 3];
				self.reader.read(&mut buf)?;
				match &buf {
					b"rue" => visitor.visit_bool(true),
					_ => Err(DeError::invalid_token("", "`true`"))
				}
			}
			b'f' => {
				let mut buf = [0u8; 4];
				self.reader.read(&mut buf)?;
				match &buf {
					b"alse" => visitor.visit_bool(false),
					_ => Err(DeError::invalid_token("", "`false`"))
				}
			}
			b'\'' => visitor.visit_string(String::from_utf8(std::iter::repeat_with(|| self.next("'''"))
				.take_while(|ch| !matches!(ch, Ok(b'\'')))
				.collect::<Result<Vec<_>, _>>()?)
				.unwrap()),
			b'"' => visitor.visit_string(String::from_utf8(std::iter::repeat_with(|| self.next("'\"'"))
				.take_while(|ch| !matches!(ch, Ok(b'"')))
				.collect::<Result<Vec<_>, _>>()?)
				.unwrap()),
			b'[' => visitor.visit_seq(self),
			b'{' => visitor.visit_map(self),
			ch   => Err(DeError::invalid_token(&format!("'{}'", ch), "a value"))
		}
	}
	
	forward_to_deserialize_any! {
		bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf option unit
		unit_struct newtype_struct seq tuple tuple_struct map struct enum ignored_any
	}
	
	fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let mut buf = Vec::new();
		
		let end = match self.reader.fill_buf()?.first().copied() {
			Some(end @ b'\'' | end @ b'"') => Some(end),
			Some(ch) if ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'-' => {
				buf.push(ch);
				self.reader.consume(1);
				None
			}
			Some(ch) => return Err(DeError::invalid_char(ch as _, "a key (A-Za-z0-9_-))")),
			None => return Err(DeError::unexpected_eof("expected a key"))
		};
		
		loop {
			match (self.reader.fill_buf()?, end) {
				([ch, ..], _) if ch.is_ascii_alphanumeric() || *ch == b'_' || *ch == b'-' => buf.push(*ch),
				([ch, ..], Some(end)) if *ch == end => {
					self.reader.consume(1);
					break;
				}
				([ch, ..], None) if ch.is_ascii_whitespace() || *ch == b'.' || *ch == b'=' => break,
				([ch, ..], _) => return Err(DeError::invalid_char(*ch as _, "a key (A-Za-z0-9_-))")),
				([],       _) => return Err(DeError::unexpected_eof("expected a key"))
			}
		}
		
		visitor.visit_string(String::from_utf8(buf).unwrap())
	}
	
	fn is_human_readable(&self) -> bool {
		true
	}
}

impl<'a, 'de, T: io::BufRead> MapAccess<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		// TODO comments
		
		let mut delimiter = self.first;
		
		if self.first {
			self.first = false;
		}
		
		loop {
			match (self.reader.fill_buf()?, delimiter) {
				([b'\n', ..], _) | ([b',', ..], false) => delimiter = true,
				([ch, ..], _) if ch.is_ascii_whitespace() => self.reader.consume(1),
				([b'}', ..], _) => {
					self.reader.consume(1);
					return Ok(None)
				}
				([b'[', ..], _) | ([], _) => return Ok(None),
				([_,  ..], true)  => break,
				([ch, ..], false) => return Err(DeError::invalid_char(*ch as _, "'\\n', ',', '}', '[' or whitespace"))
			}
		}
		
		seed.deserialize(&mut**self).map(Some)
	}
	
	fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
		let mut delimiter = false;
		
		loop {
			match (self.reader.fill_buf()?, delimiter) {
				([b'=', ..], false) => delimiter = true,
				([ch,   ..], _) if ch.is_ascii_whitespace() => self.reader.consume(1),
				([_,    ..], true)  => break,
				([ch,   ..], false) => return Err(DeError::invalid_char(*ch as _, "'=' or whitespace")),
				([],         true)  => return Err(DeError::unexpected_eof("a value")),
				([],         false) => return Err(DeError::unexpected_eof("'=' or whitespace"))
			}
		}
		
		seed.deserialize(&mut**self)
	}
}

impl<'a, 'de, T: io::BufRead> SeqAccess<'de> for &'a mut Deserializer<T> {
	type Error = DeError;
	
	fn next_element_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error> {
		let mut delimiter = self.first;
		
		if self.first {
			self.first = false;
		}
		
		loop {
			match (self.reader.fill_buf()?, delimiter) {
				([b']', ..], _) => {
					self.reader.consume(1);
					return Ok(None);
				}
				([b',', ..], false) => delimiter = true,
				([ch,   ..], _) if ch.is_ascii_whitespace() => self.reader.consume(1),
				([_,    ..], true) => break,
				([ch,   ..], false) => return Err(DeError::invalid_char(*ch as _, "',', ']' or whitespace")),
				([],         true)  => return Err(DeError::unexpected_eof("a value")),
				([],         false) => return Err(DeError::unexpected_eof("',', ']' or whitespace"))
			}
		}
		
		seed.deserialize(&mut**self).map(Some)
	}
}*/

#[cfg(test)]
mod tests {
	use serde::Deserialize;
	
	#[derive(Debug, Eq, PartialEq, Deserialize)]
	struct Struct {
		a: bool,
		b: String,
		c: Struct2
	}
	
	#[derive(Debug, Eq, PartialEq, Deserialize)]
	struct Struct2 {
		a: String
	}
	
	#[test]
	fn simple() {
		assert_eq!(super::deserialize::<Struct, _>(&mut r#"
		a = true
		b = "test"
		
		[c]
		a = "test"
		"#.as_bytes()).unwrap(), Struct { a: true, b: "test".to_string(), c: Struct2 { a: "test".to_string() } });
	}
}