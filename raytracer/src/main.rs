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
use basic_tools::{
    camera::{get_background, Camera},
    ray::Ray,
    vec3::{add_u, minus_u, pow_u, Color, Vec3},
};
use hittable::{
    bvh::BVHNode,
    hittable_list::HittableList,
    hittable_origin::{clamp, random_double, HitRecord, Hittable},
    pdf::{HittablePDF, MixturePDF, PDF},
};
use material::metal::ScatterRecord;
use rand::{prelude::SliceRandom, thread_rng};

fn ray_color(r: &Ray, t: f64, world: &dyn Hittable, light: Arc<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::default();
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return get_background(t);
    }
    let mut srec = ScatterRecord::default();
    let emitted = rec
        .mat_ptr
        .as_ref()
        .unwrap()
        .emit(rec.u, rec.v, &rec.p, r, &rec);

    if !rec.mat_ptr.as_ref().unwrap().scatter(r, &rec, &mut srec) {
        return emitted;
    }

    if srec.is_specular {
        let a = ray_color(&srec.specular_ray, t, world, light, depth - 1) * srec.attenuation;
        return a;
    }
    let light_ptr = Arc::new(HittablePDF::new(light.clone(), rec.p));
    let p = MixturePDF::new(light_ptr, srec.pdf_ptr.as_ref().unwrap().clone());

    let scattered = Ray::new(rec.p, p.generate(), r.time);
    let pdf = p.value(&scattered.direct);
    emitted
        + ray_color(&scattered, t, world, light, depth - 1)
            * srec.attenuation
            * rec
                .mat_ptr
                .clone()
                .as_ref()
                .unwrap()
                .scattering_pdf(r, &rec, &scattered)
            / pdf
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen 27 as char --> esc
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const HEIGHT: usize = 500;
    const WIDTH: usize = (ASPECT_RATIO * HEIGHT as f64) as usize;
    let quality = 100; // From 0 to 100
    let path = "output/try6_14.jpg";
    let samples_per_pixel = 1000;
    let max_depth = 50;

    let camera = Camera::new_random_scence();
    let world = HittableList::random_scene();
    let lamp = Arc::new(HittableList::whale_lights());

    let bvhworld = BVHNode::new(world.objects.clone(), 0, world.objects.len(), 0.0, 1.0);

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(WIDTH.to_string() + "x" + &HEIGHT.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    println!("Sample per pixel: {}", samples_per_pixel);
    // Create image data
    let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true);

    let thread_total = 8;
    let mut threads = Vec::new();
    let mut output_pixel = Vec::new();
    let hight_line = HEIGHT / thread_total;

    let mut random_pixal = Vec::default();
    let sum = WIDTH * HEIGHT;
    for i in 0..sum {
        random_pixal.push(i);
    }
    let mut rng = thread_rng();
    random_pixal.shuffle(&mut rng);

    for thread_num in 0..thread_total {
        let hight_begin = hight_line * thread_num;
        let mut hight_end = hight_begin + hight_line;
        if thread_num == thread_total - 1 {
            hight_end = HEIGHT;
        }

        let world_thread = bvhworld.clone();
        let camera_thread = camera;
        //let background_color = Color::new(0.7, 0.8, 1.0);

        let t_random_pixel = random_pixal.clone();

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(((hight_end - hight_begin) * WIDTH) as u64));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = channel();
        let light = lamp.clone();

        threads.push((
            thread::spawn(move || {
                let mut progress = 0;
                let channel_send = tx;
                progress_bar.set_position(0);

                let mut pixel_color_thread = Vec::new();

                for y in hight_begin..hight_end {
                    for x in 0..WIDTH {
                        let cnt = y * WIDTH + x;
                        let map_cnt = t_random_pixel[cnt as usize];
                        let x_map = map_cnt % WIDTH;
                        let y_map = map_cnt / WIDTH;

                        let mut col = Vec3::new(0.0, 0.0, 0.0);
                        for _s in 0..samples_per_pixel {
                            let u = (x_map as f64 + random_double()) / (WIDTH as f64);
                            let v = (y_map as f64 + random_double()) / (HEIGHT as f64);
                            let r = camera_thread.get_ray(u, v);
                            col += ray_color(&r, v, &world_thread, light.clone(), max_depth);
                        }
                        col = col / samples_per_pixel as f64;
                        let mut r = col.x;
                        let mut g = col.y;
                        let mut b = col.z;
                        if r.is_nan() {
                            r = 0.0;
                        }
                        if g.is_nan() {
                            g = 0.0;
                        }
                        if b.is_nan() {
                            b = 0.0;
                        }
                        let pixel_color = [
                            (clamp(r.sqrt(), 0.0, 0.999) * 255.999) as u8,
                            (clamp(g.sqrt(), 0.0, 0.999) * 255.999) as u8,
                            (clamp(b.sqrt(), 0.0, 0.999) * 255.999) as u8,
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

    let mut image_output = [[[0; 3]; HEIGHT]; WIDTH];
    let mut pixel_num = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel_color = output_pixel[pixel_num];
            let cnt = y * WIDTH + x;
            let map_cnt = random_pixal[cnt as usize];
            let y_map = map_cnt / WIDTH;
            let x_map = map_cnt % WIDTH;
            image_output[x_map][y_map] = pixel_color;
            pixel_num += 1
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut pixel_color = image_output[x as usize][y as usize];
            if x != 0 && x != WIDTH - 1 && y != 0 && y != HEIGHT - 1 {
                let gx = add_u(
                    image_output[x][y + 1],
                    image_output[x - 1][y + 1],
                    image_output[x + 1][y + 1],
                );
                let fx = add_u(
                    image_output[x][y - 1],
                    image_output[x - 1][y - 1],
                    image_output[x + 1][y - 1],
                );
                let sx = minus_u(gx, fx);
                let gy = add_u(
                    image_output[x + 1][y + 1],
                    image_output[x + 1][y],
                    image_output[x + 1][y - 1],
                );
                let fy = add_u(
                    image_output[x - 1][y + 1],
                    image_output[x - 1][y],
                    image_output[x - 1][y + 1],
                );
                let sy = minus_u(gy, fy);
                let s = pow_u(sx) + pow_u(sy);
                // println!("G!:{}", s);
                if s > 9000 {
                    pixel_color = [0, 0, 0];
                }
            }
            let pixel = img.get_pixel_mut(x as u32, (HEIGHT - y - 1) as u32);
            *pixel = image::Rgb(pixel_color);
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
