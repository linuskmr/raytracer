use crate::{Color, Ray, Vec3};
use crate::hittable::Hit;
use crate::material::Material;

#[derive(Debug, Default)]
pub struct Lambertian {
	pub albedo: Color,
}

impl Material for Lambertian {
	fn scatter(&self, _ray: Ray, hit: Hit) -> Option<(Ray, Color)> {
		// scatter always and attenuate by its reflectance?!
		let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

		if scatter_direction.is_near_zero() {
			// If the scatter direction is near zero, we would get NaNs and infinities later, so we just
			// return the normal as the scatter direction.
			scatter_direction = hit.normal;
		}

		let scattered = Ray {
			origin: hit.point,
			direction: scatter_direction,
		};
		let attenuation = self.albedo;
		Some((scattered, attenuation))
	}
}