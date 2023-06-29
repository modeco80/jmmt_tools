//! General utility code, used throughout the JMMT crate

/// Like image::Rgba<u8>, but a safe C repressentation,
/// and alpha is multiplied to match PS2. Some helpers
/// are also provided to work with 16-bit colors.
#[derive(Clone)]
#[repr(C, packed)]
pub struct Ps2Rgba {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Ps2Rgba {
	pub const fn to_rgba(&self) -> image::Rgba<u8> {
		// avoid multiplication overflow
		if self.a as u32 * 2 > 255 {
			return image::Rgba::<u8>([self.r, self.g, self.b, 255]);
		}
		image::Rgba::<u8>([self.r, self.g, self.b, self.a * 2])
	}

	/// Create a instance from an rgb565 16bpp pixel.
	pub const fn from_rgb565(value: u16) -> Ps2Rgba {
		return Ps2Rgba {
			r: ((value & 0x7C00) >> 7) as u8,
			g: ((value & 0x03E0) >> 2) as u8,
			b: ((value & 0x001F) << 3) as u8,
			a: 255,
		};
	}
}


/// Make a Rust [String] from a byte slice that came from a C string/structure.
///
/// # Usage
///
/// The byte slice has to be a valid UTF-8 string.
/// (Note that in most cases, ASCII strings are valid UTF-8, so this isn't something you'll particularly
/// have to worry about).
///
/// # Safety
///
/// This function does not directly make use of any unsafe Rust code.
pub fn make_c_string(bytes: &[u8]) -> Option<String> {
	let bytes_without_null = match bytes.iter().position(|&b| b == 0) {
		Some(ix) => &bytes[..ix],
		None => bytes,
	};

	match std::str::from_utf8(bytes_without_null).ok() {
		Some(string) => Some(String::from(string)),
		None => None,
	}
}
