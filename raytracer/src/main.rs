use std::{
    f64::INFINITY,
    fs::File,
    process::exit,
    sync::{mpsc::channel, Arc},
    thread,
};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
pub mod basic_tools;
pub mod hittable;
pub mod material;
pub mod texture;
use basic_tools::{camera::Camera, ray::Ray, vec3::Color, vec3::Vec3};
use hittable::{
    bvh::BVHNode,
    hittable_list::HittableList,
    hittable_origin::{clamp, random_double, HitRecord, Hittable},
};
fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0);
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if rec
            .mat_ptr
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return ray_color(&scattered, world, depth - 1) * attenuation;
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::unit_vector(r.direct);
    let t = (unit_direction.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
fn main() {
    print!("{}[2J", 27 as char); // Clear screen 27 as char --> esc
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let aspect_ratio = 16.0 / 9.0;
    let height = 900;
    let width = (aspect_ratio * height as f64) as u32;
    let quality = 100; // From 0 to 100
    let path = "output/book2_image2.jpg";
    let samples_per_pixel = 100;
    let max_depth = 50;
  //  let camera = Camera::new_two_sphere();

   // let world = HittableList::two_sphere();
    let camera=Camera::new_random_scence();
    let world=HittableList::random_scene();
    let bvhworld = BVHNode::new(world.objects.clone(), 0, world.objects.len(), 0.0, 1.0);

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + "x" + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true);

    let thread_total = 8;
    let mut threads = Vec::new();
    let mut output_pixel = Vec::new();
    let hight_line = height / thread_total;

    for thread_num in 0..thread_total {
        let hight_begin = hight_line * thread_num;
        let mut hight_end = hight_begin + hight_line;
        if thread_num == thread_total - 1 {
            hight_end = height;
        }

        let world_thread = bvhworld.clone();
        let camera_thread = camera;

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(((hight_end - hight_begin) * width) as u64));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = channel();

        threads.push((
            thread::spawn(move || {
                let mut progress = 0;
                let channel_send = tx;
                progress_bar.set_position(0);

                let mut pixel_color_thread = Vec::new();

                for y in hight_begin..hight_end {
                    for x in 0..width {
                        let mut col = Vec3::new(0.0, 0.0, 0.0);
                        for _s in 0..samples_per_pixel {
                            let u = (x as f64 + random_double()) / (width as f64);
                            let v = (y as f64 + random_double()) / (height as f64);
                            let r = camera_thread.get_ray(u, v);
                            col += ray_color(&r, &world_thread, max_depth);
                        }
                        col = col / samples_per_pixel as f64;
                        let pixel_color = [
                            (clamp(col.x.sqrt(), 0.0, 0.999) * 255.999) as u8,
                            (clamp(col.y.sqrt(), 0.0, 0.999) * 255.999) as u8,
                            (clamp(col.z.sqrt(), 0.0, 0.999) * 255.999) as u8,
                        ];
                        pixel_color_thread.push(pixel_color);

                        progress += 1;
                        progress_bar.set_position(progress);
                    }
                }

                channel_send.send(pixel_color_thread).unwrap();
                progress_bar.finish_with_message("Finish.");
            }),
            rx,
        ));
    }

    multiprogress.join().unwrap();

    println!("Collecting Threads Results...");

    for _thread_num in 0..thread_total {
        let thread = threads.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut receive = thread.1.recv().unwrap();
                output_pixel.append(&mut receive);
            }
            Err(_) => {
                print!("!");
            }
        };
    }
    println!("Generating Image...");

    let mut pixel_num = 0;
    for y in 0..height {
        for x in 0..width {
            let pixel_color = output_pixel[pixel_num];
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            pixel_num += 1;
        }
    }

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
