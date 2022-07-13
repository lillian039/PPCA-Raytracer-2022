use super::perlin::Perlin;
use crate::basic_tools::vec3::{Color, Point};
use std::sync::Arc;

pub trait Texture: Send + Sync {
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
pub struct CheckerTexture {
    //棋盘样式
    pub odd: Option<Arc<dyn Texture>>,
    pub even: Option<Arc<dyn Texture>>,
}

impl CheckerTexture {
    pub fn new(_even: Arc<dyn Texture>, _odd: Arc<dyn Texture>) -> Self {
        Self {
            odd: (Some(_odd)),
            even: (Some(_even)),
        }
    }

    pub fn new_col(col1: &Color, col2: &Color) -> Self {
        Self {
            odd: (Some(Arc::new(SolidColor::new(col1)))),
            even: (Some(Arc::new(SolidColor::new(col2)))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.as_ref().unwrap().value(u, v, p);
        }
        self.even.as_ref().unwrap().value(u, v, p)
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
