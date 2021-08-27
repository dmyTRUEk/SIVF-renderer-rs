//! used for array of SIVF items

use crate::sivf_objects::sivf_complex::layer::Layer;
use crate::sivf_objects::sivf_shapes::circle::Circle;
use crate::utils::sizes::ImageSizes;
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::trait_render::TraitRender;



#[derive(Clone, Debug, PartialEq)]
pub enum SivfObject {

    // complex:
    Layer(Layer),

    // objects:
    Circle(Circle),

}

impl SivfObject {

    pub fn render(&self, image_sizes: ImageSizes) -> Canvas {
        match self {
            SivfObject::Layer(layer) => {
                layer.render(image_sizes)
            }

            SivfObject::Circle(circle) => {
                circle.render(image_sizes)
            }

            _ => {
                panic!("unknown Sivf Object: {:?}", *self);
            }
        }
    }

}
