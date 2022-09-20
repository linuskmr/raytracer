use crate::{Ray, Vec3};

#[derive(Clone, PartialEq, Debug)]
pub struct Camera {
	/// The position of the camera / "eye".
	pub origin: Vec3,
	/// Horizontal offset vector from the upper left corner to the upper right corner of the viewport
	pub horizontal: Vec3,
	/// Vertical offset vector from the upper left corner to the lower left corner of the viewport
	pub vertical: Vec3,
	/// Upper left corner of the viewport
	pub upper_left_corner: Vec3,
}

impl Camera {
	fn new(aspect_ratio: f64, viewport_height: f64, origin: Vec3) -> Self {
		assert!(viewport_height > 0.0);

		let viewport_width = aspect_ratio * viewport_height;
		let focal_length = 1.0;
		dbg!(viewport_width, viewport_height, focal_length);

		// Eye or camera center
		// Offset vectors from the lower upper left corner of the viewport
		let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
		let vertical = Vec3 { x: 0.0, y: -viewport_height, z: 0.0 };
		// "Origin" of the viewport
		let upper_left_corner = {
			// TODO: I would prefer to use the positive z-axis, but I'll follow the tutorial for now.
			//  "In order to respect the convention of a right handed coordinate system, into the screen is the negative z-axis."
			let viewport_center = origin - Vec3 { x: 0.0, y: 0.0, z: focal_length };
			viewport_center
				- horizontal / 2.0 // half a screen to the left
				- vertical / 2.0 // half a screen to the top
		};

		Self {
			origin,
			horizontal,
			vertical,
			upper_left_corner,
		}
	}

	/// Returns a ray that starts at the camera's origin and goes through the pixel in the viewport, offset by the
	/// given horizontal and vertical scalars (between 0.0 and 1.0).
	pub fn ray(&self, horizontal_scalar: f64, vertical_scalar: f64) -> Ray {
		assert!(horizontal_scalar >= -0.05 && horizontal_scalar <= 1.05);
		assert!(vertical_scalar >= -0.05 && vertical_scalar <= 1.05);

		// Offset vectors from the lower upper left corner into the pixel of the viewport
		let horizontal_offset = self.horizontal * horizontal_scalar;
		let vertical_offset = self.vertical * vertical_scalar;
		Ray {
			origin: self.origin,
			direction: {
				// A direction vector is always calculated from target minus start
				let target = self.upper_left_corner + horizontal_offset + vertical_offset;
				let start = self.origin;
				target - start
			},
		}
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self::new(16.0 / 9.0, 2.0, Vec3::ZERO)
	}
}