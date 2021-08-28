//! used for array of SIVF items

use crate::sivf_objects::sivf_complex::layer::Layer;
use crate::sivf_objects::sivf_shapes::circle::Circle;
use crate::sivf_objects::sivf_shapes::square::Square;
use crate::utils::sizes::ImageSizes;
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::trait_render::{RenderType, TraitRender};



#[derive(Clone, Debug, PartialEq)]
pub enum SivfObject {

    // complex:
    Layer(Layer),

    // objects:
    Circle(Circle),
    Square(Square),

}

impl SivfObject {

    pub fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        match self {
            SivfObject::Layer(layer) => {
                layer.render(image_sizes, render_type)
            }

            SivfObject::Circle(circle) => {
                circle.render(image_sizes, render_type)
            }

            SivfObject::Square(square) => {
                square.render(image_sizes, render_type)
            }

            _ => {
                panic!("dont know how to render this Sivf Object: {:?}", *self);
            }
        }
    }

}
