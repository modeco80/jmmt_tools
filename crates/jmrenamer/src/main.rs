//! A reimplemntation of jmmt_renamer in Rust.
//! This program should be run in the root directory
//! of an extracted (from image) copy of the game.

use std::{fs, io, path::Path};
use jmmt::hash::hash_string;

const FILENAME_TABLE : [&str; 20] = [
		// First loaded by the game
		"package.toc",

		// General packs
		"config.pak",
	
		// This file is referenced in the game files,
		// but doesn't seem to exist anymore in the final build.
		//"shell.pak",
	
		"shell_character_select.pak",
		"shell_main.pak",
		"shell_title.pak",
		"shell_venue.pak",
		"shell_event.pak",
		"shell_option.pak",
		"win_screens.pak",
	
		// Game levels
		"SF_san_fran.pak",
		"DC_washington.pak",
		"MK_MT_KILI.pak",
		"MP_MACHU_PIHU.pak",
		"LV_Las_Vegas.pak",
		"AN_ANTARTICA.pak",
		"NP_Nepal.pak",
		"TH_TAHOE.pak",
		"VA_Valdez_alaska.pak",
		"RV_Rome.pak",
		"TR_training.pak"
];

/// Make a .DAT filename from a cleartext filename.
/// 
/// .DAT and .MET filenames are formatted like "[hex char * 8].DAT"
/// The name component is the CRC32 of the original filename.
///
/// The DAT/MET filename can be a max of 13 characters long.
fn hashed_dat_filename(filename: &str) -> String {
	format!("{:X}.DAT", hash_string(String::from(filename)))
}

fn main() -> io::Result<()> {
	
	// A relatively simple idiot-check.
	if !Path::new("DATA").is_dir() {
		println!("This program should be run in the root of an extracted copy.");
		std::process::exit(1);
	}
	
	for clearname in FILENAME_TABLE.iter() {
		let dat_filename = hashed_dat_filename(clearname);
		let dat_src = format!("DATA/{}", dat_filename);
		let dat_clearname = format!("DATA/{}", String::from(*clearname));

		let src_path = Path::new(dat_src.as_str());
		let dest_path = Path::new(dat_clearname.as_str());

		if src_path.exists() {
			fs::rename(src_path, dest_path)?;
			println!("moved {} -> {}", src_path.display(), dest_path.display());
		}
	}

	Ok(())
}
