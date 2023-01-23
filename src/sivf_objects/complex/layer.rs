//! SIVF Layer

use crate::{
    sivf_misc::{
        blending::BlendTypes,
        canvas::Canvas,
        render::{RenderType, Render},
        sizes::ImageSizes,
    },
    sivf_objects::sivf_object::SivfObject,
};



#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    elements: Vec<LayerElement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LayerElement {
    BlendTypes(BlendTypes),
    SivfObject(SivfObject),
}

impl Layer {

    // pub const fn new() -> Self {
    //     /// child.all(sivf_object) have [.render()] is guaranteed by [SivfObject]
    //     Layer { elements: vec![] }
    // }

    /// child.all(sivf_object) have [.render()] is guaranteed by [SivfObject]
    pub const fn from(children: Vec<LayerElement>) -> Self {
        Layer { elements: children }
    }

    // pub fn push(&mut self, layer_element: LayerElement) {
    //     self.elements.push(layer_element);
    // }

    // pub fn extend(&mut self, layer_elements: Vec<LayerElement>) {
    //     self.elements.extend(layer_elements);
    // }

    // pub fn get_children(&self) -> Vec<LayerElement> {
    //     self.elements.clone()
    // }

}



impl Render for Layer {
    fn render(&self, image_sizes: ImageSizes, render_type: RenderType) -> Canvas {
        #[derive(Clone, Debug)]
        struct RenderingState { pub canvas: Canvas, pub blend_types: BlendTypes }
        // TODO LATER: try different approaches and measure times:
        //   - render all, then blend all
        //   - render one, blend one, repeat
        self.elements.iter().fold(
            RenderingState { canvas: Canvas::new(image_sizes), blend_types: BlendTypes::overlap() },
            |mut acc, child| match child {
                LayerElement::BlendTypes(blend_types) => {
                    // println!("layer.render.fold.BlendTypes:");
                    acc.blend_types = *blend_types;
                    // println!("acc = {acc:?}");
                    acc
                }
                LayerElement::SivfObject(sivf_object) => {
                    println!("layer.render.fold.SivfObject: {sivf_object:?}");
                    // TODO?: rewrite using `measure_time`
                    let render_time_start = chrono::Local::now();
                    let canvas_child = sivf_object.render(image_sizes, render_type);
                    let render_time_end = chrono::Local::now();
                    let render_time = render_time_end - render_time_start;
                    println!("finished render in {s}s {ms}ms. ", s=render_time.num_seconds(), ms=render_time.num_milliseconds() % 1000);

                    let blend_types: BlendTypes = acc.blend_types;

                    // TODO?: rewrite using `measure_time`
                    let render_time_start = chrono::Local::now();
                    acc.canvas.blend_with(&canvas_child, &blend_types, &render_type);
                    let render_time_end = chrono::Local::now();
                    let render_time = render_time_end - render_time_start;
                    println!("finished blend  in {s}s {ms}ms. ", s=render_time.num_seconds(), ms=render_time.num_milliseconds() % 1000);
                    // println!("acc = {acc:?}");
                    acc
                }
            }
        ).canvas
    }
}

