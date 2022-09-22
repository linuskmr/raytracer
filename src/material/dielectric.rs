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
		let cos_theta = f64::min((-unit_direction).dot(hit.normal), 1.0);
		let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let direction = if cannot_refract {
			unit_direction.reflect(hit.normal)
		} else {
			unit_direction.refract(hit.normal, refraction_ratio)
		};

		let scattered = Ray {
			origin: hit.point,
			direction,
		};
		Some((scattered, attenuation_color))
	}
}