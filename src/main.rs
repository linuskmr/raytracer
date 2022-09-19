use std::error::Error;
use std::fs::File;
use raytracer::Vec3;
use raytracer::Image;
use raytracer::Color;


fn main() -> Result<(), Box<dyn Error>> {
    // let mut image = Box::<Image<500, 500>>::default();
    let mut image = Image::<500, 500>::default();

    let width = image.rows[0].len();
    let height = image.rows.len();

    for (y, row) in image.rows.iter_mut().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let r = (x as f64 / (width - 1) as f64) * 255.0;
            let g = (y as f64 / (height - 1) as f64) * 255.0;
            let b = (0.25) * 255.0;
            *pixel = Color(r as u8, g as u8, b as u8);
        }
    }

    println!("Writing image to filesystem");
    let mut file = File::create("image.ppm");
    image.write_ppm(&mut file.unwrap())?;

    Ok(())
}
