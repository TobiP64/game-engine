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

pub const UNICODE_FILE_MAGIC_BYTES: [u8; 3] = [0xEF, 0xBB, 0xBF];

#[cfg(feature = "serde")]
pub use self::serde::*;

#[cfg(feature = "serde")]
mod serde {
	use {std::{io, fmt}, serde::de::{self, Expected, Unexpected}};
	
	pub enum SerError {
		Custom(String),
		Io(io::Error),
		UnsupportedType
	}
	
	impl fmt::Debug for SerError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			fmt::Display::fmt(self, f)
		}
	}
	
	impl fmt::Display for SerError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match self {
				Self::Custom(s)       => f.write_str(s),
				Self::Io(err)         => write!(f, "io error: {}", err),
				Self::UnsupportedType => write!(f, "unsupported type")
			}
		}
	}
	
	impl std::error::Error for SerError {}
	
	impl serde::ser::Error for SerError {
		fn custom<T: fmt::Display>(msg: T) -> Self {
			Self::Custom(msg.to_string())
		}
	}
	
	impl From<io::Error> for SerError {
		fn from(e: io::Error) -> Self {
			Self::Io(e)
		}
	}
	
	struct DisplayImpl<'a>(&'a dyn Expected);
	
	impl fmt::Display for DisplayImpl<'_> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			self.0.fmt(f)
		}
	}
	
	pub enum DeError {
		// serde errors
		Custom(String),
		InvalidType(String, String),
		InvalidValue(String, String),
		InvalidLength(usize, String),
		UnknownVariant(String, &'static [&'static str]),
		UnknownField(String, &'static [&'static str]),
		MissingField(&'static str),
		DuplicateField(&'static str),
		// custom errors
		Io(io::Error),
		Utf8(std::str::Utf8Error),
		InvalidToken(String, String),
		ParseError(String)
	}
	
	impl DeError {
		pub fn invalid_token(unexp: impl ToString, exp: impl ToString) -> Self {
			Self::InvalidToken(unexp.to_string(), exp.to_string())
		}
	}
	
	impl fmt::Debug for DeError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			fmt::Display::fmt(self, f)
		}
	}
	
	impl fmt::Display for DeError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match self {
				Self::Custom(s)                  => f.write_str(s),
				Self::InvalidType(unexp, exp)    => write!(f, "invalid type `{}`, expected `{}`", unexp, exp),
				Self::InvalidValue(unexp, exp)   => write!(f, "invalid value `{}`, expected `{}`", unexp, exp),
				Self::InvalidLength(unexp, exp)  => write!(f, "invalid length `{}`, expected `{}`", *unexp, exp),
				Self::UnknownVariant(unexp, exp) => write!(f, "unknown variant `{}`, expected variants `{:?}`", unexp, exp),
				Self::UnknownField(unexp, exp)   => write!(f, "unknown field `{}`, expected fields `{:?}`", unexp, exp),
				Self::MissingField(exp)          => write!(f, "missing field `{}`", exp),
				Self::DuplicateField(field)      => write!(f, "duplicate field `{}`", field),
				Self::Io(err)                    => write!(f, "io error: {}", err),
				Self::Utf8(err)                  => write!(f, "utf8 error: {}", err),
				Self::InvalidToken(unexp, exp)   => write!(f, "invalid token: {}, expected {}", unexp, exp),
				Self::ParseError(err)            => write!(f, "parse error: {}", err)
			}
		}
	}
	
	impl std::error::Error for DeError {}
	
	impl de::Error for DeError {
		fn custom<U: fmt::Display>(msg: U) -> Self {
			Self::Custom(msg.to_string())
		}
		
		fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
			Self::InvalidType(unexp.to_string(), DisplayImpl(exp).to_string())
		}
		
		fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
			Self::InvalidValue(unexp.to_string(), DisplayImpl(exp).to_string())
		}
		
		fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
			Self::InvalidLength(len, DisplayImpl(exp).to_string())
		}
		
		fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
			Self::UnknownVariant(variant.to_string(), expected)
		}
		
		fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
			Self::UnknownField(field.to_string(), expected)
		}
		
		fn missing_field(field: &'static str) -> Self {
			Self::MissingField(field)
		}
		
		fn duplicate_field(field: &'static str) -> Self {
			Self::DuplicateField(field)
		}
	}
	
	impl From<io::Error> for DeError {
		fn from(e: io::Error) -> Self {
			Self::Io(e)
		}
	}
	
	impl From<std::str::Utf8Error> for DeError {
		fn from(e: std::str::Utf8Error) -> Self {
			Self::Utf8(e)
		}
	}
	
	impl From<std::string::FromUtf8Error> for DeError {
		fn from(e: std::string::FromUtf8Error) -> Self {
			Self::Utf8(e.utf8_error())
		}
	}
}