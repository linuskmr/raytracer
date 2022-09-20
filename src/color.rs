use std::fmt;

use crate::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
	}
}

impl From<Vec3> for Color {
	fn from(vec: Vec3) -> Self {
		let r = (vec.x * 255.999) as u8;
		let g = (vec.y * 255.999) as u8;
		let b = (vec.z * 255.999) as u8;
		Color { r, g, b }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display() {
		assert_eq!(Color::WHITE.to_string(), "#ffffff");
		assert_eq!(Color::BLACK.to_string(), "#000000");
	}
}