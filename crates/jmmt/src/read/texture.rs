//! High-level .ps2_texture reader

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use binext::BinaryRead;
use thiserror::Error;

use crate::{
	format::{ps2_palette::*, ps2_texture::*, Validatable},
	util::Ps2Rgba,
};

#[derive(Error, Debug)]
pub enum Error {
	#[error("underlying I/O error: {0}")]
	IoError(#[from] std::io::Error),

	/// Invalid texture/palette file header
	#[error("invalid texture/palette file header")]
	InvalidHeader,

	/// Couldn't read enough bytes from either texture or palette file
	#[error("read too few bytes in texture/palette file to complete texture")]
	ReadTooSmall,
}

pub type Result<T> = std::result::Result<T, Error>;

/// High-level reader for PS2 textures.
pub struct Ps2TextureReader {
	/// The initial path to the .ps2_texture file.
	path: PathBuf,

	texture_header: Ps2TextureHeader,
	texture_data: Vec<u8>,

	has_palette: bool,
	palette_header: Ps2PaletteHeader,
	palette_data: Vec<u8>,
}

impl Ps2TextureReader {
	pub fn new(path: &Path) -> Self {
		Ps2TextureReader {
			path: path.to_path_buf(),
			texture_header: Ps2TextureHeader::default(),
			texture_data: Vec::default(),
			has_palette: false,
			palette_header: Ps2PaletteHeader::default(),
			palette_data: Vec::default(),
		}
	}

	/// Reads texture data into the reader. For most textures, this will mean we open/read
	/// two files: the .ps2_texture file, and the .ps2_palette file.
	///
	/// This function does not parse or convert data, simply reads it for later conversion.
	/// See [Ps2TextureReader::convert_to_image] for the function which does so.
	pub fn read_data(&mut self) -> Result<()> {
		// Open the .ps2_texture file
		let mut texture_file = File::open(self.path.clone())?;

		self.texture_header = texture_file.read_binary::<Ps2TextureHeader>()?;

		if self.texture_header.valid() {
			let texture_data_size: usize = ((self.texture_header.width as u32
				* self.texture_header.height as u32)
				* (self.texture_header.bpp / 8) as u32) as usize;

			match self.texture_header.bpp {
				8 => {
					self.has_palette = true;
				}
				_ => {}
			}

			if self.has_palette {
				let mut palette_path = self.path.clone();

				// A lot of textures use the texture_<name>.b pattern.
				// the palette for these is in a palette_<name>.b file, so we need to handle that specifically.
				if let Some(ext) = self.path.extension() {
					if ext == "b" {
						match self.path.file_name() {
							Some(name) => {
								if let Some(name) = name.to_str() {
									palette_path.set_file_name(
										String::from(name).replace("texture_", "palette_"),
									);
								} else {
									// this is a bad error to return here, but for now it works I guess
									return Err(Error::InvalidHeader);	
								}
							}
							None => {
								// ditto
								return Err(Error::InvalidHeader);
							}
						}
					} else {
						// We can assume this came from a regular .ps2_texture file, and just set the extension.
						palette_path.set_extension("ps2_palette");
					}
				}

				let mut palette_file = File::open(palette_path)?;
				self.palette_header = palette_file.read_binary::<Ps2PaletteHeader>()?;

				if self.palette_header.valid() {
					// There are no known files in JMMT which use a non-32bpp palette, so I consider this,
					// while hacky, a "ok" assumption. if this isn't actually true then Oh Well
					let pal_data_size = (self.palette_header.color_count * 4) as usize;
					palette_file.seek(SeekFrom::Start(self.palette_header.data_start as u64))?;

					// Read palette color data
					self.palette_data.resize(pal_data_size, 0x0);
					if palette_file.read(self.palette_data.as_mut_slice())? != pal_data_size {
						return Err(Error::ReadTooSmall);
					}
				} else {
					// I give up
					return Err(Error::InvalidHeader);
				}
			}

			texture_file.seek(SeekFrom::Start(
				self.texture_header.data_start_offset as u64,
			))?;
			self.texture_data.resize(texture_data_size, 0x0);

			if texture_file.read(self.texture_data.as_mut_slice())? != texture_data_size {
				return Err(Error::ReadTooSmall);
			}
		} else {
			return Err(Error::InvalidHeader);
		}
		Ok(())
	}

	fn palette_rgba_slice(&self) -> &[Ps2Rgba] {
		assert_eq!(
			self.palette_header.palette_bpp, 32,
			"Palette BPP invalid for usage with palette_rgba_slice"
		);

		// Safety: The palette data buffer will always be 32-bits aligned, so the newly created slice
		// will not end up reading any memory out of bounds of the Vec<u8> allocation.
		// We assert this as true before returning.
		return unsafe {
			std::slice::from_raw_parts(
				self.palette_data.as_ptr() as *const Ps2Rgba,
				self.palette_header.color_count as usize,
			)
		};
	}

	/// Convert the read data into a image implemented by the [image] crate.
	pub fn convert_to_image(&self) -> image::RgbaImage {
		let mut image = image::RgbaImage::new(
			self.texture_header.width as u32,
			self.texture_header.height as u32,
		);

		if self.has_palette {
			let palette_slice: &[Ps2Rgba] = self.palette_rgba_slice();

			// this is shoddy and slow, but meh
			for x in 0..self.texture_header.width as usize {
				for y in 0..self.texture_header.height as usize {
					image[(x as u32, y as u32)] = palette_slice
						[self.texture_data[y * self.texture_header.width as usize + x] as usize]
						.to_rgba();
				}
			}
		} else {
			match self.texture_header.bpp {
				16 => {
					let rgb565_slice = unsafe {
						std::slice::from_raw_parts(
							self.texture_data.as_ptr() as *const u16,
							(self.texture_header.width as u32 * self.texture_header.height as u32)
								as usize,
						)
					};

					for x in 0..self.texture_header.width as usize {
						for y in 0..self.texture_header.height as usize {
							image[(x as u32, y as u32)] = Ps2Rgba::from_rgb565(
								rgb565_slice[y * self.texture_header.width as usize + x],
							)
							.to_rgba();
						}
					}
				}

				// TODO: Find some less awful way to do this, that can be done without creating a new image object.
				32 => {
					let rgba_slice = unsafe {
						std::slice::from_raw_parts(
							self.texture_data.as_ptr() as *const Ps2Rgba,
							(self.texture_header.width as u32 * self.texture_header.height as u32)
								as usize,
						)
					};

					for x in 0..self.texture_header.width as usize {
						for y in 0..self.texture_header.height as usize {
							image[(x as u32, y as u32)] =
								rgba_slice[y * self.texture_header.width as usize + x].to_rgba();
						}
					}
				}

				_ => panic!(
					"somehow got here with invalid bpp {}",
					self.texture_header.bpp
				),
			}
		}

		image
	}
}
