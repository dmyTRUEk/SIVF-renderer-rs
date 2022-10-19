//! used for array of SIVF items

use crate::{
    sivf_objects::{
        complex::{gradient::Gradient, layer::Layer},
        shapes::{
            circle::Circle,
            rectangle::Rectangle,
            square::Square,
            triangle::Triangle,
        },
    },
    sivf_misc::{
        canvas::Canvas,
        render::{RenderType, Render},
    },
    utils::sizes::ImageSizes,
};



#[derive(Clone, Debug, PartialEq)]
pub enum SivfObject {
    // complex:
    Gradient(Gradient),
    Layer(Layer),

    // objects:
    Circle(Circle),
    Rectangle(Rectangle),
    Square(Square),
    Triangle(Triangle),
}

impl Render for SivfObject {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        match self {
            SivfObject::Gradient(gradient) => {
                gradient.render(image_sizes, render_type)
            }
            SivfObject::Layer(layer) => {
                layer.render(image_sizes, render_type)
            }

            SivfObject::Circle(circle) => {
                circle.render(image_sizes, render_type)
            }
            SivfObject::Rectangle(rectangle) => {
                rectangle.render(image_sizes, render_type)
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

