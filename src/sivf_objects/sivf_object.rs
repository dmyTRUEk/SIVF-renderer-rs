//! used for array of SIVF items

use crate::sivf_objects::complex::layer::Layer;
use crate::sivf_objects::shapes::{circle::Circle, square::Square, triangle::Triangle};
use crate::utils::sizes::ImageSizes;
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::render::{RenderType, Render};



#[derive(Clone, Debug, PartialEq)]
pub enum SivfObject {

    // complex:
    Layer(Layer),

    // objects:
    Circle(Circle),
    Square(Square),
    Triangle(Triangle),

}

impl Render for SivfObject {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
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
            SivfObject::Triangle(triangle) => {
                triangle.render(image_sizes, render_type)
            }
        }
    }
}

