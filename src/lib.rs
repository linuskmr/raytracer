mod image;
mod vec3;
mod color;
mod ray;

pub use vec3::Vec3;
pub use image::Image;
pub use color::Color;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
