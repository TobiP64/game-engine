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

use xml::Body;
use serde::{*, de::Error};
use std::{io::{self, Write}, fs};

mod xml;

const HELP: &str = r#"
wlgen
Usage: wlgen <input file>... [options]

If no input file is specified, the protocols will be read from stdin.
If an input file is a directory, all files in that directory with a `.xml` extension will be parsed.

Options:
--help, -h           - display this help page
--out=<output file>,
-o=<output file>     - specify the output file, if no output file is specified, the generated code will be written to `wl.rs`

For more information, see https://crates.io/crates/wlgen
"#;

fn main() {
	let mut files = Vec::new();
	let mut file_out = None;
	
	for arg in std::env::args().skip(1) {
		if arg.starts_with('-') {
			if arg.starts_with("-o=") || arg.starts_with("--out=") {
				file_out = Some(arg.trim_start_matches("-o=")
					.trim_start_matches("--out=").to_string());
			} else if arg == "-h" || arg == "--help" {
				println!("{}", HELP);
				return;
			} else {
				println!("ignored unknown option: {}", arg)
			}
		} else if fs::metadata(&arg).unwrap().is_dir() {
			files.extend(fs::read_dir(arg).unwrap()
				.filter_map(Result::ok)
				.map(|f| f.path().as_path().to_str().unwrap().to_string())
				.filter(|f| f.ends_with(".xml")));
		} else {
			files.push(arg);
		}
	}
	
	let mut writer = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(file_out.as_ref().map_or("wl.rs", String::as_str))
		.map(io::BufWriter::new)
		.expect("failed to open output file");
	
	for file in files {
		print!("parsing file `{}` ... ", &file);
		std::io::stdout().flush().unwrap();
		let protocol: WlProtocol = xml::deserialize(fs::File::open(&file)
			.map(io::BufReader::new)
			.expect("failed to open file"))
			.expect("failed to parse protocol");
		print!("\x1b[32mok\x1b[0m\ngenerating code for `{}` ... ", &file);
		std::io::stdout().flush().unwrap();
		gen(&mut writer, protocol).expect("failed to output code");
		println!("\x1b[32mok\x1b[0m");
	}
}

#[allow(clippy::cognitive_complexity)]
fn gen(writer: &mut impl io::Write, protocol: WlProtocol) -> io::Result<()> {
	fn write_desc(writer: &mut impl io::Write, pre: &str, desc: Option<WlDescription>) -> io::Result<()> {
		if let Some(desc) = desc {
			writeln!(writer, "\n{}/// # {}\n{0}///", pre, desc.summary
				.lines().map(str::trim).collect::<String>())?;
			
			for line in desc.content.lines() {
				writeln!(writer, "{}/// {}", pre, line.trim())?;
			}
		}
		
		Ok(())
	}
	
	write_desc(writer, "", protocol.description)?;
	writeln!(writer, "pub use {}::*;\nmod {0} {{\n\tuse crate::*;\n", protocol.name)?;
	
	if let Some(copyright) = protocol.copyright {
		for line in copyright.lines() {
			writeln!(writer, "\t// {}", line.trim())?;
		}
	}
	
	for interface in protocol.interfaces {
		let name = convert_name(&interface.name);
		let uppercase = interface.name.to_uppercase();
		
		// interface
		
		writeln!(writer, r#"
	pub static {}_INTERFACE: WlInterface = WlInterface {{
		name:         "{}\0".as_ptr(),
		version:      {},
		method_count: {},
		methods:      ["#, &uppercase, &interface.name, interface.version, interface.requests.len())?;
		
		// interface requests
		
		for request in &interface.requests {
			let mut buf = String::new();
			
			for arg in &request.args {
				if let Some(interface) = &arg.interface {
					buf.push('&');
					buf.push_str(&interface.to_uppercase());
					buf.push_str("_INTERFACE as _, ");
				}
			}
			
			writeln!(writer, r#"			WlMessage {{
				name:      "{}\0".as_ptr(),
				signature: "{}\0".as_ptr(),
				types:     [{}].as_ptr()
			}},"#, &request.name, get_signature(&request.args), buf.trim_end_matches(", "))?;
		}
		
		writeln!(writer, r#"		].as_ptr(),
		event_count:  {},
		events:       ["#, interface.events.len())?;
		
		// interface events
		
		for event in &interface.events {
			let mut buf = String::new();
			
			for arg in &event.args {
				if let Some(interface) = &arg.interface {
					buf.push('&');
					buf.push_str(&interface.to_uppercase());
					buf.push_str("_INTERFACE as _, ");
				}
			}
			
			writeln!(writer, r#"			WlMessage {{
				name:      "{}\0".as_ptr(),
				signature: "{}\0".as_ptr(),
				types:     [{}].as_ptr()
			}},"#, &event.name, get_signature(&event.args), buf.trim_end_matches(", "))?;
		}
		
		writeln!(writer, r#"		].as_ptr()
	}};"#)?;
		
		// struct definition
		
		write_desc(writer, "\t", interface.description)?;
		write!(writer, r#"	pub struct {0}(WlProxy);
	
	impl std::ops::Deref for {0} {{
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {{
			&self.0
		}}
	}}
	
	impl std::ops::DerefMut for {0} {{
		fn deref_mut(&mut self) -> &mut Self::Target {{
			&mut self.0
		}}
	}}
	
	impl std::fmt::Debug for {0} {{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
			f.debug_struct("{0}")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}}
	}}
	
	impl {0} {{
		pub fn get_version(&self) -> u32 {{
			unsafe {{ WlProxy::get_version(&self.0) }}
		}}

		pub fn get_class(&self) -> &'static std::ffi::CStr {{
			unsafe {{ std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }}
		}}
		
		pub fn get_id(&self) -> u32 {{
			unsafe {{ WlProxy::get_id(&self.0) }}
		}}"#, name)?;
		
		// dispatcher implementation
		
		if !interface.events.is_empty() {
			write!(writer, r#"
		pub fn set_listener(&mut self, listener: impl {0}Listener) -> Result<()> {{
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {{
				unsafe {{
					let listener = std::mem::transmute::<_, &dyn {0}Listener>(TraitObject {{
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					}});
					
					match opcode {{"#, name)?;
			
			for (i, event) in interface.events.iter().enumerate() {
				write!(writer, "\n\t\t\t\t\t\t{} => log::trace!(\"[WAYLAND] ID `{{}}` CLASS `{{}}` VERSION `{{}}` OPCODE `{{}}` EVENT `{}` ARGS: ", i, &event.name)?;
				
				for arg in event.args.iter() {
					write!(writer, "{}: {{:?}}, ", &arg.name)?;
				}
				
				write!(writer, "\", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode")?;
				
				for (i, arg) in event.args.iter().enumerate() {
					match arg.r#type.as_str() {
						"object" => write!(writer, ", ((*args.add({})).o as *mut {}).as_mut()", i, arg.interface.as_ref()
							.map_or_else(|| "WlProxy".to_string(), |v| convert_name(v))),
						"array"  => write!(writer, ", (*args.add({})).a.as_ref().unwrap()", i),
						"string" => write!(writer, ", std::ffi::CStr::from_ptr((*args.add({0})).s as _).to_str().unwrap()", i),
						ty       => write!(writer, ", (*args.add({})).{}", i, match ty {
							"int"    => "i",
							"uint"   => "u",
							"fixed"  => "f",
							"string" => "s",
							"fd"     => "h",
							"new_id" => "n",
							_ => panic!()
						})
					}?;
				}
				
				write!(writer, "),")?;
			}
			
			writeln!(writer, r#"
						_ => ()
					}}
					
					match opcode {{
			"#)?;
			
			for (i, event) in interface.events.iter().enumerate() {
				write!(writer, "\n\t\t\t\t\t\t{} => listener.{}((proxy as *mut {}).as_mut().unwrap(), ", i, convert_fn_name(&event.name), name)?;
				
				for (i, arg) in event.args.iter().enumerate() {
					match arg.r#type.as_str() {
						"object" => write!(writer, "((*args.add({})).o as *mut {}).as_mut(), ", i, arg.interface.as_ref()
							.map_or_else(|| "WlProxy".to_string(), |v| convert_name(v))),
						"array"  => write!(writer, "(*args.add({})).a.as_ref().unwrap(), ", i),
						"string" => write!(writer, "std::ffi::CStr::from_ptr((*args.add({0})).s as _).to_str().unwrap(), ", i),
						ty       => write!(writer, "(*args.add({})).{}, ", i, match ty {
							"int"    => "i",
							"uint"   => "u",
							"fixed"  => "f",
							"string" => "s",
							"fd"     => "h",
							"new_id" => "n",
							_ => panic!()
						})
					}?;
				}
				
				write!(writer, "),")?;
			}
			
			writeln!(writer, r#"
						_ => ()
					}}
				}}
			}}
		
			let listener: Box<dyn {0}Listener> = Box::new(listener);
			unsafe {{
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {{
					0 => Ok(()),
					_ => Err(())
				}}
			}}
		}}"#, name)?;
		}
		
		// destructor
		
		let (destructor, destroy) = interface.requests.iter().fold((false, false), |(destructor, destroy), request| (
			destructor | (request.r#type.as_deref() == Some("destructor")),
			destroy | (request.name == "destroy")
		));
		
		if !destructor && destroy {
			panic!("interface {} has method named destroy but no destructor", interface.name);
		}
		
		if !destroy && interface.name != "wl_display" {
			writeln!(writer, "\n\t\tpub fn destroy(&mut self) {{\n\t\t\tunsafe {{ WlProxy::destroy(&mut self.0); }}\n\t\t}}")?;
		}
		
		// requests
		
		for (i, request) in interface.requests.into_iter().enumerate() {
			let new_id = request.args.iter().find(|arg| arg.r#type == "new_id");
			write_desc(writer, "\t\t", request.desc)?;
			
			let mutability = match request.r#type.as_deref() {
				Some("destructor") => "mut ",
				_ => ""
			};
			
			let types = match new_id {
				Some(WlArg { interface: None, .. }) => "<T>",
				_ => ""
			};
			
			write!(writer, "\t\tpub fn {}{}(\n\t\t\t&{}self", convert_fn_name(&request.name), types, mutability)?;
			
			for arg in &request.args {
				match (arg.r#type.as_str(), arg.allow_null, arg.interface.as_deref()) {
					("new_id", _,          None)            => write!(writer, ",\n\t\t\tinterface       : *const WlInterface,\n\t\t\tversion         : u32,")?,
					("new_id", _,          Some(_))         => (),
					("object", Some(true), Some(interface)) => write!(writer, ",\n\t\t\t{:16}: Option<&{}>", &arg.name, convert_name(interface))?,
					("object", _,          Some(interface)) => write!(writer, ",\n\t\t\t{:16}: &{}", &arg.name, convert_name(interface))?,
					(ty, ..)                                => write!(writer, ",\n\t\t\t{:16}: {}", &arg.name, convert_type(ty))?,
				}
			}
			
			match new_id {
				Some(WlArg { interface: Some(interface), .. }) => write!(
					writer, "\n\t\t) -> Result<Box<{}, WlAlloc>> {{\n\t\t\tlet proxy = unsafe {{ (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, {}, &{}_INTERFACE",
					convert_name(interface), i, &interface.to_uppercase()),
				Some(_) => write!(
					writer, "\n\t\t) -> Result<Box<T, WlAlloc>> {{\n\t\t\tlet proxy = unsafe {{ (LIB_WAYLAND.wl_proxy_marshal_constructor_versioned)(self as *const Self as _, {}, interface, version", i),
				None => write!(writer, "\n\t\t) {{\n\t\t\tunsafe {{ (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, {}", i)
			}?;
			
			for arg in &request.args {
				match (arg.r#type.as_str(), arg.allow_null) {
					("new_id", _) => {
						if arg.interface.is_none() {
							write!(writer, ", (*interface).name, version")?;
						}
						write!(writer, ", std::ptr::null::<u8>()")?;
					}
					("object", Some(true)) => write!(writer, ", {}.map_or(std::ptr::null_mut(), |r| r as *const {} as *mut {1})", &arg.name, convert_name(arg.interface.as_ref().unwrap()))?,
					("string", _)          => write!(writer, ", {}.as_ptr()", &arg.name)?,
					_                      => write!(writer, ", {}", &arg.name)?
				}
			}
			
			match new_id {
				Some(WlArg { interface: Some(interface), .. }) => writeln!(writer, ") as *mut {} }};", convert_name(interface))?,
				Some(_) => writeln!(writer, ") as *mut T }};")?,
				None => writeln!(writer, "); }}")?,
			}
			
			if request.r#type.as_deref() == Some("destructor") {
				writeln!(writer, "\t\t\tunsafe {{ WlProxy::destroy(&mut self.0); }}")?;
			}
			
			if new_id.is_some() {
				writeln!(writer, "\t\t\tunsafe {{ box_from_raw(proxy) }}")?;
			}
			
			writeln!(writer, "\t\t}}")?;
		}
		
		writeln!(writer, "\t}}\n")?;
		
		// listener/events
		
		if !interface.events.is_empty() {
			writeln!(writer, "\n\tpub trait {}Listener: std::any::Any {{", &name)?;
			
			for event in interface.events {
				write_desc(writer, "\t\t", event.desc)?;
				writeln!(writer, "\t\tfn {}(\n\t\t\t&self,\n\t\t\tproxy: &mut {},", convert_fn_name(&event.name), name)?;
				
				for arg in event.args {
					match arg.r#type.as_str() {
						"object" => writeln!(writer, "\t\t\t{:16}: Option<&mut {}>,", arg.name, arg.interface.as_ref()
							.map_or_else(|| "WlProxy".to_string(), |v| convert_name(v)))?,
						_        => writeln!(writer, "\t\t\t{:16}: {},", arg.name, convert_type(&arg.r#type))?
					}
				}
				
				writeln!(writer, "\t\t);")?;
			}
			
			writeln!(writer, "\t}}")?;
		}
		
		// enums
		
		for e in interface.enums {
			write_desc(writer, "\t", e.desc)?;
			writeln!(writer, "\t#[repr(u32)]\n\t#[derive(Copy, Clone, Debug, Eq, PartialEq)]\n\tpub enum {}{} {{", &name, convert_name(&e.name))?;
			
			for entry in e.entries {
				writeln!(writer, "\t\t/// {}\n\t\t{} = {},",
						 entry.summary.as_ref().map_or("", String::as_str).lines().map(str::trim).collect::<String>(),
						 convert_name(&entry.name), entry.value)?;
			}
			
			writeln!(writer, "\t}}")?;
		}
	}
	
	writeln!(writer, "}}")
}

fn get_signature(args: &[WlArg]) -> String {
	let mut buf = String::new();
	for arg in args {
		if arg.allow_null == Some(true) && matches!(arg.r#type.as_str(), "string" | "object" | "new_id" | "array") {
			buf.push('?');
		}
		
		match arg.r#type.as_str() {
			"int"    => buf.push('i'),
			"uint"   => buf.push('u'),
			"fixed"  => buf.push('f'),
			"string" => buf.push('s'),
			"object" => buf.push('o'),
			"array"  => buf.push('a'),
			"fd"     => buf.push('h'),
			"new_id" => {
				if arg.interface.is_none() {
					buf.push_str("su");
				}
				buf.push('n')
			},
			_ => panic!()
		}
	}
	buf
}

fn convert_name(name: &str) -> String {
	let mut buf = String::new();
	
	if !name.chars().next().unwrap().is_ascii_alphabetic() {
		buf.push('_');
	}
	
	for s in name.split('_') {
		let ch = s.chars().next().unwrap();
		buf.push(ch.to_ascii_uppercase());
		buf.push_str(s.trim_start_matches(ch));
	}
	
	buf
}

fn convert_fn_name(name: &str) -> &str {
	match name {
		"move" => "r#move",
		"type" => "r#type",
		name => name
	}
}

fn convert_type(ty: &str) -> &str {
	match ty {
		"int"    => "i32",
		"uint"   => "u32",
		"fixed"  => "WlFixed",
		"object" => "u32",
		"new_id" => "u32",
		"string" => "&str",
		"array"  => "&WlArray",
		"fd"     => "RawFd",
		ty => panic!("unknown type: {}", ty)
	}
}

#[derive(Debug)]
struct WlProtocol {
	name:        String,
	copyright:   Option<String>,
	description: Option<WlDescription>,
	interfaces:  Vec<WlInterface>
}

impl<'de> serde::Deserialize<'de> for WlProtocol {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		#[derive(Deserialize)]
		#[serde(rename_all = "lowercase")]
		enum Var {
			Name(String),
			Copyright(Body<String>),
			Description(WlDescription),
			Interface(WlInterface)
		}
		
		let mut name = None;
		let mut copyright = None;
		let mut description = None;
		let mut interfaces = Vec::new();
		
		for e in Vec::<Var>::deserialize(deserializer)? {
			match e {
				Var::Name(v) => name = Some(v),
				Var::Copyright(v) => copyright = Some(v.value),
				Var::Description(v) => description = Some(v),
				Var::Interface(v) => interfaces.push(v)
			}
		}
		
		Ok(Self {
			name:      name.ok_or_else(|| D::Error::missing_field("name"))?,
			copyright,
			description,
			interfaces
		})
	}
}

#[derive(Debug)]
struct WlInterface {
	name:        String,
	version:     usize,
	description: Option<WlDescription>,
	requests:    Vec<WlRequest>,
	events:      Vec<WlEvent>,
	enums:       Vec<WlEnum>
}

impl<'de> serde::Deserialize<'de> for WlInterface {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		#[derive(Deserialize)]
		#[serde(rename_all = "lowercase")]
		enum Var {
			Name(String),
			Version(usize),
			Description(WlDescription),
			Request(WlRequest),
			Event(WlEvent),
			Enum(WlEnum)
		}
		
		let mut name = None;
		let mut version = None;
		let mut description = None;
		let mut requests = Vec::new();
		let mut events = Vec::new();
		let mut enums = Vec::new();
		
		for e in Vec::<Var>::deserialize(deserializer)? {
			match e {
				Var::Name(v) => name = Some(v),
				Var::Version(v) => version = Some(v),
				Var::Description(v) => description = Some(v),
				Var::Request(v) => requests.push(v),
				Var::Event(v) => events.push(v),
				Var::Enum(v) => enums.push(v)
			}
		}
		
		Ok(Self {
			name:        name.ok_or_else(|| D::Error::missing_field("name"))?,
			version:     version.ok_or_else(|| D::Error::missing_field("version"))?,
			description,
			requests,
			events,
			enums
		})
	}
}

#[derive(Debug, Deserialize)]
struct WlDescription {
	summary: String,
	#[serde(rename = "$value", default)]
	content: String
}

#[derive(Debug, Deserialize)]
struct WlArg {
	name:       String,
	r#type:     String,
	summary:    Option<String>,
	interface:  Option<String>,
	#[serde(rename = "allow-null")]
	allow_null: Option<bool>,
	r#enum:     Option<String>
}

#[derive(Debug)]
struct WlRequest {
	name:   String,
	desc:   Option<WlDescription>,
	since:  Option<String>,
	r#type: Option<String>,
	args:   Vec<WlArg>
}

type WlEvent = WlRequest;

impl<'de> serde::Deserialize<'de> for WlRequest {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		#[derive(Deserialize)]
		#[serde(rename_all = "lowercase")]
		enum Var {
			Name(String),
			Description(WlDescription),
			Since(String),
			Type(String),
			Arg(WlArg)
		}
		
		let mut name = None;
		let mut desc = None;
		let mut since = None;
		let mut r#type = None;
		let mut args = Vec::new();
		
		for e in Vec::<Var>::deserialize(deserializer)? {
			match e {
				Var::Name(v) => name = Some(v),
				Var::Description(v) => desc = Some(v),
				Var::Since(v) => since = Some(v),
				Var::Type(v) => r#type = Some(v),
				Var::Arg(v) => args.push(v)
			}
		}
		
		Ok(Self {
			name:  name.ok_or_else(|| D::Error::missing_field("name"))?,
			desc,
			since,
			r#type,
			args
		})
	}
}



#[derive(Debug, Deserialize)]
struct WlEntry {
	name:    String,
	value:   String,
	summary: Option<String>,
	since:   Option<String>
}

#[derive(Debug)]
struct WlEnum {
	name:     String,
	desc:     Option<WlDescription>,
	since:    Option<String>,
	bitfield: Option<String>,
	entries:  Vec<WlEntry>
}

impl<'de> serde::Deserialize<'de> for WlEnum {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		#[derive(Deserialize)]
		#[serde(rename_all = "lowercase")]
		enum Var {
			Name(String),
			Description(WlDescription),
			Since(String),
			Bitfield(String),
			Entry(WlEntry)
		}
		
		let mut name = None;
		let mut desc = None;
		let mut since = None;
		let mut bitfield = None;
		let mut entries = Vec::new();
		
		for e in Vec::<Var>::deserialize(deserializer)? {
			match e {
				Var::Name(v) => name = Some(v),
				Var::Description(v) => desc = Some(v),
				Var::Since(v) => since = Some(v),
				Var::Bitfield(v) => bitfield = Some(v),
				Var::Entry(v) => entries.push(v)
			}
		}
		
		Ok(Self {
			name:  name.ok_or_else(|| D::Error::missing_field("name"))?,
			desc,
			since,
			bitfield,
			entries
		})
	}
}
