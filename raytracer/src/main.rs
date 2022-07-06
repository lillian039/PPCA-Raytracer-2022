use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
pub mod basic_tools;
use basic_tools::{camera::Camera, ray::Ray, vec3::Color, vec3::Point, vec3::Vec3};

fn hit_sphere(center: Point, radius: f64, r: &Ray) -> bool {
    let oc: Vec3 = r.point - center;
    let a = Vec3::dot(&r.direct, &r.direct);
    let b = 2.0 * Vec3::dot(&oc, &r.direct);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        r,
    ) {
        return Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let unit_direction = Vec3::unit_vector(r.direct);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - t)
        + Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t
}
fn main() {
    print!("{}[2J", 27 as char); // Clear screen 27 as char --> esc
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let height = 900;
    let width = 1600;
    let quality = 100; // From 0 to 100
    let path = "output/image3.jpg";

    let camera = Camera::new(2.25, 4.0, 1.0);

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
            let col = ray_color(&r);
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
