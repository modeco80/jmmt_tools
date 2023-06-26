//! .ps2_texture structures

use super::Validatable;

/// .ps2_texture header.
#[repr(C)]
#[derive(Debug, Default)]
pub struct Ps2TextureHeader {
	pub magic: u32,
	pub unk: u32,
	pub unk2: u16,
	pub width: u16,
	pub height: u16,

	/// bits-per-pixel of the texture data. Anything above 8
	/// will not have an associated .ps2_palette file,
	/// since the texture data will not be indirect color.
	pub bpp: u16,

	/// Data start offset.
	pub data_start_offset: u32,

	/// Possibly the size of this header.
	pub header_end_offset: u32,

	pub unk6: [u32; 8], // mostly unrelated values, this is probably padding space for the game code to put stuff
}

impl Ps2TextureHeader {
	/// 'TEX1'
	pub const VALID_FOURCC: u32 = 0x31584554;

	fn has_palette(&self) -> bool {
		// if the BPP is less than or equal to 8 (I've only seen 8bpp and 16bpp),
		// then the texture will be palettized.
		self.bpp >= 8
	}
}

impl Validatable for Ps2TextureHeader {
	fn valid(&self) -> bool {
		self.magic == Self::VALID_FOURCC && self.header_end_offset == 0x38
	}
}
