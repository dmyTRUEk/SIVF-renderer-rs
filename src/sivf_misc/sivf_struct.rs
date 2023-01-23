//! SIVF Struct

use crate::{
    sivf_misc::{
        canvas::Canvas,
        color::ColorModel,
        render::{Render, RenderType},
        serde::deserialize_to_sivf_struct,
        sizes::ImageSizes,
        vals::Vals,
    },
    sivf_objects::complex::layer::Layer,
};



#[derive(Clone, Debug, PartialEq)]
pub struct SivfStruct {
    // TODO: force serde convert array of two into [ImageSizes]
    pub image_sizes: ImageSizes,
    // TODO LATER: maybe add lowercase support
    pub color_model: ColorModel,
    pub vals: Vals,
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

