use std::fmt;

use crate::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);


impl Color {
	const WHITE: Color = Color(255, 255, 255);
	const BLACK: Color = Color(0, 0, 0);
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
	}
}

impl From<Vec3> for Color {
	fn from(vec: Vec3) -> Self {
		let r = (vec.0 * 255.0) as u8;
		let g = (vec.1 * 255.0) as u8;
		let b = (vec.2 * 255.0) as u8;
		Color(r, g, b)
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