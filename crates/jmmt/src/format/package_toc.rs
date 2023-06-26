//! Package.toc structures

use super::make_c_string;

/// An entry inside the `package.toc` file
#[derive(Debug)]
#[repr(C)]
pub struct PackageTocEntry {
	/// Package file name.
	file_name: [u8; 0x40],

	file_name_hash: u32,
	toc_start_offset: u32,
	toc_size: u32,
	toc_file_count: u32,
}

impl PackageTocEntry {
	fn file_name(&self) -> Option<String> {
		make_c_string(&self.file_name)
	}

	fn file_name_hash(&self) -> u32 {
		self.file_name_hash
	}

	fn toc_start_offset(&self) -> u32 {
		self.toc_start_offset
	}

	fn toc_size(&self) -> u32 {
		self.toc_size
	}

	fn toc_file_count(&self) -> u32 {
		self.toc_file_count
	}
}
