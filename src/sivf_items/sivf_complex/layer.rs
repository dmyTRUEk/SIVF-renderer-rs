//! SIVF Layer

use std::any::Any;

use crate::sivf_misc::trait_render::TraitRender;



// #[derive(Clone, Copy, Debug, PartialEq)]
// #[derive(Clone, Debug, PartialEq)]
// TODO
pub struct Layer {
    // pub children: Vec<Box<dyn TraitRender>>
    pub children: Vec<dyn Any>
}

impl Layer {

    pub fn get_children(&self) -> () {
        // self.children
    }

}
