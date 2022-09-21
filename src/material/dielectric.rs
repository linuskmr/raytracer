use crate::{Color, Ray};
use crate::hittable::{Hit, IntersectionSide};
use crate::material::Material;

#[derive(Debug)]
pub struct Dielectric {
	pub index_of_refraction: f64,
}

impl Material for Dielectric {
	fn scatter(&self, ray: Ray, hit: Hit) -> Option<(Ray, Color)> {
		let attenuation_color = Color { r: 255, g: 255, b: 255 };
		let refraction_ratio = match hit.intersection_side {
			IntersectionSide::Inside => self.index_of_refraction,
			IntersectionSide::Outside => 1.0 / self.index_of_refraction,
		};
		let unit_direction = ray.direction.unit_vector();
		let refracted = unit_direction.refract(hit.normal, refraction_ratio);
		let scattered = Ray {
			origin: hit.point,
			direction: refracted,
		};
		Some((scattered, attenuation_color))
	}
}