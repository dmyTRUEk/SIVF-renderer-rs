//! Trait Render

use crate::sivf_misc::{canvas::Canvas, sizes::ImageSizes};



pub trait Render {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderType {
    /// use only one CPU core (simplest variant)
    Cpu1,

    /// use specified amount of threads
    Cpu(u32),

    /// detect all cores/threads and use them all
    CpuMax,

    /// use GPU
    Gpu,
}

