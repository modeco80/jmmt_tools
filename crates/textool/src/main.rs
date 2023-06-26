use clap::{Parser, Subcommand};

use std::default;
use std::path::Path;
use std::fs::{
	File
};

use jmmt::format::{
	ps2_palette::*,
	ps2_texture::*
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Exports the texture to a .png file
	Export { path: String },

	//Import
}

struct ImageReader {
	file: File,
	header: Ps2TextureHeader,
	pal_header: Ps2PaletteHeader,

	image_data: Vec<u8>,
	palette_data: Vec<u8>
}

impl ImageReader {
	fn new(file: &mut File) -> Self {
		ImageReader {
			file: file.try_clone().unwrap(),
			header: Ps2TextureHeader::default(),
			pal_header: Ps2PaletteHeader::default(),
			image_data: Vec::default(),
			palette_data: Vec::default()
		}
	}

	fn read(&mut self) -> std::io::Result<()> {
		
		Ok(())
	}
}

fn main() {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Export { path } => {
			println!("exporting {}", path);
			let path = Path::new(path);
		}
	}
}
