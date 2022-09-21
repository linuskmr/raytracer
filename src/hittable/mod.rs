//! Components for rays hitting objects.


pub use sphere::Sphere;

use crate::{Ray, Vec3};

pub mod sphere;
mod hittable_list;

/// Whether the [Ray] intersects the the object from the inside or outside.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntersectionSide {
	Inside,
	Outside,
}

/// A hit is a point on a surface that was hit by a [Ray].
#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
	/// The point at which the ray hit the object.
	pub point: Vec3,
	/// The normal of the object at the point of intersection.
	pub normal: Vec3,
	/// The distance along the ray at which the object was hit.
	pub t: f64,
	/// Whether the ray intersects the the object from the inside or outside.
	pub intersection_side: IntersectionSide,
}

/// A trait for objects that can be hit by a ray.
pub trait Hittable {
	/// Returns the [Hit] point of the ray if it hits the object, otherwise returns `None`.
	fn hits(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

/// Calculates whether the [Ray] intersects the object from the inside or outside.
fn calc_intersection_side(ray: Ray, outward_normal: Vec3) -> IntersectionSide {
	// If the ray and the outward normal are pointing in the same direction, the dot product will be positive
	// and the ray is hitting the object from the *inside*. If the dot product is negative, the ray is hitting the
	// object from the *outside*.
	if ray.direction.dot(outward_normal) < 0.0 {
		IntersectionSide::Outside
	} else {
		IntersectionSide::Inside
	}
}

/// Returns the normal vector of the object, taking into account whether the [Ray] intersected the object from the
/// inside or outside.
fn calc_normal(ray_intersection_side: IntersectionSide, outward_normal: Vec3) -> Vec3 {
	match ray_intersection_side {
		IntersectionSide::Inside => -outward_normal,
		IntersectionSide::Outside => outward_normal,
	}
}