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
