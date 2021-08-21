//! SIVF Layer

use crate::sivf_objects::_trait_render::TraitSivfRender;



// #[derive(Clone, Copy, Debug, PartialEq)]
// #[derive(Clone, Debug, PartialEq)]
// TODO
pub struct SivfLayer {
    pub children: Vec<Box<dyn TraitSivfRender>>
}
