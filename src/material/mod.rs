use std::fmt::Debug;

pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{Color, Ray};
use crate::hittable::Hit;

mod lambertian;
mod metal;

pub trait Material: Debug {
	/// Returns the attenuation (reflective radiation) and the scattered ray.
	fn scatter(&self, ray: Ray, hit: Hit) -> Option<(Ray, Color)>;
}