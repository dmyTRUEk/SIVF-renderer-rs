//! used for array of SIVF items

use serde_derive::{Serialize, Deserialize};

use crate::sivf_objects::sivf_complex::layer::Layer;
use crate::sivf_objects::sivf_shapes::circle::SivfCircle;
use crate::utils::sizes::ImageSizes;
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::trait_render::TraitRender;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SivfObject {
    // complex:
    #[serde(rename="layer")]
    Layer(Layer),

    // objects:
    #[serde(rename="circle")]
    Circle(SivfCircle),

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
