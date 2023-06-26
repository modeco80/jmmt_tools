//! .ps2_palette structures

use super::Validatable;

/// .ps2_palette header
#[repr(C)]
#[derive(Debug, Default)]
pub struct Ps2PaletteHeader {
	pub fourcc: u32,
	pub unk: u32,
	pub unk2: u16,

	pub color_count: u16,
	pub palette_bpp: u16,
	pub unk3: u16,

	pub data_start: u32,
	pub header_size: u32,

	pub pad: [u32; 6] // reserved for game code, like .ps2_texture?
}

impl Ps2PaletteHeader {
	/// 'PAL1'
	pub const VALID_FOURCC : u32 = 0x314c4150;
}

impl Validatable for Ps2PaletteHeader {
	fn valid(&self) -> bool {
		self.fourcc == Self::VALID_FOURCC && self.header_size == 0x18
	}
}
