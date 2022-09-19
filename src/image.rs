use std::hash::Hasher;
use std::io;
use std::io::Write;
use crate::color::Color;
use crate::Vec3;

#[derive(Clone)]
pub struct Image<const X: usize, const Y: usize>{
    pub rows: Box<[[Color; X]]>,
}

impl<const X: usize, const Y: usize> Image<X, Y> {
    pub fn write_ppm(&self, writer: &mut impl io::Write) -> io::Result<()> {
        // Header
        writer.write_fmt(format_args!("P3\n{X} {Y}\n255\n"))?;

        // Image data
        for row in self.rows.iter() {
            for pixel in row {

                writer.write_fmt(format_args!("{} {} {}", pixel.0, pixel.1, pixel.2))?;
                writer.write_all(b" ")?;
            }
            writer.write_all(b"\n")?;
        }

        Ok(())
    }
}


impl<const X: usize, const Y: usize> Default for Image<X, Y> {
    fn default() -> Self {
        Self {
            rows: vec![[Color::default(); X]; Y].into_boxed_slice()
        }
    }
}