//! SIVF Struct

use crate::utils::color::ColorModel;
use crate::sivf_objects::layer::SivfLayer;
use crate::sivf_objects::_trait_render::TraitSivfRender;
use crate::sivf_objects::_canvas::Canvas;
use crate::sivf_objects::_blend_types::BlendType;
use crate::utils::sizes::{sizes, ImageSizes, image_sizes};



// #[derive(Clone, Copy, Debug, PartialEq)]
pub struct SivfStruct {
    pub image_sizes: ImageSizes,
    pub color_model: ColorModel,
    pub root_layer: SivfLayer,
}

impl SivfStruct {

    pub fn render(&self) -> Canvas {
        let mut canvas = Canvas::new(self.image_sizes);
        for child in &self.root_layer.children {
            let blend_type: BlendType = BlendType::Overlap;
            let canvas_child: Canvas = child.render(self.image_sizes);
            // TODO: blend
            canvas.blend_with(canvas_child, blend_type);
        }
        canvas
    }

}
