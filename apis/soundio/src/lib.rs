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

#![feature(try_trait_v2, allocator_api)]
#![warn(clippy::all)]
#![allow(dead_code, clippy::from_over_into, clippy::upper_case_acronyms)]

use std::{str::FromStr, io, fmt};

pub type SoundIoResult<T> = Result<T, SoundIoError>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundIoError {
	None,
	/// Out of memory.
	NoMem,
	/// The backend does not appear to be active or running.
	InitAudioBackend,
	/// A system resource other than memory was not available.
	SystemResources,
	/// Attempted to open a device and failed.
	OpeningDevice,
	NoSuchDevice,
	/// The programmer did not comply with the API.
	Invalid,
	/// libsoundio was compiled without support for that backend.
	BackendUnavailable,
	/// An open stream had an error that can only be recovered from by
	/// destroying the stream and creating it again.
	Streaming,
	/// Attempted to use a device with parameters it cannot support.
	IncompatibleDevice,
	/// When JACK returns `JackNoSuchClient`
	NoSuchClient,
	/// Attempted to use parameters that the backend cannot support.
	IncompatibleBackend,
	/// Backend server shutdown or became inactive.
	BackendDisconnected,
	Interrupted,
	/// Buffer underrun occurred.
	Underflow,
	/// Unable to convert to or from UTF-8 to the native string format.
	EncodingString
}

impl SoundIoError {
	pub fn into_result(self) -> Result<(), Self> {
		match self {
			Self::None => Ok(()),
			err        => Err(err)
		}
	}
}

impl<T, E: From<SoundIoError>> std::ops::FromResidual<SoundIoError> for Result<T, E> {
	fn from_residual(r: SoundIoError) -> Self {
		Err(r.into())
	}
}

impl std::ops::FromResidual<Self> for SoundIoError {
	fn from_residual(residual: Self) -> Self {
		residual
	}
}

impl std::ops::Try for SoundIoError {
	type Output   = ();
	type Residual = SoundIoError;

	fn from_output(_output: Self::Output) -> Self {
		Self::None
	}

	fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
		match self {
			Self::None => std::ops::ControlFlow::Continue(()),
			err        => std::ops::ControlFlow::Break(err)
		}
	}
}

impl Into<&'static str> for SoundIoError {
	fn into(self) -> &'static str {
		unsafe { str_convert((LibSoundIo::get().soundio_strerror)(self as _)) }
	}
}

/// Specifies where a channel is physically located.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundIoChannelId {
	Invalid,

	// First of the more commonly supported ids.
	FrontLeft,
	FrontRight,
	FrontCenter,
	Lfe,
	BackLeft,
	BackRight,
	FrontLeftCenter,
	FrontRightCenter,
	BackCenter,
	SideLeft,
	SideRight,
	TopCenter,
	TopFrontLeft,
	TopFrontCenter,
	TopFrontRight,
	TopBackLeft,
	TopBackCenter,
	// Last of the more commonly supported ids.
	TopBackRight,

	// First of the less commonly supported ids.
	BackLeftCenter,
	BackRightCenter,
	FrontLeftWide,
	FrontRightWide,
	FrontLeftHigh,
	FrontCenterHigh,
	FrontRightHigh,
	TopFrontLeftCenter,
	TopFrontRightCenter,
	TopSideLeft,
	TopSideRight,
	LeftLfe,
	RightLfe,
	Lfe2,
	BottomCenter,
	BottomLeftCenter,
	BottomRightCenter,

	// Mid/side recording
	MsMid,
	MsSide,

	// first order ambisonic channels
	AmbisonicW,
	AmbisonicX,
	AmbisonicY,
	AmbisonicZ,

	// X-Y Recording
	XyX,
	XyY,

	// First of the "other" channel ids
	HeadphonesLeft,
	HeadphonesRight,
	ClickTrack,
	ForeignLanguage,
	HearingImpaired,
	Narration,
	Haptic,
	// Last of the "other" channel ids
	DialogCentricMix,

	Aux,
	Aux0,
	Aux1,
	Aux2,
	Aux3,
	Aux4,
	Aux5,
	Aux6,
	Aux7,
	Aux8,
	Aux9,
	Aux10,
	Aux11,
	Aux12,
	Aux13,
	Aux14,
	Aux15
}

impl Into<&'static str> for SoundIoChannelId {
	fn into(self) -> &'static str {
		unsafe { str_convert((LibSoundIo::get().soundio_channel_get_name)(self)) }
	}
}

impl FromStr for SoundIoChannelId {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok((LibSoundIo::get().soundio_channel_parse_id)(s.as_ptr(), s.len() as _))
	}
}

/// Built-in channel layouts for convenience.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundIoChannelLayoutId {
	IdMono,
	IdStereo,
	Id2Point1,
	Id3Point0,
	Id3Point0Back,
	Id3Point1,
	Id4Point0,
	IdQuad,
	IdQuadSide,
	Id4Point1,
	Id5Point0Back,
	Id5Point0Side,
	Id5Point1,
	Id5Point1Back,
	Id6Point0Side,
	Id6Point0Front,
	IdHexagonal,
	Id6Point1,
	Id6Point1Back,
	Id6Point1Front,
	Id7Point0,
	Id7Point0Front,
	Id7Point1,
	Id7Point1Wide,
	Id7Point1WideBack,
	IdOctagonal
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundIoBackend {
	None,
	Jack,
	PulseAudio,
	Alsa,
	CoreAudio,
	Wasapi,
	Dummy,
}

impl SoundIoBackend {
	pub fn have(self) -> bool {
		(LibSoundIo::get().soundio_backend_have)(self)
	}
}

impl Into<&'static str> for SoundIoBackend {
	fn into(self) -> &'static str {
		unsafe { str_convert((LibSoundIo::get().soundio_backend_get_name)(self)) }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundIoDeviceAim {
	/// capture / recording
	Input,
	/// playback
	Output
}

/// For your convenience, Native Endian and Foreign Endian constants are defined
/// which point to the respective SoundIoFormat values.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoundIoFormat {
	Invalid,
	/// Signed 8 bit
	S8,
	/// Unsigned 8 bit
	U8,
	/// Signed 16 bit Little Endian
	S16LE,
	/// Signed 16 bit Big Endian
	S16BE,
	/// Unsigned 16 bit Little Endian
	U16LE,
	/// Unsigned 16 bit Big Endian
	U16BE,
	/// Signed 24 bit Little Endian using low three bytes in 32-bit word
	S24LE,
	/// Signed 24 bit Big Endian using low three bytes in 32-bit word
	S24BE,
	/// Unsigned 24 bit Little Endian using low three bytes in 32-bit word
	U24LE,
	/// Unsigned 24 bit Big Endian using low three bytes in 32-bit word
	U24BE,
	/// Signed 32 bit Little Endian
	S32LE,
	/// Signed 32 bit Big Endian
	S32BE,
	/// Unsigned 32 bit Little Endian
	U32LE,
	/// Unsigned 32 bit Big Endian
	U32BE,
	/// Float 32 bit Little Endian, Range -1.0 to 1.0
	Float32LE,
	/// Float 32 bit Big Endian, Range -1.0 to 1.0
	Float32BE,
	/// Float 64 bit Little Endian, Range -1.0 to 1.0
	Float64LE,
	/// Float 64 bit Big Endian, Range -1.0 to 1.0
	Float64BE,
}

impl SoundIoFormat {
	pub fn get_bytes_per_sample(self) -> usize {
		(LibSoundIo::get().soundio_format_get_bytes_per_sample)(self) as _
	}
}

impl Into<&'static str> for SoundIoFormat {
	fn into(self) -> &'static str {
		unsafe { str_convert((LibSoundIo::get().soundio_format_string)(self)) }
	}
}

pub const SOUNDIO_MAX_CHANNELS: usize = 24;

/// The size of this struct is OK to use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SoundIoChannelLayout {
	name:          *const u8,
	channel_count: u32,
	channels:      [SoundIoChannelId; SOUNDIO_MAX_CHANNELS]
}

impl SoundIoChannelLayout {
	/// Populates the name field of layout if it matches a builtin one.
	/// returns whether it found a match
	pub fn detect_builtin(&mut self) ->  bool {
		(LibSoundIo::get().soundio_channel_layout_detect_builtin)(self)
	}
}

impl PartialEq for SoundIoChannelLayout {
	fn eq(&self, other: &Self) -> bool {
		(LibSoundIo::get().soundio_channel_layout_equal)(self, other)
	}
}

/// The size of this struct is OK to use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SoundIoSampleRateRange {
	min: u32,
	max: u32
}

/// The size of this struct is OK to use.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SoundIoChannelArea {
	/// Base address of buffer.
	ptr: *mut u8,
	/// How many bytes it takes to get from the beginning of one sample to
	/// the beginning of the next sample.
	step: u32
}

/// The size of this struct is not part of the API or ABI.
#[repr(C)]
#[derive(Debug)]
pub struct SoundIo {
	/// Optional. Put whatever you want here. Defaults to NULL.
	user_data: *mut u8,
	/// Optional callback. Called when the list of devices change. Only called
	/// during a call to ::soundio_flush_events or ::soundio_wait_events.
	on_devices_change: extern "C" fn(*mut SoundIo),
	/// Optional callback. Called when the backend disconnects. For example,
	/// when the JACK server shuts down. When this happens, listing devices
	/// and opening streams will always fail with
	/// SoundIoErrorBackendDisconnected. This callback is only called during a
	/// call to ::soundio_flush_events or ::soundio_wait_events.
	/// If you do not supply a callback, the default will crash your program
	/// with an error message. This callback is also called when the thread
	/// that retrieves device information runs into an unrecoverable condition
	/// such as running out of memory.
	///
	/// Possible errors:
	/// * #SoundIoErrorBackendDisconnected
	/// * #SoundIoErrorNoMem
	/// * #SoundIoErrorSystemResources
	/// * #SoundIoErrorOpeningDevice - unexpected problem accessing device
	///   information
	on_backend_disconnect: extern "C" fn(*mut SoundIo, u32),
	/// Optional callback. Called from an unknown thread that you should not use
	/// to call any soundio functions. You may use this to signal a condition
	/// variable to wake up. Called when ::soundio_wait_events would be woken up.
	on_events_signal: extern "C" fn(*mut SoundIo),

	/// Read-only. After calling ::soundio_connect or ::soundio_connect_backend,
	/// this field tells which backend is currently connected.
	current_backend: SoundIoBackend,

	/// Optional: Application name.
	/// PulseAudio uses this for "application name".
	/// JACK uses this for `client_name`.
	/// Must not contain a colon (":").
	app_name: *mut u8,

	/// Optional: Real time priority warning.
	/// This callback is fired when making thread real-time priority failed. By
	/// default, it will print to stderr only the first time it is called
	/// a message instructing the user how to configure their system to allow
	/// real-time priority threads. This must be set to a function not NULL.
	/// To silence the warning, assign this to a function that does nothing.
	emit_rtprio_warning: extern "C" fn(),

	/// Optional: JACK info callback.
	/// By default, libsoundio sets this to an empty function in order to
	/// silence stdio messages from JACK. You may override the behavior by
	/// setting this to `NULL` or providing your own function. This is
	/// registered with JACK regardless of whether ::soundio_connect_backend
	/// succeeds.
	jack_info_callback: fn(*const u8),
	/// Optional: JACK error callback.
	/// See SoundIo::jack_info_callback
	jack_error_callback: fn(*const u8),
}

impl SoundIo {
	pub fn version_string() -> &'static str {
		unsafe {
			LIB_SOUNDIO.load();
			str_convert((LibSoundIo::get().soundio_version_string)())
		}
	}

	pub fn version() -> (usize, usize, usize) {
		unsafe {
			LIB_SOUNDIO.load();
			(
				(LibSoundIo::get().soundio_version_major)() as _,
				(LibSoundIo::get().soundio_version_minor)() as _,
				(LibSoundIo::get().soundio_version_patch)() as _,
			)
		}
	}

	pub fn builtin_channel_layouts() -> SoundIoChannelLayoutIter {
		SoundIoChannelLayoutIter {
			idx: 0,
			len: (LibSoundIo::get().soundio_channel_layout_builtin_count)() as _
		}
	}

	pub fn default_channel_layout(channels: usize) -> Option<&'static SoundIoChannelLayout> {
		unsafe { (LibSoundIo::get().soundio_channel_layout_get_default)(channels as _).as_ref() }
	}

	pub fn new() -> Option<Box<Self, SoundIoAlloc>> {
		unsafe {
			LIB_SOUNDIO.load();
			box_from_raw((LibSoundIo::get().soundio_create)())
		}
	}

	pub fn connect(&mut self, backend: Option<SoundIoBackend>) -> SoundIoResult<()> {
		match backend {
			None => (LibSoundIo::get().soundio_connect)(self),
			Some(backend) => (LibSoundIo::get().soundio_connect_backend)(self, backend)
		}.into_result()
	}

	pub fn disconnect(&mut self) {
		(LibSoundIo::get().soundio_disconnect)(self)
	}

	pub fn flush_events(&mut self) {
		(LibSoundIo::get().soundio_flush_events)(self)
	}

	pub fn wait_events(&mut self) {
		(LibSoundIo::get().soundio_wait_events)(self)
	}

	pub fn wakeup(&mut self) {
		(LibSoundIo::get().soundio_wakeup)(self)
	}

	pub fn force_device_scan(&mut self) {
		(LibSoundIo::get().soundio_force_device_scan)(self)
	}

	pub fn backends(&mut self) -> SoundIoIter<SoundIoBackend> {
		fn next(soundio: &mut SoundIo, idx: usize) ->  SoundIoBackend {
			(LibSoundIo::get().soundio_get_backend)(soundio, idx as _)
		}

		SoundIoIter {
			len:     (LibSoundIo::get().soundio_backend_count)(self) as _,
			soundio: self,
			idx:     0,
			next
		}
	}

	pub fn input_devices(&mut self) -> SoundIoIter<Box<SoundIoDevice, SoundIoAlloc>> {
		fn next(soundio: &mut SoundIo, idx: usize) ->  Box<SoundIoDevice, SoundIoAlloc> {
			unsafe { box_from_raw((LibSoundIo::get().soundio_get_input_device)(
				soundio, idx as _)).unwrap() }
		}

		SoundIoIter {
			len:     (LibSoundIo::get().soundio_input_device_count)(self) as _,
			soundio: self,
			idx:     0,
			next
		}
	}

	pub fn output_devices(&mut self) -> SoundIoIter<Box<SoundIoDevice, SoundIoAlloc>> {
		fn next(soundio: &mut SoundIo, idx: usize) ->  Box<SoundIoDevice, SoundIoAlloc> {
			unsafe { box_from_raw((LibSoundIo::get().soundio_get_output_device)(
				soundio, idx as _)).unwrap() }
		}

		SoundIoIter {
			len:     (LibSoundIo::get().soundio_output_device_count)(self) as _,
			soundio: self,
			idx:     0,
			next
		}
	}

	pub fn default_input_device(&mut self) -> Option<Box<SoundIoDevice, SoundIoAlloc>> {
		let idx = (LibSoundIo::get().soundio_default_input_device_index)(self);
		(idx != !0).then(|| unsafe { box_from_raw(
			(LibSoundIo::get().soundio_get_input_device)(self, idx)).unwrap() })
	}

	pub fn default_output_device(&mut self) -> Option<Box<SoundIoDevice, SoundIoAlloc>> {
		let idx = (LibSoundIo::get().soundio_default_output_device_index)(self);
		(idx != !0).then(|| unsafe { box_from_raw(
			(LibSoundIo::get().soundio_get_output_device)(self, idx)).unwrap() })
	}
}

impl Drop for SoundIo {
	fn drop(&mut self) {
		(LibSoundIo::get().soundio_destroy)(self)
	}
}

pub struct SoundIoIter<'a, T> {
	soundio: &'a mut SoundIo,
	idx:     usize,
	len:     usize,
	next:    fn(&mut SoundIo, usize) -> T
}

impl<T> Iterator for SoundIoIter<'_, T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if  self.idx == self.len { return None; }

		let v = (self.next)(self.soundio, self.idx);
		self.idx += 1;
		Some(v)
	}
}

pub struct SoundIoChannelLayoutIter {
	idx: usize,
	len: usize
}

impl Iterator for SoundIoChannelLayoutIter {
	type Item = &'static SoundIoChannelLayout;

	fn next(&mut self) -> Option<Self::Item> {
		if  self.idx == self.len { return None; }

		let v = unsafe { (LibSoundIo::get().soundio_channel_layout_get_builtin)(self.idx as _).as_ref().unwrap() };
		self.idx += 1;
		Some(v)
	}
}

/// The size of this struct is not part of the API or ABI.
#[repr(C)]
pub struct SoundIoDevice {
	/// Read-only. Set automatically.
	soundio:      *mut SoundIo,
	/// A string of bytes that uniquely identifies this device.
	/// If the same physical device supports both input and output, that makes
	/// one SoundIoDevice for the input and one SoundIoDevice for the output.
	/// In this case, the id of each SoundIoDevice will be the same, and
	/// SoundIoDevice::aim will be different. Additionally, if the device
	/// supports raw mode, there may be up to four devices with the same id:
	/// one for each value of SoundIoDevice::is_raw and one for each value of
	/// SoundIoDevice::aim.
	id:           *mut u8,
	/// User-friendly UTF-8 encoded text to describe the device.
	name:         *mut u8,
	/// Tells whether this device is an input device or an output device.
	aim:          SoundIoDeviceAim,

	/// Channel layouts are handled similarly to SoundIoDevice::formats.
	/// If this information is missing due to a SoundIoDevice::probe_error,
	/// layouts will be NULL. It's OK to modify this data, for example calling
	/// ::soundio_sort_channel_layouts on it.
	/// Devices are guaranteed to have at least 1 channel layout.
	layouts:      *mut SoundIoChannelLayout,
	layout_count: u32,
	/// See SoundIoDevice::current_format
	current_layout: SoundIoChannelLayout,

	/// List of formats this device supports. See also
	/// SoundIoDevice::current_format.
	formats:        *mut SoundIoFormat,
	/// How many formats are available in SoundIoDevice::formats.
	format_count:   u32,
	/// A device is either a raw device or it is a virtual device that is
	/// provided by a software mixing service such as dmix or PulseAudio (see
	/// SoundIoDevice::is_raw). If it is a raw device,
	/// current_format is meaningless;
	/// the device has no current format until you open it. On the other hand,
	/// if it is a virtual device, current_format describes the
	/// destination sample format that your audio will be converted to. Or,
	/// if you're the lucky first application to open the device, you might
	/// cause the current_format to change to your format.
	/// Generally, you want to ignore current_format and use
	/// whatever format is most convenient
	/// for you which is supported by the device, because when you are the only
	/// application left, the mixer might decide to switch
	/// current_format to yours. You can learn the supported formats via
	/// formats and SoundIoDevice::format_count. If this information is missing
	/// due to a probe error, formats will be `NULL`. If current_format is
	/// unavailable, it will be set to #SoundIoFormatInvalid.
	/// Devices are guaranteed to have at least 1 format available.
	current_format: SoundIoFormat,

	/// Sample rate is the number of frames per second.
	/// Sample rate is handled very similar to SoundIoDevice::formats.
	/// If sample rate information is missing due to a probe error, the field
	/// will be set to NULL.
	/// Devices which have SoundIoDevice::probe_error set to #SoundIoErrorNone are
	/// guaranteed to have at least 1 sample rate available.
	sample_rates: *mut SoundIoSampleRateRange,
	/// How many sample rate ranges are available in
	/// SoundIoDevice::sample_rates. 0 if sample rate information is missing
	/// due to a probe error.
	sample_rate_count: u32,
	/// See SoundIoDevice::current_format
	/// 0 if sample rate information is missing due to a probe error.
	sample_rate_current: u32,

	/// Software latency minimum in seconds. If this value is unknown or
	/// irrelevant, it is set to 0.0.
	/// For PulseAudio and WASAPI this value is unknown until you open a
	/// stream.
	software_latency_min: f64,
	/// Software latency maximum in seconds. If this value is unknown or
	/// irrelevant, it is set to 0.0.
	/// For PulseAudio and WASAPI this value is unknown until you open a
	/// stream.
	software_latency_max: f64,
	/// Software latency in seconds. If this value is unknown or
	/// irrelevant, it is set to 0.0.
	/// For PulseAudio and WASAPI this value is unknown until you open a
	/// stream.
	/// See SoundIoDevice::current_format
	software_latency_current: f64,

	/// Raw means that you are directly opening the hardware device and not
	/// going through a proxy such as dmix, PulseAudio, or JACK. When you open a
	/// raw device, other applications on the computer are not able to
	/// simultaneously access the device. Raw devices do not perform automatic
	/// resampling and thus tend to have fewer formats available.
	is_raw: bool,

	/// Devices are reference counted. See ::soundio_device_ref and
	/// ::soundio_device_unref.
	ref_count: u32,

	/// This is set to a SoundIoError representing the result of the device
	/// probe. Ideally this will be SoundIoErrorNone in which case all the
	/// fields of the device will be populated. If there is an error code here
	/// then information about formats, sample rates, and channel layouts might
	/// be missing.
	///
	/// Possible errors:
	/// * #SoundIoErrorOpeningDevice
	/// * #SoundIoErrorNoMem
	probe_error: u32
}

impl SoundIoDevice {
	/// Convenience function. Returns whether `format` is included in the device's
	/// supported formats.
	pub fn supports_format(&mut self, format: SoundIoFormat) -> bool {
		(LibSoundIo::get().soundio_device_supports_format)(self, format)
	}

	/// Convenience function. Returns whether `layout` is included in the device's
	/// supported channel layouts.
	pub fn supports_layout(&mut self, layout: &SoundIoChannelLayout) -> bool {
		(LibSoundIo::get().soundio_device_supports_layout)(self, layout)
	}

	/// Convenience function. Returns whether `sample_rate` is included in the
	/// device's supported sample rates.
	pub fn supports_sample_rate(&mut self, sample_rate: usize) -> bool {
		(LibSoundIo::get().soundio_device_supports_sample_rate)(self, sample_rate as _)
	}

	/// Convenience function. Returns the available sample rate nearest to
	/// `sample_rate`, rounding up.
	pub fn nearest_sample_rate(&mut self, sample_rate: usize) -> usize {
		(LibSoundIo::get().soundio_device_nearest_sample_rate)(self, sample_rate as _) as _
	}
}

impl PartialEq for SoundIoDevice {
	fn eq(&self, other: &Self) -> bool {
		(LibSoundIo::get().soundio_device_equal)(self, other)
	}
}

impl Drop for SoundIoDevice {
	fn drop(&mut self) {
		(LibSoundIo::get().soundio_device_unref)(self)
	}
}

impl fmt::Debug for SoundIoDevice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SoundIoDevice")
			.field("id", &unsafe { str_convert(self.id) })
			.field("name", &unsafe { str_convert(self.name) })
			.finish()
	}
}

/// The size of this struct is not part of the API or ABI.
#[repr(C)]
#[derive(Debug)]
pub struct SoundIoOutStream {
	/// Populated automatically when you call ::soundio_outstream_create.
	device: *mut SoundIoDevice,

	/// Defaults to #SoundIoFormatFloat32NE, followed by the first one
	/// supported.
	format: SoundIoFormat,

	/// Sample rate is the number of frames per second.
	/// Defaults to 48000 (and then clamped into range).
	sample_rate: u32,

	/// Defaults to Stereo, if available, followed by the first layout
	/// supported.
	layout: SoundIoChannelLayout,

	/// Ignoring hardware latency, this is the number of seconds it takes for
	/// the last sample in a full buffer to be played.
	/// After you call ::soundio_outstream_open, this value is replaced with the
	/// actual software latency, as near to this value as possible.
	/// On systems that support clearing the buffer, this defaults to a large
	/// latency, potentially upwards of 2 seconds, with the understanding that
	/// you will call ::soundio_outstream_clear_buffer when you want to reduce
	/// the latency to 0. On systems that do not support clearing the buffer,
	/// this defaults to a reasonable lower latency value.
	///
	/// On backends with high latencies (such as 2 seconds), `frame_count_min`
	/// will be 0, meaning you don't have to fill the entire buffer. In this
	/// case, the large buffer is there if you want it; you only have to fill
	/// as much as you want. On backends like JACK, `frame_count_min` will be
	/// equal to `frame_count_max` and if you don't fill that many frames, you
	/// will get glitches.
	///
	/// If the device has unknown software latency min and max values, you may
	/// still set this, but you might not get the value you requested.
	/// For PulseAudio, if you set this value to non-default, it sets
	/// `PA_STREAM_ADJUST_LATENCY` and is the value used for `maxlength` and
	/// `tlength`.
	///
	/// For JACK, this value is always equal to
	/// SoundIoDevice::software_latency_current of the device.
	software_latency: f64,
	/// Core Audio and WASAPI only: current output Audio Unit volume. Float, 0.0-1.0.
	volume: f32,
	/// Defaults to NULL. Put whatever you want here.
	user_data: *mut u8,
	/// In this callback, you call ::soundio_outstream_begin_write and
	/// ::soundio_outstream_end_write as many times as necessary to write
	/// at minimum `frame_count_min` frames and at maximum `frame_count_max`
	/// frames. `frame_count_max` will always be greater than 0. Note that you
	/// should write as many frames as you can; `frame_count_min` might be 0 and
	/// you can still get a buffer underflow if you always write
	/// `frame_count_min` frames.
	///
	/// For Dummy, ALSA, and PulseAudio, `frame_count_min` will be 0. For JACK
	/// and CoreAudio `frame_count_min` will be equal to `frame_count_max`.
	///
	/// The code in the supplied function must be suitable for real-time
	/// execution. That means that it cannot call functions that might block
	/// for a long time. This includes all I/O functions (disk, TTY, network),
	/// malloc, free, printf, pthread_mutex_lock, sleep, wait, poll, select,
	/// pthread_join, pthread_cond_wait, etc.
	write_callback: extern "C" fn(*mut SoundIoOutStream, frame_count_min: u32, frame_count_max: u32),
	/// This optional callback happens when the sound device runs out of
	/// buffered audio data to play. After this occurs, the outstream waits
	/// until the buffer is full to resume playback.
	/// This is called from the SoundIoOutStream::write_callback thread context.
	underflow_callback: extern "C" fn(*mut SoundIoOutStream),
	/// Optional callback. `err` is always SoundIoErrorStreaming.
	/// SoundIoErrorStreaming is an unrecoverable error. The stream is in an
	/// invalid state and must be destroyed.
	/// If you do not supply error_callback, the default callback will print
	/// a message to stderr and then call `abort`.
	/// This is called from the SoundIoOutStream::write_callback thread context.
	error_callback: extern "C" fn(*mut SoundIoOutStream, err: u32),

	/// Optional: Name of the stream. Defaults to "SoundIoOutStream"
	/// PulseAudio uses this for the stream name.
	/// JACK uses this for the client name of the client that connects when you
	/// open the stream.
	/// WASAPI uses this for the session display name.
	/// Must not contain a colon (":").
	name: *const u8,

	/// Optional: Hint that this output stream is nonterminal. This is used by
	/// JACK and it means that the output stream data originates from an input
	/// stream. Defaults to `false`.
	non_terminal_hint: bool,


	/// computed automatically when you call ::soundio_outstream_open
	bytes_per_frame: u32,
	/// computed automatically when you call ::soundio_outstream_open
	bytes_per_sample: u32,

	/// If setting the channel layout fails for some reason, this field is set
	/// to an error code. Possible error codes are:
	/// * #SoundIoErrorIncompatibleDevice
	layout_error: u32,
}

impl SoundIoOutStream {
	/// Allocates memory and sets defaults. Next you should fill out the struct fields
	/// and then call SoundIoOutStream::open. Sets all fields to defaults.
	pub fn new(device: &mut SoundIoDevice) -> Box<Self, SoundIoAlloc> {
		unsafe { box_from_raw((LibSoundIo::get().soundio_outstream_create)(device))
			.expect("failed to create `SoundIoOutStream`, out of memory") }
	}

	/// After you call this function, SoundIoOutStream::software_latency is set to
	/// the correct value.
	///
	/// The next thing to do is call ::soundio_outstream_start.
	/// If this function returns an error, the outstream is in an invalid state and
	/// you must drop it.
	///
	/// Possible errors:
	/// * SoundIoError::Invalid
	///   * SoundIoDevice::aim is not SoundIoDeviceAim::Output
	///   * SoundIoOutStream::format is not valid
	///   * SoundIoOutStream::channel_count is greater than SOUNDIO_MAX_CHANNELS
	/// * SoundIoError::NoMem
	/// * SoundIoError::OpeningDevice
	/// * SoundIoError::BackendDisconnected
	/// * SoundIoError::SystemResources
	/// * SoundIoError::NoSuchClient - when JACK returns `JackNoSuchClient`
	/// * SoundIoError::IncompatibleBackend - SoundIoOutStream::channel_count is
	///   greater than the number of channels the backend can handle.
	/// * SoundIoError::IncompatibleDevice - stream parameters requested are not
	///   compatible with the chosen device.
	pub fn open(&mut self) -> SoundIoResult<()>  {
		(LibSoundIo::get().soundio_outstream_open)(self).into_result()
	}

	/// After you call this function, SoundIoOutStream::write_callback will be called.
	///
	/// This function might directly call SoundIoOutStream::write_callback.
	///
	/// # Possible errors:
	/// * SoundIoError::Streaming
	/// * SoundIoError::NoMem
	/// * SoundIoError::SystemResources
	/// * SoundIoError::BackendDisconnected
	pub fn start(&mut self) -> SoundIoResult<()>  {
		(LibSoundIo::get().soundio_outstream_start)(self).into_result()
	}

	/// Clears the output stream buffer.
	/// This function can be called from any thread.
	/// This function can be called regardless of whether the outstream is paused
	/// or not.
	/// Some backends do not support clearing the buffer. On these backends this
	/// function will return SoundIoErrorIncompatibleBackend.
	/// Some devices do not support clearing the buffer. On these devices this
	/// function might return SoundIoError::IncompatibleDevice.
	/// Possible errors:
	///
	/// * SoundIoError::Streaming
	/// * SoundIoError::IncompatibleBackend
	/// * SoundIoError::IncompatibleDevice
	pub fn clear_buffer(&mut self) -> SoundIoResult<()> {
		(LibSoundIo::get().soundio_outstream_clear_buffer)(self).into_result()
	}

	/// If the underlying backend and device support pausing, this pauses the
	/// stream. SoundIoOutStream::write_callback may be called a few more times if
	/// the buffer is not full.
	/// Pausing might put the hardware into a low power state which is ideal if your
	/// software is silent for some time.
	/// This function may be called from any thread context, including
	/// SoundIoOutStream::write_callback.
	/// Pausing when already paused or unpausing when already unpaused has no
	/// effect and returns SoundIoError::None.
	///
	/// Possible errors:
	/// * SoundIoError::BackendDisconnected
	/// * SoundIoError::Streaming
	/// * SoundIoError::IncompatibleDevice - device does not support
	///   pausing/unpausing. This error code might not be returned even if the
	///   device does not support pausing/unpausing.
	/// * SoundIoError::IncompatibleBackend - backend does not support
	///   pausing/unpausing.
	/// * SoundIoError::Invalid - outstream not opened and started
	pub fn pause(&mut self, pause: bool) -> SoundIoResult<()>  {
		(LibSoundIo::get().soundio_outstream_pause)(self, pause).into_result()
	}

	/// Obtain the total number of seconds that the next frame written after the
	/// last frame written with ::soundio_outstream_end_write will take to become
	/// audible. This includes both software and hardware latency. In other words,
	/// if you call this function directly after calling ::soundio_outstream_end_write,
	/// this gives you the number of seconds that the next frame written will take
	/// to become audible.
	///
	/// This function must be called only from within SoundIoOutStream::write_callback.
	///
	/// # Possible errors:
	/// * SoundIoError::Streaming
	pub fn get_latency(&mut self) -> SoundIoResult<f64> {
		let mut latency = 0f64;
		(LibSoundIo::get().soundio_outstream_get_latency)(self, &mut latency).into_result()
			.map(|_| latency)
	}

	pub fn set_volume(&mut self, volume: f64) -> SoundIoResult<()> {
		(LibSoundIo::get().soundio_outstream_set_volume)(self, volume).into_result()
	}

	#[allow(clippy::type_complexity)]
	pub fn begin_write(&mut self, frame_count: usize) -> SoundIoResult<(usize, [(usize, &mut [u8]); SOUNDIO_MAX_CHANNELS])> {
		let mut channels = [SoundIoChannelArea { ptr: std::ptr::null_mut(), step: 0 }; SOUNDIO_MAX_CHANNELS];
		let mut pp = [std::ptr::null_mut::<SoundIoChannelArea>(); SOUNDIO_MAX_CHANNELS];

		for i in 0..SOUNDIO_MAX_CHANNELS {
			pp[i] = &mut channels[i];
		}

		let mut frame_count = frame_count as u32;
		(LibSoundIo::get().soundio_outstream_begin_write)(self, pp.as_mut_ptr(), &mut frame_count)?;

		let mut channels: [(usize, &mut [u8]); 24] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

		for i in 0..SOUNDIO_MAX_CHANNELS {
			unsafe { channels[i] = ((*pp[i]).step as usize, std::slice::from_raw_parts_mut(
				(*pp[i]).ptr, ((*pp[i]).step * frame_count) as _)); }
		}

		Ok((frame_count as _, channels))
	}

	pub fn end_write(&mut self) -> SoundIoResult<()> {
		(LibSoundIo::get().soundio_outstream_end_write)(self).into_result()
	}
}

impl Drop for SoundIoOutStream {
	fn drop(&mut self) {
		(LibSoundIo::get().soundio_outstream_destroy)(self)
	}
}

/// The size of this struct is not part of the API or ABI.
#[repr(C)]
#[derive(Debug)]
pub struct SoundIoInStream {
	/// Populated automatically when you call ::soundio_outstream_create.
	device: *mut SoundIoDevice,

	/// Defaults to #SoundIoFormatFloat32NE, followed by the first one
	/// supported.
	format: SoundIoFormat,

	/// Sample rate is the number of frames per second.
	/// Defaults to max(sample_rate_min, min(sample_rate_max, 48000))
	sample_rate: u32,

	/// Defaults to Stereo, if available, followed by the first layout
	/// supported.
	layout: SoundIoChannelLayout,

	/// Ignoring hardware latency, this is the number of seconds it takes for a
	/// captured sample to become available for reading.
	/// After you call ::soundio_instream_open, this value is replaced with the
	/// actual software latency, as near to this value as possible.
	/// A higher value means less CPU usage. Defaults to a large value,
	/// potentially upwards of 2 seconds.
	/// If the device has unknown software latency min and max values, you may
	/// still set this, but you might not get the value you requested.
	/// For PulseAudio, if you set this value to non-default, it sets
	/// `PA_STREAM_ADJUST_LATENCY` and is the value used for `fragsize`.
	/// For JACK, this value is always equal to
	/// SoundIoDevice::software_latency_current
	software_latency: f64,

	/// Defaults to NULL. Put whatever you want here.
	user_data: *mut u8,
	/// In this function call ::soundio_instream_begin_read and
	/// ::soundio_instream_end_read as many times as necessary to read at
	/// minimum `frame_count_min` frames and at maximum `frame_count_max`
	/// frames. If you return from read_callback without having read
	/// `frame_count_min`, the frames will be dropped. `frame_count_max` is how
	/// many frames are available to read.
	///
	/// The code in the supplied function must be suitable for real-time
	/// execution. That means that it cannot call functions that might block
	/// for a long time. This includes all I/O functions (disk, TTY, network),
	/// malloc, free, printf, pthread_mutex_lock, sleep, wait, poll, select,
	/// pthread_join, pthread_cond_wait, etc.
	read_callback: extern "C" fn(*mut SoundIoInStream, frame_count_min: u32, frame_count_max: u32),
	/// This optional callback happens when the sound device buffer is full,
	/// yet there is more captured audio to put in it.
	/// This is never fired for PulseAudio.
	/// This is called from the SoundIoInStream::read_callback thread context.
	overflow_callback: extern "C" fn(*mut SoundIoInStream),
	/// Optional callback. `err` is always SoundIoErrorStreaming.
	/// SoundIoErrorStreaming is an unrecoverable error. The stream is in an
	/// invalid state and must be destroyed.
	/// If you do not supply `error_callback`, the default callback will print
	/// a message to stderr and then abort().
	/// This is called from the SoundIoInStream::read_callback thread context.
	error_callback: extern "C" fn(*mut SoundIoInStream, err: u32),

	/// Optional: Name of the stream. Defaults to "SoundIoInStream";
	/// PulseAudio uses this for the stream name.
	/// JACK uses this for the client name of the client that connects when you
	/// open the stream.
	/// WASAPI uses this for the session display name.
	/// Must not contain a colon (":").
	name: *const u8,

	/// Optional: Hint that this input stream is nonterminal. This is used by
	/// JACK and it means that the data received by the stream will be
	/// passed on or made available to another stream. Defaults to `false`.
	non_terminal_hint: bool,

	/// computed automatically when you call ::soundio_instream_open
	bytes_per_frame: u32,
	/// computed automatically when you call ::soundio_instream_open
	bytes_per_sample: u32,

	/// If setting the channel layout fails for some reason, this field is set
	/// to an error code. Possible error codes are: #SoundIoErrorIncompatibleDevice
	layout_error: u32
}

impl SoundIoInStream {
	/// Allocates memory and sets defaults. Next you should fill out the struct fields
	/// and then call SoundIoInStream::open. Sets all fields to defaults.
	pub fn new(device: &mut SoundIoDevice) -> Box<Self, SoundIoAlloc> {
		unsafe { box_from_raw((LibSoundIo::get().soundio_instream_create)(device))
			.expect("failed to create `SoundIoInStream`, out of memory") }
	}

	/// After you call this function, SoundIoInStream::software_latency is set to the correct
	/// value.
	/// The next thing to do is call ::soundio_instream_start.
	/// If this function returns an error, the instream is in an invalid state and
	/// you must call ::soundio_instream_destroy on it.
	///
	/// # Possible errors:
	/// * SoundIoError::Invalid
	///   * device aim is not SoundIoDeviceAim::Input
	///   * format is not valid
	///   * requested layout channel count > SOUNDIO_MAX_CHANNELS
	/// * SoundIoError::OpeningDevice
	/// * SoundIoError::NoMem
	/// * SoundIoError::BackendDisconnected
	/// * SoundIoError::SystemResources
	/// * SoundIoError::NoSuchClient
	/// * SoundIoError::IncompatibleBackend
	/// * SoundIoError::IncompatibleDevice
	pub fn open(&mut self) -> SoundIoResult<()> {
		(LibSoundIo::get().soundio_instream_open)(self).into_result()
	}

	/// After you call this function, SoundIoInStream::read_callback will be called.
	///
	/// # Possible errors:
	/// * SoundIoError::BackendDisconnected
	/// * SoundIoError::Streaming
	/// * SoundIoError::OpeningDevice
	/// * SoundIoError::SystemResources
	pub fn start(&mut self) -> SoundIoResult<()> {
		(LibSoundIo::get().soundio_instream_start)(self).into_result()
	}

	/// If the underyling device supports pausing, this pauses the stream and
	/// prevents SoundIoInStream::read_callback from being called. Otherwise this returns
	/// #SoundIoErrorIncompatibleDevice.
	/// This function may be called from any thread.
	/// Pausing when already paused or unpausing when already unpaused has no
	/// effect and always returns #SoundIoErrorNone.
	///
	/// # Possible errors:
	/// * SoundIoError::BackendDisconnected
	/// * SoundIoError::Streaming
	/// * SoundIoError::IncompatibleDevice - device does not support pausing/unpausing
	pub fn pause(&mut self, pause: bool) -> SoundIoResult<()> {
		(LibSoundIo::get().soundio_instream_pause)(self, pause).into_result()
	}

	/// Obtain the number of seconds that the next frame of sound being
	/// captured will take to arrive in the buffer, plus the amount of time that is
	/// represented in the buffer. This includes both software and hardware latency.
	///
	/// This function must be called only from within SoundIoInStream::read_callback.
	///
	/// # Possible errors:
	/// * SoundIoError::Streaming
	pub fn get_latency(&mut self) -> SoundIoResult<f64> {
		let mut latency = 0f64;
		(LibSoundIo::get().soundio_instream_get_latency)(self, &mut latency).into_result()
			.map(|_| latency)
	}

	#[allow(clippy::type_complexity)]
	pub fn begin_write(&mut self, frame_count: usize) -> SoundIoResult<(usize, [(usize, &[u8]); SOUNDIO_MAX_CHANNELS])> {
		let mut channels = [SoundIoChannelArea { ptr: std::ptr::null_mut(), step: 0 }; SOUNDIO_MAX_CHANNELS];
		let mut pp = [std::ptr::null_mut::<SoundIoChannelArea>(); SOUNDIO_MAX_CHANNELS];

		for i in 0..SOUNDIO_MAX_CHANNELS {
			pp[i] = &mut channels[i];
		}

		let mut frame_count = frame_count as u32;
		(LibSoundIo::get().soundio_instream_begin_read)(self, pp.as_mut_ptr(), &mut frame_count)?;

		let mut channels: [(usize, &[u8]); 24] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

		for i in 0..SOUNDIO_MAX_CHANNELS {
			unsafe { channels[i] = ((*pp[i]).step as usize, std::slice::from_raw_parts(
				(*pp[i]).ptr, ((*pp[i]).step * frame_count) as _)); }
		}

		Ok((frame_count as _, channels))
	}

	pub fn end_write(&mut self) -> SoundIoResult<()> {
		(LibSoundIo::get().soundio_instream_end_read)(self).into_result()
	}
}

impl Drop for SoundIoInStream {
	fn drop(&mut self) {
		(LibSoundIo::get().soundio_instream_destroy)(self)
	}
}

/// A ring buffer is a single-reader single-writer lock-free fixed-size queue.
/// libsoundio ring buffers use memory mapping techniques to enable a
/// contiguous buffer when reading or writing across the boundary of the ring
/// buffer's capacity.
#[derive(Debug)]
pub struct SoundIoRingBuffer;

impl SoundIoRingBuffer {
	pub fn new(soundio: &mut SoundIo, len: usize) -> Box<Self, SoundIoAlloc> {
		unsafe { box_from_raw((LibSoundIo::get().soundio_ring_buffer_create)(soundio, len as _))
			.expect("failed to create `SoundIoRingBuffer`, out of memory") }
	}

	/// When you create a ring buffer, capacity might be more than the requested
	/// capacity for alignment purposes. This function returns the actual capacity.
	pub fn capacity(&mut self) -> usize {
		(LibSoundIo::get().soundio_ring_buffer_capacity)(self) as _
	}

	pub fn clear(&mut self) {
		(LibSoundIo::get().soundio_ring_buffer_clear)(self)
	}
}

impl io::Read for SoundIoRingBuffer {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		unsafe {
			let len = buf.len().min((LibSoundIo::get().soundio_ring_buffer_fill_count)(self) as _);
			(LibSoundIo::get().soundio_ring_buffer_read_ptr)(self)
				.copy_to_nonoverlapping(buf.as_mut_ptr(), len);
			(LibSoundIo::get().soundio_ring_buffer_advance_read_ptr)(self, len as _);
			Ok(len)
		}
	}
}

impl io::Write for SoundIoRingBuffer {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		unsafe {
			let len = buf.len().min((LibSoundIo::get().soundio_ring_buffer_free_count)(self) as _);
			(LibSoundIo::get().soundio_ring_buffer_write_ptr)(self)
				.copy_from_nonoverlapping(buf.as_ptr(), len);
			(LibSoundIo::get().soundio_ring_buffer_advance_write_ptr)(self, len as _);
			Ok(len)
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

impl Drop for SoundIoRingBuffer {
	fn drop(&mut self) {
		(LibSoundIo::get().soundio_ring_buffer_destroy)(self)
	}
}

static mut LIB_SOUNDIO: LibSoundIo = unsafe { LibSoundIo::uninit() };

struct LibSoundIo {
	lib:                                   Option<libloading::Library>,
	soundio_version_string:                extern "C" fn() -> *mut u8,
	soundio_version_major:                 extern "C" fn() -> u32,
	soundio_version_minor:                 extern "C" fn() -> u32,
	soundio_version_patch:                 extern "C" fn() -> u32,
	soundio_strerror:                      extern "C" fn(u32) -> *const u8,
	soundio_backend_get_name:              extern "C" fn(SoundIoBackend) -> *const u8,
	soundio_backend_have:                  extern "C" fn(SoundIoBackend) -> bool,
	soundio_create:                        extern "C" fn() -> *mut SoundIo,
	soundio_destroy:                       extern "C" fn(*mut SoundIo),
	soundio_connect:                       extern "C" fn(*mut SoundIo) -> SoundIoError,
	soundio_connect_backend:               extern "C" fn(*mut SoundIo, SoundIoBackend) -> SoundIoError,
	soundio_disconnect:                    extern "C" fn(*mut SoundIo),
	soundio_backend_count:                 extern "C" fn(*mut SoundIo) -> u32,
	soundio_get_backend:                   extern "C" fn(*mut SoundIo, u32) -> SoundIoBackend,
	soundio_flush_events:                  extern "C" fn(*mut SoundIo),
	soundio_wait_events:                   extern "C" fn(*mut SoundIo),
	soundio_wakeup:                        extern "C" fn(*mut SoundIo),
	soundio_force_device_scan:             extern "C" fn(*mut SoundIo),
	soundio_input_device_count:            extern "C" fn(*mut SoundIo) -> u32,
	soundio_output_device_count:           extern "C" fn(*mut SoundIo) -> u32,
	soundio_get_input_device:              extern "C" fn(*mut SoundIo, u32) -> *mut SoundIoDevice,
	soundio_get_output_device:             extern "C" fn(*mut SoundIo, u32) -> *mut SoundIoDevice,
	soundio_default_input_device_index:    extern "C" fn(*mut SoundIo) -> u32,
	soundio_default_output_device_index:   extern "C" fn(*mut SoundIo) -> u32,
	soundio_channel_get_name:              extern "C" fn(SoundIoChannelId) -> *const u8,
	soundio_channel_parse_id:              extern "C" fn(*const u8, u32) -> SoundIoChannelId,
	soundio_channel_layout_equal:          extern "C" fn(*const SoundIoChannelLayout, *const SoundIoChannelLayout) -> bool,
	soundio_channel_layout_builtin_count:  extern "C" fn() -> u32,
	soundio_channel_layout_get_builtin:    extern "C" fn(u32) -> *const SoundIoChannelLayout,
	soundio_channel_layout_get_default:    extern "C" fn(u32) -> *const SoundIoChannelLayout,
	soundio_channel_layout_find_channel:   extern "C" fn(*const SoundIoChannelLayout, SoundIoChannelId) -> u32,
	soundio_channel_layout_detect_builtin: extern "C" fn(*mut SoundIoChannelLayout) -> bool,
	soundio_channel_layout_best_matching:  extern "C" fn(*const SoundIoChannelLayout, u32, *const SoundIoChannelLayout, u32) -> *const SoundIoChannelLayout,
	soundio_channel_layouts_sort:          extern "C" fn(*mut SoundIoChannelLayout, u32),
	soundio_format_get_bytes_per_sample:   extern "C" fn(SoundIoFormat) -> u32,
	soundio_format_string:                 extern "C" fn(SoundIoFormat) -> *const u8,
	soundio_device_ref:                    extern "C" fn(*mut SoundIoDevice),
	soundio_device_unref:                  extern "C" fn(*mut SoundIoDevice),
	soundio_device_equal:                  extern "C" fn(*const SoundIoDevice, *const SoundIoDevice) -> bool,
	soundio_device_sort_channel_layouts:   extern "C" fn(*mut SoundIoDevice),
	soundio_device_supports_format:        extern "C" fn(*mut SoundIoDevice, SoundIoFormat) -> bool,
	soundio_device_supports_layout:        extern "C" fn(*mut SoundIoDevice, *const  SoundIoChannelLayout) -> bool,
	soundio_device_supports_sample_rate:   extern "C" fn(*mut SoundIoDevice, u32) -> bool,
	soundio_device_nearest_sample_rate:    extern "C" fn(*mut SoundIoDevice, u32) -> u32,
	soundio_outstream_create:              extern "C" fn(*mut SoundIoDevice) -> *mut SoundIoOutStream,
	soundio_outstream_destroy:             extern "C" fn(*mut SoundIoOutStream),
	soundio_outstream_open:                extern "C" fn(*mut SoundIoOutStream) -> SoundIoError,
	soundio_outstream_start:               extern "C" fn(*mut SoundIoOutStream) -> SoundIoError,
	soundio_outstream_begin_write:         extern "C" fn(*mut SoundIoOutStream, *mut *mut SoundIoChannelArea, *mut u32) -> SoundIoError,
	soundio_outstream_end_write:           extern "C" fn(*mut SoundIoOutStream) -> SoundIoError,
	soundio_outstream_clear_buffer:        extern "C" fn(*mut SoundIoOutStream) -> SoundIoError,
	soundio_outstream_pause:               extern "C" fn(*mut SoundIoOutStream, bool) -> SoundIoError,
	soundio_outstream_get_latency:         extern "C" fn(*mut SoundIoOutStream, *mut f64) -> SoundIoError,
	soundio_outstream_set_volume:          extern "C" fn(*mut SoundIoOutStream, f64) -> SoundIoError,
	soundio_instream_create:               extern "C" fn(*mut SoundIoDevice) -> *mut SoundIoInStream,
	soundio_instream_destroy:              extern "C" fn(*mut SoundIoInStream),
	soundio_instream_open:                 extern "C" fn(*mut SoundIoInStream) -> SoundIoError,
	soundio_instream_start:                extern "C" fn(*mut SoundIoInStream) -> SoundIoError,
	soundio_instream_begin_read:           extern "C" fn(*mut SoundIoInStream, *mut *mut SoundIoChannelArea, *mut u32) -> SoundIoError,
	soundio_instream_end_read:             extern "C" fn(*mut SoundIoInStream) -> SoundIoError,
	soundio_instream_pause:                extern "C" fn(*mut SoundIoInStream, bool) -> SoundIoError,
	soundio_instream_get_latency:          extern "C" fn(*mut SoundIoInStream, *mut f64) -> SoundIoError,
	soundio_ring_buffer_create:            extern "C" fn(*mut SoundIo, u32) -> *mut SoundIoRingBuffer,
	soundio_ring_buffer_destroy:           extern "C" fn(*mut SoundIoRingBuffer),
	soundio_ring_buffer_capacity:          extern "C" fn(*mut SoundIoRingBuffer) -> u32,
	soundio_ring_buffer_write_ptr:         extern "C" fn(*mut SoundIoRingBuffer) -> *mut u8,
	soundio_ring_buffer_advance_write_ptr: extern "C" fn(*mut SoundIoRingBuffer, count: u32),
	soundio_ring_buffer_read_ptr:          extern "C" fn(*mut SoundIoRingBuffer) -> *mut u8,
	soundio_ring_buffer_advance_read_ptr:  extern "C" fn(*mut SoundIoRingBuffer, count: u32),
	soundio_ring_buffer_fill_count:        extern "C" fn(*mut SoundIoRingBuffer) -> u32,
	soundio_ring_buffer_free_count:        extern "C" fn(*mut SoundIoRingBuffer) -> u32,
	soundio_ring_buffer_clear:             extern "C" fn(*mut SoundIoRingBuffer)
}

impl LibSoundIo {
	const unsafe fn uninit() -> Self {
		extern fn abort() { panic!("libsoundio has not been loaded") }

		Self {
			lib:                                   None,
			soundio_version_string:                std::mem::transmute(abort as extern fn()),
			soundio_version_major:                 std::mem::transmute(abort as extern fn()),
			soundio_version_minor:                 std::mem::transmute(abort as extern fn()),
			soundio_version_patch:                 std::mem::transmute(abort as extern fn()),
			soundio_strerror:                      std::mem::transmute(abort as extern fn()),
			soundio_backend_get_name:              std::mem::transmute(abort as extern fn()),
			soundio_backend_have:                  std::mem::transmute(abort as extern fn()),
			soundio_create:                        std::mem::transmute(abort as extern fn()),
			soundio_destroy:                       std::mem::transmute(abort as extern fn()),
			soundio_connect:                       std::mem::transmute(abort as extern fn()),
			soundio_connect_backend:               std::mem::transmute(abort as extern fn()),
			soundio_disconnect:                    std::mem::transmute(abort as extern fn()),
			soundio_backend_count:                 std::mem::transmute(abort as extern fn()),
			soundio_get_backend:                   std::mem::transmute(abort as extern fn()),
			soundio_flush_events:                  std::mem::transmute(abort as extern fn()),
			soundio_wait_events:                   std::mem::transmute(abort as extern fn()),
			soundio_wakeup:                        std::mem::transmute(abort as extern fn()),
			soundio_force_device_scan:             std::mem::transmute(abort as extern fn()),
			soundio_input_device_count:            std::mem::transmute(abort as extern fn()),
			soundio_output_device_count:           std::mem::transmute(abort as extern fn()),
			soundio_get_input_device:              std::mem::transmute(abort as extern fn()),
			soundio_get_output_device:             std::mem::transmute(abort as extern fn()),
			soundio_default_input_device_index:    std::mem::transmute(abort as extern fn()),
			soundio_default_output_device_index:   std::mem::transmute(abort as extern fn()),
			soundio_channel_get_name:              std::mem::transmute(abort as extern fn()),
			soundio_channel_parse_id:              std::mem::transmute(abort as extern fn()),
			soundio_channel_layout_equal:          std::mem::transmute(abort as extern fn()),
			soundio_channel_layout_builtin_count:  std::mem::transmute(abort as extern fn()),
			soundio_channel_layout_get_builtin:    std::mem::transmute(abort as extern fn()),
			soundio_channel_layout_get_default:    std::mem::transmute(abort as extern fn()),
			soundio_channel_layout_find_channel:   std::mem::transmute(abort as extern fn()),
			soundio_channel_layout_detect_builtin: std::mem::transmute(abort as extern fn()),
			soundio_channel_layout_best_matching:  std::mem::transmute(abort as extern fn()),
			soundio_channel_layouts_sort:          std::mem::transmute(abort as extern fn()),
			soundio_format_get_bytes_per_sample:   std::mem::transmute(abort as extern fn()),
			soundio_format_string:                 std::mem::transmute(abort as extern fn()),
			soundio_device_ref:                    std::mem::transmute(abort as extern fn()),
			soundio_device_unref:                  std::mem::transmute(abort as extern fn()),
			soundio_device_equal:                  std::mem::transmute(abort as extern fn()),
			soundio_device_sort_channel_layouts:   std::mem::transmute(abort as extern fn()),
			soundio_device_supports_format:        std::mem::transmute(abort as extern fn()),
			soundio_device_supports_layout:        std::mem::transmute(abort as extern fn()),
			soundio_device_supports_sample_rate:   std::mem::transmute(abort as extern fn()),
			soundio_device_nearest_sample_rate:    std::mem::transmute(abort as extern fn()),
			soundio_outstream_create:              std::mem::transmute(abort as extern fn()),
			soundio_outstream_destroy:             std::mem::transmute(abort as extern fn()),
			soundio_outstream_open:                std::mem::transmute(abort as extern fn()),
			soundio_outstream_start:               std::mem::transmute(abort as extern fn()),
			soundio_outstream_begin_write:         std::mem::transmute(abort as extern fn()),
			soundio_outstream_end_write:           std::mem::transmute(abort as extern fn()),
			soundio_outstream_clear_buffer:        std::mem::transmute(abort as extern fn()),
			soundio_outstream_pause:               std::mem::transmute(abort as extern fn()),
			soundio_outstream_get_latency:         std::mem::transmute(abort as extern fn()),
			soundio_outstream_set_volume:          std::mem::transmute(abort as extern fn()),
			soundio_instream_create:               std::mem::transmute(abort as extern fn()),
			soundio_instream_destroy:              std::mem::transmute(abort as extern fn()),
			soundio_instream_open:                 std::mem::transmute(abort as extern fn()),
			soundio_instream_start:                std::mem::transmute(abort as extern fn()),
			soundio_instream_begin_read:           std::mem::transmute(abort as extern fn()),
			soundio_instream_end_read:             std::mem::transmute(abort as extern fn()),
			soundio_instream_pause:                std::mem::transmute(abort as extern fn()),
			soundio_instream_get_latency:          std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_create:            std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_destroy:           std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_capacity:          std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_write_ptr:         std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_advance_write_ptr: std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_read_ptr:          std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_advance_read_ptr:  std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_fill_count:        std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_free_count:        std::mem::transmute(abort as extern fn()),
			soundio_ring_buffer_clear:             std::mem::transmute(abort as extern fn()),
		}
	}

	unsafe fn load(&mut self) {
		if self.lib.is_some() { return; }

		let lib                                    = libloading::Library::new("libsoundio.so.2").expect("failed to load libsoundio");
		self.soundio_version_string                = *lib.get(b"soundio_version_string\0").expect("failed to load `soundio_version_string`");
		self.soundio_version_major                 = *lib.get(b"soundio_version_major\0").expect("failed to load `soundio_version_major`");
		self.soundio_version_minor                 = *lib.get(b"soundio_version_minor\0").expect("failed to load `soundio_version_minor`");
		self.soundio_version_patch                 = *lib.get(b"soundio_version_patch\0").expect("failed to load `soundio_version_patch`");
		self.soundio_strerror                      = *lib.get(b"soundio_strerror\0").expect("failed to load `soundio_strerror`");
		self.soundio_backend_get_name              = *lib.get(b"soundio_backend_name\0").expect("failed to load `soundio_backend_name`");
		self.soundio_backend_have                  = *lib.get(b"soundio_have_backend\0").expect("failed to load `soundio_have_backend`");
		self.soundio_create                        = *lib.get(b"soundio_create\0").expect("failed to load `soundio_create`");
		self.soundio_destroy                       = *lib.get(b"soundio_destroy\0").expect("failed to load `soundio_destroy`");
		self.soundio_connect                       = *lib.get(b"soundio_connect\0").expect("failed to load `soundio_connect`");
		self.soundio_connect_backend               = *lib.get(b"soundio_connect_backend\0").expect("failed to load `soundio_connect_backend`");
		self.soundio_disconnect                    = *lib.get(b"soundio_disconnect\0").expect("failed to load `soundio_disconnect`");
		self.soundio_backend_count                 = *lib.get(b"soundio_backend_count\0").expect("failed to load `soundio_backend_count`");
		self.soundio_get_backend                   = *lib.get(b"soundio_get_backend\0").expect("failed to load `soundio_get_backend`");
		self.soundio_flush_events                  = *lib.get(b"soundio_flush_events\0").expect("failed to load `soundio_flush_events`");
		self.soundio_wait_events                   = *lib.get(b"soundio_wait_events\0").expect("failed to load `soundio_wait_events`");
		self.soundio_wakeup                        = *lib.get(b"soundio_wakeup\0").expect("failed to load `soundio_wakeup`");
		self.soundio_force_device_scan             = *lib.get(b"soundio_force_device_scan\0").expect("failed to load `soundio_force_device_scan`");
		self.soundio_input_device_count            = *lib.get(b"soundio_input_device_count\0").expect("failed to load `soundio_input_device_count`");
		self.soundio_output_device_count           = *lib.get(b"soundio_output_device_count\0").expect("failed to load `soundio_output_device_count`");
		self.soundio_get_input_device              = *lib.get(b"soundio_get_input_device\0").expect("failed to load `soundio_get_input_device`");
		self.soundio_get_output_device             = *lib.get(b"soundio_get_output_device\0").expect("failed to load `soundio_get_output_device`");
		self.soundio_default_input_device_index    = *lib.get(b"soundio_default_input_device_index\0").expect("failed to load `soundio_default_input_device_index`");
		self.soundio_default_output_device_index   = *lib.get(b"soundio_default_output_device_index\0").expect("failed to load `soundio_default_output_device_index`");
		self.soundio_channel_get_name              = *lib.get(b"soundio_get_channel_name\0").expect("failed to load `soundio_get_channel_name`");
		self.soundio_channel_parse_id              = *lib.get(b"soundio_parse_channel_id\0").expect("failed to load `soundio_parse_channel_id`");
		self.soundio_channel_layout_equal          = *lib.get(b"soundio_channel_layout_equal\0").expect("failed to load `soundio_channel_layout_equal`");
		self.soundio_channel_layout_builtin_count  = *lib.get(b"soundio_channel_layout_builtin_count\0").expect("failed to load `soundio_channel_layout_builtin_count`");
		self.soundio_channel_layout_get_builtin    = *lib.get(b"soundio_channel_layout_get_builtin\0").expect("failed to load `soundio_channel_layout_get_builtin`");
		self.soundio_channel_layout_get_default    = *lib.get(b"soundio_channel_layout_get_default\0").expect("failed to load `soundio_channel_layout_get_default`");
		self.soundio_channel_layout_find_channel   = *lib.get(b"soundio_channel_layout_find_channel\0").expect("failed to load `soundio_channel_layout_find_channel`");
		self.soundio_channel_layout_detect_builtin = *lib.get(b"soundio_channel_layout_detect_builtin\0").expect("failed to load `soundio_channel_layout_detect_builtin`");
		self.soundio_channel_layout_best_matching  = *lib.get(b"soundio_best_matching_channel_layout\0").expect("failed to load `soundio_best_matching_channel_layout`");
		self.soundio_channel_layouts_sort          = *lib.get(b"soundio_sort_channel_layouts\0").expect("failed to load `soundio_sort_channel_layouts`");
		self.soundio_format_get_bytes_per_sample   = *lib.get(b"soundio_get_bytes_per_sample\0").expect("failed to load `soundio_get_bytes_per_sample`");
		self.soundio_format_string                 = *lib.get(b"soundio_format_string\0").expect("failed to load `soundio_format_string`");
		self.soundio_device_ref                    = *lib.get(b"soundio_device_ref\0").expect("failed to load `soundio_device_ref`");
		self.soundio_device_unref                  = *lib.get(b"soundio_device_unref\0").expect("failed to load `soundio_device_unref`");
		self.soundio_device_equal                  = *lib.get(b"soundio_device_equal\0").expect("failed to load `soundio_device_equal`");
		self.soundio_device_sort_channel_layouts   = *lib.get(b"soundio_device_sort_channel_layouts\0").expect("failed to load `soundio_device_sort_channel_layouts`");
		self.soundio_device_supports_format        = *lib.get(b"soundio_device_supports_format\0").expect("failed to load `soundio_device_supports_format`");
		self.soundio_device_supports_layout        = *lib.get(b"soundio_device_supports_layout\0").expect("failed to load `soundio_device_supports_layout`");
		self.soundio_device_supports_sample_rate   = *lib.get(b"soundio_device_supports_sample_rate\0").expect("failed to load `soundio_device_supports_sample_rate`");
		self.soundio_device_nearest_sample_rate    = *lib.get(b"soundio_device_nearest_sample_rate\0").expect("failed to load `soundio_device_nearest_sample_rate`");
		self.soundio_outstream_create              = *lib.get(b"soundio_outstream_create\0").expect("failed to load `soundio_outstream_create`");
		self.soundio_outstream_destroy             = *lib.get(b"soundio_outstream_destroy\0").expect("failed to load `soundio_outstream_destroy`");
		self.soundio_outstream_open                = *lib.get(b"soundio_outstream_open\0").expect("failed to load `soundio_outstream_open`");
		self.soundio_outstream_start               = *lib.get(b"soundio_outstream_start\0").expect("failed to load `soundio_outstream_start`");
		self.soundio_outstream_begin_write         = *lib.get(b"soundio_outstream_begin_write\0").expect("failed to load `soundio_outstream_begin_write`");
		self.soundio_outstream_end_write           = *lib.get(b"soundio_outstream_end_write\0").expect("failed to load `soundio_outstream_end_write`");
		self.soundio_outstream_clear_buffer        = *lib.get(b"soundio_outstream_clear_buffer\0").expect("failed to load `soundio_outstream_clear_buffer`");
		self.soundio_outstream_pause               = *lib.get(b"soundio_outstream_pause\0").expect("failed to load `soundio_outstream_pause`");
		self.soundio_outstream_get_latency         = *lib.get(b"soundio_outstream_get_latency\0").expect("failed to load `soundio_outstream_get_latency`");
		self.soundio_outstream_set_volume          = *lib.get(b"soundio_outstream_set_volume\0").expect("failed to load `soundio_outstream_set_volume`");
		self.soundio_instream_create               = *lib.get(b"soundio_instream_create\0").expect("failed to load `soundio_instream_create`");
		self.soundio_instream_destroy              = *lib.get(b"soundio_instream_destroy\0").expect("failed to load `soundio_instream_destroy`");
		self.soundio_instream_open                 = *lib.get(b"soundio_instream_open\0").expect("failed to load `soundio_instream_open`");
		self.soundio_instream_start                = *lib.get(b"soundio_instream_start\0").expect("failed to load `soundio_instream_start`");
		self.soundio_instream_begin_read           = *lib.get(b"soundio_instream_begin_read\0").expect("failed to load `soundio_instream_begin_read`");
		self.soundio_instream_end_read             = *lib.get(b"soundio_instream_end_read\0").expect("failed to load `soundio_instream_end_read`");
		self.soundio_instream_pause                = *lib.get(b"soundio_instream_pause\0").expect("failed to load `soundio_instream_pause`");
		self.soundio_instream_get_latency          = *lib.get(b"soundio_instream_get_latency\0").expect("failed to load `soundio_instream_get_latency`");
		self.soundio_ring_buffer_create            = *lib.get(b"soundio_ring_buffer_create\0").expect("failed to load `soundio_ring_buffer_create`");
		self.soundio_ring_buffer_destroy           = *lib.get(b"soundio_ring_buffer_destroy\0").expect("failed to load `soundio_ring_buffer_destroy`");
		self.soundio_ring_buffer_capacity          = *lib.get(b"soundio_ring_buffer_capacity\0").expect("failed to load `soundio_ring_buffer_capacity`");
		self.soundio_ring_buffer_write_ptr         = *lib.get(b"soundio_ring_buffer_write_ptr\0").expect("failed to load `soundio_ring_buffer_write_ptr`");
		self.soundio_ring_buffer_advance_write_ptr = *lib.get(b"soundio_ring_buffer_advance_write_ptr\0").expect("failed to load `soundio_ring_buffer_advance_write_ptr`");
		self.soundio_ring_buffer_read_ptr          = *lib.get(b"soundio_ring_buffer_read_ptr\0").expect("failed to load `soundio_ring_buffer_read_ptr`");
		self.soundio_ring_buffer_advance_read_ptr  = *lib.get(b"soundio_ring_buffer_advance_read_ptr\0").expect("failed to load `soundio_ring_buffer_advance_read_ptr`");
		self.soundio_ring_buffer_fill_count        = *lib.get(b"soundio_ring_buffer_fill_count\0").expect("failed to load `soundio_ring_buffer_fill_count`");
		self.soundio_ring_buffer_free_count        = *lib.get(b"soundio_ring_buffer_free_count\0").expect("failed to load `soundio_ring_buffer_free_count`");
		self.soundio_ring_buffer_clear             = *lib.get(b"soundio_ring_buffer_clear\0").expect("failed to load `soundio_ring_buffer_clear`");
		self.lib                                   = Some(lib);
		log::trace!("loaded libsoundio");
	}

	fn get() -> &'static Self {
		unsafe { &LIB_SOUNDIO }
	}
}

pub fn soundio_get_bytes_per_frame(format: SoundIoFormat, channel_count: usize) -> usize {
	format.get_bytes_per_sample() * channel_count
}

pub fn soundio_get_bytes_per_second(format: SoundIoFormat, channel_count: usize, sample_rate: usize) -> usize {
	soundio_get_bytes_per_frame(format, channel_count) * sample_rate
}

pub struct SoundIoAlloc;

unsafe impl std::alloc::Allocator for SoundIoAlloc {
	fn allocate(&self, _: std::alloc::Layout) -> std::result::Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
		log::error!("attempted alloc on noop allocator");
		Err(std::alloc::AllocError)
	}

	unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
		log::trace!("noop dealloc: {:?} with layout {:?}", ptr.as_ptr(), layout);
	}
}

unsafe fn box_from_raw<T: ?Sized>(ptr: *mut T) -> Option<Box<T, SoundIoAlloc>> {
	if ptr.is_null() {
		None
	} else {
		Some(Box::from_raw_in(ptr, SoundIoAlloc))
	}
}

/// # Safety
///
/// The passed pointer must be valid, otherwise calling this function is UB.
pub unsafe fn str_convert<'a>(ptr: *const u8) -> &'a str {
	if ptr.is_null() { panic!("pointer was null") }
	std::ffi::CStr::from_ptr(ptr as _).to_str().unwrap()
}