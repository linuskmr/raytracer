use crate::{Ray, Vec3};

#[derive(Clone, PartialEq, Debug)]
pub struct Camera {
	/// The position of the camera / "eye".
	pub look_from: Vec3,
	/// Horizontal offset vector from the upper left corner to the upper right corner of the viewport
	pub horizontal: Vec3,
	/// Vertical offset vector from the upper left corner to the lower left corner of the viewport
	pub vertical: Vec3,
	/// Upper left corner of the viewport
	pub upper_left_corner: Vec3,
}

impl Camera {
	/// Create a new camera.
	/// 
	/// # Arguments
	/// 
	/// * `aspect_ratio` - The aspect ratio of the viewport (0.0..).
	/// * `vertical_fov` - The vertical field of view in degrees (0.0..360.0).
	/// * `look_from` - The position of the camera / "eye".
	/// * `look_at` - The position the camera is looking at.
	/// * `vertical_up` - The up vector of the camera.
	fn new(
		aspect_ratio: f64,
		vertical_fov: f64,
		look_from: Vec3,
		look_at: Vec3,
		vertical_up: Vec3,
	) -> Self {
		assert!((0.0..).contains(&aspect_ratio));
		assert!((0.0..360.0).contains(&vertical_fov));

		let theta = vertical_fov.to_radians();
		// Scalar value of the viewport height
		let h = f64::tan(theta / 2.0);
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;
		dbg!(viewport_width, viewport_height);

		let w = (look_from - look_at).unit_vector();
		let u = -vertical_up.cross(w).unit_vector();
		let v = w.cross(u);

		// Eye or camera center
		// Offset vectors from the lower upper left corner of the viewport
		let horizontal = u * viewport_width;
		let vertical = v * viewport_height;
		// "Origin" of the viewport
		let upper_left_corner = {
			let viewport_center = look_from - w;
			viewport_center
				- horizontal / 2.0 // half a screen to the left
				- vertical / 2.0 // half a screen to the top
		};

		Self {
			look_from,
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
			origin: self.look_from,
			direction: {
				// A direction vector is always calculated from target minus start
				let target = self.upper_left_corner + horizontal_offset + vertical_offset;
				let start = self.look_from;
				target - start
			},
		}
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self::new(
			16.0 / 9.0,
			90.0,
			Vec3 { x: -2.0, y: 2.0, z: 1.0 },
			Vec3 { x: 0.0, y: 0.0, z: -1.0 },
			Vec3 { x: 0.0, y: 1.0, z: 0.0 },
		)
	}
}