//! Trait Render

use crate::sivf_misc::canvas::Canvas;
use crate::utils::sizes::ImageSizes;



pub trait TraitRender {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderType {
    Cpu1,       // use only one CPU core (simplest variant)
    Cpu(u32),   // use specified amount of threads
    CpuBest,    // detect all cores/threads and use them all
    Gpu,        // use GPU
}
