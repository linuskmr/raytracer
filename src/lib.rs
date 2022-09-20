// #![allow(dead_code)]

pub use camera::Camera;
pub use color::Color;
pub use hittable::{
	Hit,
	Hittable,
	sphere::Sphere,
};
pub use image::Image;
pub use ray::Ray;
pub use vec3::Vec3;

mod image;
mod vec3;
mod color;
mod hittable;
pub mod ray;
mod camera;


