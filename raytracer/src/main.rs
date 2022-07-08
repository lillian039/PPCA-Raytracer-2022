use std::{f64::INFINITY, fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
pub mod basic_tools;
pub mod hittable;
use basic_tools::{camera::Camera, ray::Ray, vec3::Color, vec3::Point, vec3::Vec3};
use hittable::{
    hittable_list::HittableList,
    hittable_origin::{HitRecord, Hittable},
    sphere::Sphere,
};

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(r.direct);
    let t = (unit_direction.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
fn main() {
    print!("{}[2J", 27 as char); // Clear screen 27 as char --> esc
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let height = 900;
    let width = 1600;
    let quality = 100; // From 0 to 100
    let path = "output/image5.jpg";

    let camera = Camera::new(2.25, 4.0, 1.0);

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + "x" + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width as f64);
            let v = y as f64 / (height as f64);
            let r = Ray {
                point: camera.origin,
                direct: camera.lower_left_corner + camera.horizontal * u + camera.vertical * v
                    - camera.origin,
            };
            let col = ray_color(&r, &world);
            let pixel_color = [
                (col.x * 255.999) as u8,
                (col.y * 255.999) as u8,
                (col.z * 255.999) as u8,
            ];
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
