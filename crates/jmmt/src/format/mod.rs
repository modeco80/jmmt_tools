//! Low-level file format definitions. These are suitable for usage with the [binext] crate,
//! which just so happens to be a dependency of this crate! Funny how things work.

pub mod package;
pub mod package_toc;
pub mod ps2_palette;
pub mod ps2_texture;

/// A trait validatable format objects should implement.
/// TODO: integrate this with some FourCC crate, or re-invent the wheel.
pub trait Validatable {
	/// Returns true if the object is valid, false otherwise.
	fn valid(&self) -> bool;
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
