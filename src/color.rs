use std::fmt;

use crate::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Color {
	/// Gamma-corrects the color with "gamma 2" by .
	pub fn gamma_corrected(&self) -> Color {
		Color {
			r: ((self.r as f64 / u8::MAX as f64).sqrt() * u8::MAX as f64) as u8,
			g: ((self.g as f64 / u8::MAX as f64).sqrt() * u8::MAX as f64) as u8,
			b: ((self.b as f64 / u8::MAX as f64).sqrt() * u8::MAX as f64) as u8,
		}
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
	}
}

impl From<Vec3> for Color {
	fn from(vec: Vec3) -> Self {
		let r = (vec.x * u8::MAX as f64) as u8;
		let g = (vec.y * u8::MAX as f64) as u8;
		let b = (vec.z * u8::MAX as f64) as u8;
		Color { r, g, b }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display() {
		assert_eq!(Color { r: u8::MAX, g: 255, b: 255 }.to_string(), "#ffffff");
		assert_eq!(Color { r: 0, g: 0, b: 0 }.to_string(), "#000000");
	}
}