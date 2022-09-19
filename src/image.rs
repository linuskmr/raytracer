use std::io;
use std::io::Write;

use crate::color::Color;

#[derive(Clone)]
pub struct Image {
	pub rows: Vec<Vec<Color>>,
}

impl Image {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			rows: vec![vec![Color(0, 0, 0); width]; height],
		}
	}

	pub fn width(&self) -> usize {
		match self.rows.get(0) {
			Some(row) => row.len(),
			None => 0,
		}
	}

	pub fn height(&self) -> usize {
		self.rows.len()
	}

	pub fn aspect_ratio(&self) -> f64 {
		self.width() as f64 / self.height() as f64
	}

	/// Writes the image as binary Portable Pixmap (PPM, type P6) to the given writer.
	pub fn write_binary_ppm(&self, writer: &mut impl io::Write) -> io::Result<()> {
		// Header
		writer.write_fmt(format_args!("P6\n{} {}\n255\n", self.width(), self.height()))?;

		// Image data
		for row in &self.rows {
			for pixel in row {
				writer.write_all(&[pixel.0, pixel.1, pixel.2])?;
			}
		}

		Ok(())
	}

	// Writes the image as ASCII Portable Pixmap (PPM, type P3) to the given writer.
	pub fn write_ascii_ppm(&self, writer: &mut impl io::Write) -> io::Result<()> {
		// Header
		writer.write_fmt(format_args!("P3\n{} {}\n255\n", self.width(), self.height()))?;

		// Image data
		for row in &self.rows {
			for pixel in row {
				writer.write_fmt(format_args!("{} {} {}", pixel.0, pixel.1, pixel.2))?;
				writer.write_all(b" ")?;
			}
			writer.write_all(b"\n")?;
		}

		Ok(())
	}
}