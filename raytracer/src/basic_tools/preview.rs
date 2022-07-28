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

use crate::basic_tools::{camera::Camera, ray::Ray};
use crate::hittable::{
    bvh::BVHNode,
    hittable_list::HittableList,
    hittable_origin::{HitRecord, Hittable},
};
use rand::{prelude::SliceRandom, thread_rng};
fn scale_line(r: &Ray, world: &dyn Hittable, camera: Camera) -> f64 {
    let mut rec = HitRecord::default();
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return INFINITY;
    }
    (camera.origin.x - rec.p.x).powi(2)
        + (camera.origin.y - rec.p.y).powi(2)
        + (camera.origin.z - rec.p.z).powi(2)
}

pub fn preview() {
    print!("{}[2J", 27 as char); // Clear screen 27 as char --> esc
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    const ASPECT_RATIO: f64 = 1.0;
    const HEIGHT: usize = 1000;
    const WIDTH: usize = (ASPECT_RATIO * HEIGHT as f64) as usize;
    let quality = 100; // From 0 to 100
    let path = "output/try6_19_preview.jpg";
    let samples_per_pixel = 100;

    let camera = Camera::whale();
    let world = HittableList::whale();

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
    //  let mut output_pixel = Vec::new();
    let mut output_depth = Vec::new();
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

        let t_random_pixel = random_pixal.clone();

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(((hight_end - hight_begin) * WIDTH) as u64));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = channel();

        threads.push((
            thread::spawn(move || {
                let mut progress = 0;
                let channel_send = tx;
                progress_bar.set_position(0);

                let mut depth_thread = Vec::new();

                for y in hight_begin..hight_end {
                    for x in 0..WIDTH {
                        let cnt = y * WIDTH + x;
                        let map_cnt = t_random_pixel[cnt as usize];
                        let x_map = map_cnt % WIDTH;
                        let y_map = map_cnt / WIDTH;

                        let u = (x_map as f64) / (WIDTH as f64);
                        let v = (y_map as f64) / (HEIGHT as f64);
                        let r = camera_thread.get_ray(u, v);
                        let depth = scale_line(&r, &world_thread, camera_thread.clone());
                        depth_thread.push(depth);

                        progress += 1;
                        progress_bar.set_position(progress);
                    }
                }

                channel_send.send(depth_thread).unwrap();
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
                output_depth.append(&mut receive);
            }
            Err(_) => {
                print!("!");
            }
        };
    }
    println!("Generating Image...");

    let mut depth_output = [[0.0; HEIGHT]; WIDTH];
    let mut pixel_num = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let depth = output_depth[pixel_num];
            let cnt = y * WIDTH + x;
            let map_cnt = random_pixal[cnt as usize];
            let y_map = map_cnt / WIDTH;
            let x_map = map_cnt % WIDTH;
            depth_output[x_map][y_map] = depth;
            pixel_num += 1
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut pixel_color = [255, 255, 255];
            if x != 0 && x != WIDTH - 1 && y != 0 && y != HEIGHT - 1 {
                let gx = depth_output[x + 1][y - 1] - depth_output[x - 1][y + 1];
                let fx = depth_output[x - 1][y - 1] - depth_output[x + 1][y + 1];
                let s = (gx.powi(2) + fx.powi(2)).sqrt();

                if s > 10000.0 {
                    pixel_color = [0, 0, 0];
                }
            }
            let pixel = img.get_pixel_mut(x as u32, (HEIGHT - y - 1) as u32);
            *pixel = image::Rgb(pixel_color);
        }
    }
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
