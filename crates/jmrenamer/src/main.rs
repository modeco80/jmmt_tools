//! A reimplementation of jmmt_renamer in Rust.
//! This program should be run in the root directory
//! of an extracted (from image) copy of the game.

use jmmt::hash::filename::*;
use jmmt::read::package_toc::read_package_toc;
use std::{fs, path::Path};

use clap::{Subcommand, Parser};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Rename the files in the DATA directory to the original filenames.
	Clear,

	/// Rename the files to the .DAT filenames that the game expects.
	Unclear,
}

fn do_clear() {
	let package_toc_filename = format!("DATA/{}", dat_filename("package.toc"));

	match read_package_toc(Path::new(package_toc_filename.as_str()).to_path_buf()) {
		Ok(toc) => {
			for toc_entry in toc {
				let dat_src = format!(
					"DATA/{}",
					dat_filename_from_hash(toc_entry.file_name_hash())
				);
				let src_path = Path::new(dat_src.as_str());
				let dat_clearname = format!(
					"DATA/{}",
					toc_entry
						.file_name()
						.expect("How did invalid ASCII get here?")
				);
				let dest_path = Path::new(dat_clearname.as_str());

				if src_path.exists() {
					match fs::rename(src_path, dest_path) {
						Ok(_) => {}
						Err(error) => {
							println!("Error renaming {}: {}", src_path.display(), error);
							return ();
						}
					};
					println!("Clearnamed {} -> {}", src_path.display(), dest_path.display());
				}
			}

			match fs::rename(
				Path::new(package_toc_filename.as_str()),
				Path::new("DATA/package.toc"),
			) {
				Ok(_) => {}
				Err(error) => {
					println!("Error renaming TOC file: {}", error);
					return ();
				}
			};
		}

		Err(error) => {
			println!("Error reading package.toc file: {}", error);
			return ();
		}
	}
}

fn do_unclear() {
	let package_toc_filename = String::from("DATA/package.toc");

	match read_package_toc(Path::new(package_toc_filename.as_str()).to_path_buf()) {
		Ok(toc) => {
			for toc_entry in toc {

				let dat_clearname = format!(
					"DATA/{}",
					toc_entry
						.file_name()
						.expect("How did invalid ASCII get here?")
				);
				let src_path = Path::new(dat_clearname.as_str());


				let dat_dest = format!(
					"DATA/{}",
					dat_filename_from_hash(toc_entry.file_name_hash())
				);
				
				let dest_path = Path::new(dat_dest.as_str());

				if src_path.exists() {
					match fs::rename(src_path, dest_path) {
						Ok(_) => {}
						Err(error) => {
							println!("Error renaming {}: {}", src_path.display(), error);
							return ();
						}
					};
					println!("Uncleared {} -> {}", src_path.display(), dest_path.display());
				}
			}

			let package_toc_dat_filename = format!("DATA/{}", dat_filename("package.toc"));
			match fs::rename(
				Path::new("DATA/package.toc"),
				Path::new(package_toc_dat_filename.as_str()),
			) {
				Ok(_) => {}
				Err(error) => {
					println!("Error renaming TOC file: {}", error);
					return ();
				}
			};
		}

		Err(error) => {
			println!("Error reading package.toc file: {}", error);
			return ();
		}
	}
}

fn main() {
	// A relatively simple idiot-check. Later on utilities might have a shared library
	// of code which validates game root stuff and can open it up/etc.
	if !Path::new("DATA").is_dir() {
		println!("This program should be run in the root of an extracted copy.");
		return ();
	}

	let cli = Cli::parse();


	match &cli.command {
		Commands::Clear =>
			do_clear(),
		Commands::Unclear => 
			do_unclear()
	}
}
