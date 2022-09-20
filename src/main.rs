use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use rand::Rng;

use raytracer::{Camera, Color};
use raytracer::Hittable;
use raytracer::Image;
use raytracer::Ray;
use raytracer::Sphere;
use raytracer::Vec3;

fn main() -> Result<(), Box<dyn Error>> {
	// Image
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 720 /*px*/;
	let image_height = (image_width as f64 / aspect_ratio) as usize;
	let antialiasing_samples_per_pixel = 10;
	let reflection_depth = 50;
	let mut image = Image::new(image_width, image_height);
	dbg!(aspect_ratio, image_width, image_height);

	// World
	let world: Vec<Box<dyn Hittable>> = vec![
		Box::new(Sphere {
			center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
			radius: 0.5,
		}),
		Box::new(Sphere {
			center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
			radius: 100.0,
		}),
	];

	// Camera
	let camera = Camera::default();
	let viewport_height = 2.0;
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
		let viewport_center = camera.origin - Vec3 { x: 0.0, y: 0.0, z: focal_length };
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
			// Antialiasing
			let mut color_vec = Vec3::default();
			for _sample in 0..antialiasing_samples_per_pixel {
				// Offset vectors from the lower upper left corner into the pixel of the viewport
				let horizontal_scalar = random_scalar_sample(x, image_width);
				let vertical_scalar = random_scalar_sample(y, image_height);
				let ray = camera.ray(horizontal_scalar, vertical_scalar);
				color_vec = color_vec + Vec3::from(ray_color(ray, &world.as_slice(), reflection_depth));
			}
			color_vec = color_vec / antialiasing_samples_per_pixel as f64;
			*pixel = Color::from(color_vec).gamma_corrected();
		}
		if y & 0xf == 0 {
			println!("{}%", y * 100 / image_height);
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


/// Random anti-aliasing sample
fn random_scalar_sample(iteration: usize, dimension: usize) -> f64 {
	// TODO: Deterministic samples
	assert!(iteration < dimension);
	let random_scalar_offset: f64 = rand::thread_rng().gen_range(-0.5..0.5);
	(iteration as f64 + random_scalar_offset) / (dimension as f64 - 1.0)
}


fn ray_color(ray: Ray, world: &dyn Hittable, depth: usize) -> Color {
	if depth == 0 {
		// Exceeded the ray bounce limit; no more light is gathered.
		return Color::default();
	}

	// Hit something on the world?
	if let Some(hit) = world.hits(ray, 0.0, f64::INFINITY) {
		let reflection_target = hit.point + hit.normal + Vec3::random_in_unit_sphere();
		// Light comes from the background. When a ray is reflected often, they path to the background is longer,
		// so the color is darker.
		let color_vec = Vec3::from(ray_color(
			Ray {
				origin: hit.point,
				direction: reflection_target - hit.point,
			},
			world,
			depth - 1,
		)) * 0.8;
		return Color::from(color_vec);
	}

	// Hits background
	let unit_direction = ray.direction.unit_vector();
	let t = 0.5 * (unit_direction.y + 1.0);
	let color_vec = {
		let start_value = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
		let end_value = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
		start_value * (1.0 - t) + end_value * t
	};
	Color::from(color_vec)
}


/*#[derive(Parser, Debug)]
struct Args {
	#[clap(short, long, default_value = "720")]
	image_width: usize,

	#[clap(short, long, default_value = "10")]
	antialiasing_samples_per_pixel: usize,

	/// Szene description as JSON
	szene: String,
}*/