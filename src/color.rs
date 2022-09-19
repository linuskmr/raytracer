use std::{fmt, io};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);


impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{:x}{:x}{:x}", self.0, self.1, self.2)
	}
}