//! Utilities for making .DAT and .MET file names
//! 
//! .DAT and .MET filenames are formatted like "{:X}.DAT" in fmt parlance.
//! The name component is the CRC32 of the original filename.
//!
//! The DAT/MET filename can be a max of 13 characters long.

use super::crc32::hash_string;

/// Make a .DAT filename from a cleartext filename.
pub fn dat_filename(filename: &str) -> String {
	format!("{:X}.DAT", hash_string(String::from(filename)))
}

/// Make a .MET filename from a cleartext filename.
pub fn met_filename(filename: &str) -> String {
	format!("{:X}.MET", hash_string(String::from(filename)))
}
