use clap::{Parser, Subcommand};

use std::path::Path;
use jmmt::read::texture::Ps2TextureReader;

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

fn main() {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Export { path } => {
			let path = Path::new(path);

			if !path.is_file() {
				println!("Need to provide a path to a file to export.");
				return ();
			}

			let mut reader = Ps2TextureReader::new(path);

			match reader.read_data() {
				Ok(_) => {
					let mut path = Path::new(path).to_path_buf();
					path.set_extension("png");

					match reader.convert_to_image().save(path.clone()) {
						Ok(_) => {
							println!("Wrote image {}", path.display())
						}
						Err(error) => {
							println!("Error saving image: {}", error);
						}
					};
				}
				Err(error) => {
					println!("Error reading texture data: {}", error);
					return ();
				}
			}
		}
	}
}
