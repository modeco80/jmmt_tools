//! Package file format structures

use super::Validatable;
use crate::lzss::header::LzssHeader;

/// "EOF" header. The QuickBMS script uses this to seek to the PGRP entry.
#[repr(C)]
pub struct PackageEofHeader {
	/// Total size of the header.
	pub header_size: u32,

	/// Size of the debug string table in bytes.
	pub stringtable_size: u32,

	/// Start offset of the [PackageGroup] in the package file.
	pub header_start_offset: u32,
}

/// A Package Group. I have no idea what this is yet
#[repr(C)]
#[derive(Debug, Default)]
pub struct PackageGroup {
	pub fourcc: u32,

	/// Hash of the name of this package group. Hashed with [hash_string](crate::hash::hash_string).
	pub group_name_hash: u32,

	/// File count inside of this group.
	pub group_file_count: u32,

	/// Padding. Set to a fill of 0xCD.
	pub pad: u32,
}

impl PackageGroup {
	/// 'PGRP'
	pub const VALID_FOURCC: u32 = 0x50524750;
}

impl Validatable for PackageGroup {
	fn valid(&self) -> bool {
		self.fourcc == Self::VALID_FOURCC
	}
}

/// A package file chunk.
#[repr(C)]
#[derive(Debug, Default)]
pub struct PackageFileChunk {
	pub fourcc: u32,

	/// Unknown, stays the same per file.
	pub unk: u32,

	/// Unknown, stays the same per file.
	pub unk2: u32,

	/// The current chunk sequence number.
	pub chunk_sequence_number: u16,

	/// The total amount of chunks which make up this file.
	pub chunk_sequence_count: u16,

	/// Hash of file name. Hashed with [hash_string](crate::hash::hash_string),
	/// so it is case-insensitive. However, the debug string table is always in a
	/// particular case, so this doesn't really matter all too much.
	pub file_name_crc: u32,

	/// Unknown data, stays the same per file. Should probably be split out
	/// into seperate u32 fields at some point to figure out what they are.
	pub unk3: [u32; 7],

	/// Uncompressed size of this file chunk's data. Has a maximum of 65535 bytes.
	pub chunk_uncompressed_size: u32,

	/// Where this chunk should start in a larger buffer.
	pub chunk_buffer_offset: u32,

	/// Compressed size of this file chunk's data.
	pub chunk_compressed_size: u32,

	/// Offset in the package file where this chunk's
	/// data starts.
	pub chunk_data_offset: u32,

	/// Uncompressed file size.
	pub file_uncompressed_size: u32,

	/// LZSS header. Only used if the file chunk is compressed.
	pub lzss_header: LzssHeader,
}

impl PackageFileChunk {
	/// 'PFIL'
	pub const VALID_FOURCC: u32 = 0x4C494650;

	pub fn is_compressed(&self) -> bool {
		// If the compressed size matches the uncompressed size
		// then the file chunk is not compressed; likewise, if it does not,
		// then the file chunk is compressed.
		self.chunk_compressed_size != self.chunk_uncompressed_size
	}
}

impl Validatable for PackageFileChunk {
	fn valid(&self) -> bool {
		// Note: Even if it's not used, the LZSS header is initalized
		// to meaningful values, including magic and such.
		self.fourcc == Self::VALID_FOURCC && self.lzss_header.valid()
	}
}
