//! SIVF Layer

use std::any::Any;

use serde_derive::{Serialize, Deserialize};

use crate::sivf_objects::sivf_any_object::SivfObject;
use crate::sivf_misc::blend_types::BlendType;
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::trait_render::TraitRender;
use crate::utils::sizes::ImageSizes;



#[derive(Clone, Debug, /*Serialize,*/ Deserialize)]
pub struct Layer {

    #[serde(default="deserialize::blend_type")]
    pub blend_type: BlendType,

    // TODO: maybe smt like #[serde(with="")]
    #[serde(deserialize_with="deserialize::children", flatten)]
    children: Vec<SivfObject>,

}

mod deserialize {

    use std::str::FromStr;
    use std::fmt::Display;

    use serde::Deserializer;

    use crate::sivf_misc::blend_types::BlendType;
    use crate::sivf_objects::sivf_any_object::SivfObject;



    pub(crate) fn blend_type() -> BlendType {
        BlendType::Overlap
    }

    pub(crate) fn children<'de, D>(deserializer: D) -> Result<Vec<SivfObject>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(vec![])
    }

}

#[derive(Clone, Debug, /*Serialize,*/ Deserialize)]
pub enum LayerElement {
    BlendType(BlendType),
    SivfObject(SivfObject),
}

impl Layer {

    // TODO: try return type [Self]
    pub fn new(blend_type: BlendType, children: Vec<SivfObject>) -> Layer {
        // child.all have [.render()] is guaranteed by [SivfObject]
        Layer { blend_type, children }
    }

    pub fn extend(&mut self, sivf_objects: Vec<SivfObject>) {
        self.children.extend(sivf_objects);
    }
    pub fn push(&mut self, sivf_object: SivfObject) {
        self.children.push(sivf_object);
    }

    pub fn get_children(self) -> Vec<SivfObject> {
        self.children
    }

    pub fn render(&self, image_sizes: ImageSizes) -> Canvas {
        // TODO LATER: try:
        //   - render all, then blend all
        //   - render one, blend one, repeat
        self.children.iter().fold(Canvas::new(image_sizes), |acc, child|
            // TODO
            acc.blend_with(child.render(image_sizes), self.blend_type)
        )
    }

}
