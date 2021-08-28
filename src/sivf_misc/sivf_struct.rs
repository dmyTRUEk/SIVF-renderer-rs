//! SIVF Struct

use crate::sivf_objects::sivf_complex::layer::Layer;
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::blend_types::{BlendType, BlendTypes};
use crate::sivf_misc::serde::deserialize_to_sivf_struct;
use crate::sivf_misc::trait_render::RenderType;
use crate::utils::color::ColorModel;
use crate::utils::sizes::ImageSizes;



#[derive(Clone, Debug, PartialEq)]
pub struct SivfStruct {
    // TODO: force serde convert array of two into [ImageSizes]
    pub image_sizes: ImageSizes,
    // TODO LATER: maybe add lowercase support
    pub color_model: ColorModel,
    pub root_layer: Layer,
}

impl SivfStruct {

    pub fn from(value: &serde_yaml::Value) -> Self {
        deserialize_to_sivf_struct(value)
    }

    pub fn render(&self, render_type: RenderType) -> Canvas {
        self.root_layer.render(self.image_sizes, render_type)
    }

}
