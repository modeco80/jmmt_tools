//! Low-level structure definitions

pub mod package;
pub mod package_toc;

/// A trait validatable format objects should implement.
pub trait Validatable {
	/// Returns true if the object is valid, false otherwise.
	fn valid(&self) -> bool;
}
