use std::sync::Arc;

use crate::basic_tools::vec3::{Color, Point};

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
