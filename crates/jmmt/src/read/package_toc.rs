use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

use crate::format::package_toc::*;

use binext::BinaryRead;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("underlying I/O error: {0}")]
	IoError(#[from] std::io::Error),

	#[error("file too small to hold package toc entry")]
	FileTooSmall,

	/// Under-read of data
	#[error("underread")]
	ReadTooSmall,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn read_package_toc(path: PathBuf) -> Result<Vec<PackageTocEntry>> {
	let mut toc_file = File::open(path.clone())?;

	let file_size = toc_file.seek(SeekFrom::End(0))?;
	toc_file.seek(SeekFrom::Start(0))?;

	let vec_size: usize = file_size as usize / std::mem::size_of::<PackageTocEntry>();

	if vec_size == 0 {
		return Err(Error::FileTooSmall);
	}

	let mut vec: Vec<PackageTocEntry> = Vec::with_capacity(vec_size);

	for _ in 0..vec_size {
		vec.push(toc_file.read_binary::<PackageTocEntry>()?);
	}

	drop(toc_file);
	Ok(vec)
}
