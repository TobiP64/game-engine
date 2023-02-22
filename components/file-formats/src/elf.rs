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

// TODO ELF

pub const MAGIC: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];
pub const VERSION_1: u8 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FileHeader {
	pub magic:       [u8; 4],
	pub class:       Class,
	pub data:        Data,
	pub version:     u8,
	pub os_abi:      OsAbi,
	pub abi_version: u8,
	pub _pad0:       [u8; 7],
	pub r#type:      Type,
	pub machine:     Machine,
	pub version2:    u32,
	pub entry:       usize,
	pub phoff:       usize,
	pub shoff:       usize,
	pub flags:       u32,
	pub eh_size:     u16,
	pub phent_size:  u16,
	pub ph_num:      u16,
	pub shent_size:  u16,
	pub sh_num:      u16,
	pub shstr_ndx:   u16
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Class {
	_32Bit = 1,
	_64Bit = 2,
	__NonExhaustive__
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Data {
	Little = 1,
	Big    = 2,
	__NonExhaustive__
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OsAbi {
	SystemV       = 0x00,
	HpUx          = 0x01,
	NetBsd        = 0x02,
	Linux         = 0x03,
	GnuHurd       = 0x04,
	Solaris       = 0x06,
	Aix           = 0x07,
	Irix          = 0x08,
	FreeBsd       = 0x09,
	Tru64         = 0x0A,
	NovellModesto = 0x0B,
	OpenBsd       = 0x0C,
	OpenMvs       = 0x0D,
	NonStopKernel = 0x0E,
	Aros          = 0x0F,
	FenixOs       = 0x10,
	CloudAbi      = 0x11,
	OpenVos       = 0x12,
	__NonExhaustive__
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
	None   = 0x0000,
	Rel    = 0x0001,
	Exec   = 0x0002,
	Dyn    = 0x0003,
	Core   = 0x0004,
	LoOs   = 0xFE00,
	HiOs   = 0xFEFF,
	LoProc = 0xFF00,
	HiProc = 0xFFFF
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Machine {
	Unspecified      = 0x0000,
	AttWe32100       = 0x0001,
	Sparc            = 0x0002,
	x86              = 0x0003,
	M68k             = 0x0004,
	M88k             = 0x0005,
	IntelMcu         = 0x0006,
	Intel80860       = 0x0007,
	Mips             = 0x0008,
	IbmSystem370     = 0x0009,
	MipsRs3000LE     = 0x000A,
	HpPaRisc         = 0x000E,
	Intel80960       = 0x0013,
	PPC32            = 0x0014,
	PPC64            = 0x0015,
	S390             = 0x0016,
	IbmSpuSpc        = 0x0017,
	NecV800          = 0x0024,
	FR20             = 0x0025,
	TrwRh32          = 0x0026,
	MotorolaRCE      = 0x0027,
	AArch32          = 0x0028,
	DigitalAlpha     = 0x0029,
	SuperH           = 0x002A,
	Sparc9           = 0x002B,
	SiemensTriCore   = 0x002C,
	ArgonautRiscCore = 0x002D,
	H8_300           = 0x002E,
	H8_300H          = 0x002F,
	H8S              = 0x0030,
	H8_500           = 0x0031,
	IA64             = 0x0032,
	StandfordMipsX   = 0x0033,
	MotorolaColdFire = 0x0034,
	M68HC12          = 0x0035,
	MMA              = 0x0036,
	SiemensPcp       = 0x0037,
	SonyNCpu         = 0x0038,
	Ndr1             = 0x0039,
	MotorolaStarCore = 0x003A,
	ToyotaMe16       = 0x003B,
	ST100            = 0x003C,
	ALC              = 0x003D,
	AMD64            = 0x003E,
	TMS320C6000      = 0x008C,
	AArch64          = 0x00B7,
	RiscV            = 0x00F3,
	BPF              = 0x00F7,
	WDC65C816        = 0x0101,
	__NonExhaustive__
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ProgramHeader {
	pub r#type:    ProgramType,
	pub flags64:   u32,
	pub offset:    usize,
	pub vaddr:     usize,
	pub paddr:     usize,
	pub file_size: usize,
	pub mem_size:  usize,
	pub flags32:   u32,
	pub align:     usize
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProgramType {
	Null    = 0x00000000,
	Load    = 0x00000001,
	Dynamic = 0x00000002,
	Interp  = 0x00000003,
	Note    = 0x00000004,
	ShLib   = 0x00000005,
	Phdr    = 0x00000006,
	Tls     = 0x00000007,
	LoOs    = 0x60000000,
	HiOs    = 0x6FFFFFFF,
	LoProc  = 0x70000000,
	HiProc  = 0x7FFFFFFF,
	__NonExhaustive__
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SectionHeader {
	pub r#type:     SectionType,
	pub flags:      usize,
	pub addr:       usize,
	pub offset:     usize,
	pub size:       usize,
	pub link:       u32,
	pub info:       u32,
	pub addr_align: usize,
	pub ent_size:   usize
}

impl SectionHeader {
	pub const FLAG_WRITE:            usize = 0x0000_0001;
	pub const FLAG_FLAG_ALLOC:       usize = 0x0000_0002;
	pub const FLAG_EXEC_INSTR:       usize = 0x0000_0004;
	pub const FLAG_MERGE:            usize = 0x0000_0010;
	pub const FLAG_STRINGS:          usize = 0x0000_0020;
	pub const FLAG_INFO_LINK:        usize = 0x0000_0040;
	pub const FLAG_LINK_ORDER:       usize = 0x0000_0080;
	pub const FLAG_OS_NONCONFORMING: usize = 0x0000_0100;
	pub const FLAG_GROUP:            usize = 0x0000_0200;
	pub const FLAG_TLS:              usize = 0x0000_0400;
	pub const FLAG_MASK_OS:          usize = 0x0ff0_0000;
	pub const FLAG_MASK_PROC:        usize = 0xf000_0000;
	pub const FLAG_ORDERED:          usize = 0x0400_0000;
	pub const FLAG_EXCLUDE:          usize = 0x0800_0000;
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SectionType {
	Null          = 0x0000_0000,
	ProgBits      = 0x0000_0001,
	SymTable      = 0x0000_0002,
	StrTable      = 0x0000_0003,
	Rela          = 0x0000_0004,
	Hash          = 0x0000_0005,
	Dynamic       = 0x0000_0006,
	Note          = 0x0000_0007,
	NoBits        = 0x0000_0008,
	Rel           = 0x0000_0009,
	ShLib         = 0x0000_000A,
	DynSym        = 0x0000_000B,
	InitArray     = 0x0000_000E,
	FiniArray     = 0x0000_000F,
	PreInitArray  = 0x0000_0010,
	Group         = 0x0000_0011,
	SymTableShndx = 0x0000_0012,
	Num           = 0x0000_0013,
	LoOs          = 0x6000_0000,
	__NonExhaustive__
}