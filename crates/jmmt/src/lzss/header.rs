use std::mem::size_of;
use crate::format::Validatable;

#[repr(C)]
#[derive(Debug, Default)]
pub struct LzssHeader {
	pub next: u32, // ps2 ptr. usually 0 cause theres no next header
	pub byte_id: u8,
	pub header_size: u8, // matches size_of::<LzssHeader>()
	pub max_match: u8,
	pub fill_byte: u8,
	pub ring_size: u16,
	pub error_id: u16,
	pub uncompressed_bytes: u32,
	pub compressed_bytes: u32,
	pub crc_hash: u32,
	pub file_id: u32,
	pub compressed_data_crc: u32,
}

impl Validatable for LzssHeader {
	/// Validate this LzssHeader object.
	/// This checks if:
	/// - the "magic" byte is correct (0x91)
	/// - the [LzssHeader::header_size] member is correct
	fn valid(&self) -> bool {
		self.byte_id == 0x91 && self.header_size as usize == size_of::<LzssHeader>()
	}
}
