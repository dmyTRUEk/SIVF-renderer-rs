//! SIVF Struct

use serde_derive::{Serialize, Deserialize};

use crate::utils::color::ColorModel;
use crate::sivf_objects::sivf_complex::layer::Layer;
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::blend_types::BlendType;
use crate::utils::sizes::{sizes, ImageSizes, image_sizes};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SivfStruct {
    // TODO: force serde convert array of two into [ImageSizes]
    pub image_sizes: ImageSizes,
    // TODO LATER: maybe add lowercase support
    pub color_model: ColorModel,
    pub root_layer: Layer,
}

impl SivfStruct {

    pub fn render(&self) -> Canvas {
        let mut canvas_main = Canvas::new(self.image_sizes);
        // TODO: maybe use [Iterator.cloned()]
        // for child in self.root_layer.get_children().iter() {
        //     let blend_type: BlendType = self.root_layer.blend_type;
        //     let canvas_child: Canvas = child.render(self.image_sizes);
        //     canvas_main.blend_with(canvas_child, blend_type);
        // }
        canvas_main
    }

}
