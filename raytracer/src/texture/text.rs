use super::perlin::Perlin;
use crate::basic_tools::vec3::{Color, Point};
use image::{DynamicImage, GenericImageView};
use std::{path::Path, sync::Arc};

pub trait Texture: Send + Sync + Clone {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

#[derive(Default, Clone, Copy)]
pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    //纯色样式
    pub fn new(col: &Color) -> Self {
        Self {
            color_value: (*col),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color_value
    }
}

#[derive(Default, Clone)]
pub struct CheckerTexture<T>
where
    T: Texture,
{
    //棋盘样式
    pub odd: T,
    pub even: T,
}

impl<T: Texture> CheckerTexture<T> {
    pub fn new(_even: T, _odd: T) -> Self {
        Self {
            odd: _odd,
            even: _even,
        }
    }
}
impl CheckerTexture<SolidColor> {
    pub fn new_col(col1: &Color, col2: &Color) -> Self {
        Self {
            odd: SolidColor::new(col1),
            even: SolidColor::new(col2),
        }
    }
}

impl<T: Texture> Texture for CheckerTexture<T> {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        }
        self.even.value(u, v, p)
    }
}

#[derive(Clone, Default)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: (Perlin::new()),
            scale: sc,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        //  let p2 = *p * self.scale;
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

#[derive(Clone, Default)]
pub struct ImageTexture {
    pub data: Vec<[u8; 3]>,
    pub width: u32,
    pub height: u32,
    pub bytes_per_scanline: i32,
}

impl ImageTexture {
    pub fn new(filename: &String) -> Self {
        let pathname = String::from("img/") + filename;

        let path = Path::new(&pathname);
        let image = image::open(path).unwrap();

        let width = image.width();
        let height = image.height();

        let mut dat = Vec::new();
        for i in (0..height).rev() {
            for j in 0..width {
                let pixel = image.get_pixel(j, i);
                let tmp = [pixel[0], pixel[1], pixel[2]];

                dat.push(tmp);
            }
        }
        Self {
            data: (dat),
            width: (width),
            height: (height),
            bytes_per_scanline: (width as i32),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;

        if i >= self.width as i32 {
            i = self.width as i32 - 1;
        }
        if j >= self.height as i32 {
            j = self.height as i32 - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = (j * self.bytes_per_scanline + i) as usize;

        Color::new(
            color_scale * self.data[pixel][0] as f64,
            color_scale * self.data[pixel][1] as f64,
            color_scale * self.data[pixel][2] as f64,
        )
    }
}

#[derive(Clone)]
pub struct ObjectTexture {
    pub coordinate: [[u32; 2]; 3],
    pub img: Arc<DynamicImage>,
}

impl ObjectTexture {
    pub fn new(coor: [[u32; 2]; 3], imge: Arc<DynamicImage>) -> Self {
        Self {
            coordinate: (coor),
            img: (imge),
        }
    }
}

impl Texture for ObjectTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        //   println!("u:{} v:{} ", u, v);
        let i = (1.0 - u - v) * self.coordinate[0][0] as f64
            + u * self.coordinate[1][0] as f64
            + v * self.coordinate[2][0] as f64;
        let j = (1.0 - u - v) * self.coordinate[0][1] as f64
            + u * self.coordinate[1][1] as f64
            + v * self.coordinate[2][1] as f64;
        //println!("i:{} j: {}", i, j);
        let pixel = self.img.get_pixel(i as u32, self.img.height() - j as u32);
        let color_scale = 1.0 / 255.0;
        Color::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}
