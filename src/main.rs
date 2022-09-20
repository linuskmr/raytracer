use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use raytracer::Color;
use raytracer::Hittable;
use raytracer::Image;
use raytracer::Ray;
use raytracer::Sphere;
use raytracer::Vec3;

fn ray_color(ray: Ray) -> Color {
	// Hit sphere?
	{
		let sphere = Sphere {
			center: Vec3(0.0, 0.0, -1.0),
			radius: 0.5,
		};
		if let Some(hit) = sphere.hits(ray, 0.0, 10000.0) {
			let normal = (ray.at(hit.t) - sphere.center).unit_vector();
			let color_vec = (normal + Vec3(1.0, 1.0, 1.0)) * 0.5;
			return Color::from(color_vec);
		}
	}

	// Hits background
	let unit_direction = ray.direction.unit_vector();
	let t = 0.5 * (unit_direction.y() + 1.0);
	let color_vec = {
		let start_value = Vec3(1.0, 1.0, 1.0);
		let end_value = Vec3(0.5, 0.7, 1.0);
		start_value * (1.0 - t) + end_value * t
	};
	Color::from(color_vec)
}


fn main() -> Result<(), Box<dyn Error>> {
	// Image
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 720 /*px*/;
	let image_height = (image_width as f64 / aspect_ratio) as usize;
	let mut image = Image::new(image_width, image_height);
	dbg!(aspect_ratio, image_width, image_height);

	// Camera
	let viewport_height = 2.0;
	let viewport_width = aspect_ratio * viewport_height;
	let focal_length = 1.0;
	dbg!(viewport_width, viewport_height, focal_length);

	// Eye or camera center
	let origin = Vec3(0.0, 0.0, 0.0);
	// Offset vectors from the lower upper left corner of the viewport
	let horizontal = Vec3(viewport_width, 0.0, 0.0);
	let vertical = Vec3(0.0, -viewport_height, 0.0);
	// "Origin" of the viewport
	let upper_left_corner = {
		// TODO: I would prefer to use the positive z-axis, but I'll follow the tutorial for now.
		//  "In order to respect the convention of a right handed coordinate system, into the screen is the negative z-axis."
		let viewport_center = origin - Vec3(0.0, 0.0, focal_length);
		viewport_center
			- horizontal / 2.0 // half a screen to the left
			- vertical / 2.0 // half a screen to the top
	};
	dbg!(upper_left_corner);


	println!("Rendering image");
	let render_start_timestamp = std::time::Instant::now();
	// Render
	for (y, row) in image.rows.iter_mut().enumerate() {
		for (x, pixel) in row.iter_mut().enumerate() {
			// Offset vectors from the lower upper left corner into the pixel of the viewport
			let horizontal_offset = {
				let horizontal_scalar = x as f64 / (image_width - 1) as f64;
				horizontal * horizontal_scalar
			};
			let vertical_offset = {
				let vertical_scalar = y as f64 / (image_height - 1) as f64;
				vertical * vertical_scalar
			};
			let ray = Ray {
				origin,
				direction: {
					// A direction vector is always calculated from target minus start
					let target = upper_left_corner + horizontal_offset + vertical_offset;
					let start = origin;
					target - start
				},
			};
			*pixel = ray_color(ray);
		}
	}
	let render_duration = render_start_timestamp.elapsed();
	println!("Rendered image in {:?}", render_duration);

	println!("Writing image to filesystem");
	let write_start_timestamp = std::time::Instant::now();
	let file = File::create("image.ppm")?;
	let mut file_writer = BufWriter::new(file);
	image.write_binary_ppm(&mut file_writer)?;
	let write_duration = write_start_timestamp.elapsed();
	println!("Wrote image to filesystem in {:?}", write_duration);

	Ok(())
}
