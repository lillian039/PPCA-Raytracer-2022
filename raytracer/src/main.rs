pub mod basic_tools;
pub mod hittable;
pub mod material;
pub mod texture;
use basic_tools::render::render;

fn main() {
    render();
}
