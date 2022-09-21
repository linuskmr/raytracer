use crate::{Color, Ray};
use crate::hittable::Hit;
use crate::material::Material;

#[derive(Debug)]
pub struct Metal {
	pub albedo: Color,
}

impl Material for Metal {
	fn scatter(&self, ray: Ray, hit: Hit) -> Option<(Ray, Color)> {
		let reflected = ray.direction.unit_vector().reflect(hit.normal);
		let scattered = Ray {
			origin: hit.point,
			direction: reflected,
		};
		let attenuation = self.albedo;

		// ?!?!
		if scattered.direction.dot(hit.normal) > 0.0 {
			Some((scattered, attenuation))
		} else {
			None
		}
	}
}