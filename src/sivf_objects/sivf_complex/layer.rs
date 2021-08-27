//! SIVF Layer

use std::fmt::{self, Display};
use std::str::FromStr;

use crate::sivf_objects::sivf_any_object::SivfObject;
use crate::sivf_misc::blend_types::{BlendTypes, BlendType};
use crate::sivf_misc::canvas::Canvas;
use crate::utils::sizes::ImageSizes;



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

    pub fn new() -> Self {
        /// child.all(sivf_object) have [.render()] is guaranteed by [SivfObject]
        Layer { elements: vec![] }
    }

    pub fn from(children: Vec<LayerElement>) -> Self {
        /// child.all(sivf_object) have [.render()] is guaranteed by [SivfObject]
        Layer { elements: children }
    }

    pub fn push(&mut self, layer_element: LayerElement) {
        self.elements.push(layer_element);
    }
    pub fn extend(&mut self, layer_elements: Vec<LayerElement>) {
        self.elements.extend(layer_elements);
    }

    pub fn get_children(&self) -> Vec<LayerElement> {
        self.elements.clone()
    }

    pub fn render(&self, image_sizes: ImageSizes) -> Canvas {
        struct CurrentRenderingState { pub canvas: Canvas, pub blend_types: BlendTypes }
        // TODO LATER: try different approaches and measure times:
        //   - render all, then blend all
        //   - render one, blend one, repeat
        self.elements.iter().fold(
            CurrentRenderingState { canvas: Canvas::new(image_sizes), blend_types: BlendTypes::new() },
            |mut acc, child| match child {
                LayerElement::BlendTypes(blend_types) => {
                    acc.blend_types = *blend_types;
                    acc
                }
                LayerElement::SivfObject(sivf_object) => {
                    let canvas_child = sivf_object.render(image_sizes);
                    let blend_types: BlendTypes = acc.blend_types;
                    acc.canvas.blend_with(canvas_child, &blend_types);
                    acc
                }
            }
        ).canvas
    }

}



// impl<'de, D> Deserialize for Layer
// where
//     D: Deserializer<'de>
// {
//     fn deserialize(deserializer: D) -> Result<Self, Error> {
//         todo!()
//     }
// }



// mod deserialize {
//     use serde::{Deserializer, Deserialize};
//     use crate::sivf_misc::blend_types::BlendType;
//     use crate::sivf_objects::sivf_complex::layer::{LayerElement, MyVisitor};
//
//
//
//     pub(crate) fn blend_type() -> BlendType {
//         BlendType::Overlap
//     }
//
//
//
//     pub(crate) fn children<'de, D>(deserializer: D) -> Result<Vec<LayerElement>, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         // TODO:
//         let s = "";
//         // let s: Vec<LayerElement> = Deserialize::deserialize(deserializer)?;
//         let s: Vec<LayerElement> = deserializer.deserialize_string(MyVisitor)?.elements;
//         println!("s = {:#?}", s);
//         Ok(vec![])
//     }
//
// }
//
//
//
// impl From<String> for Layer {
//     fn from(string: String) -> Self {
//         todo!()
//     }
// }
//
//
//
// struct VecLayerElements {
//     pub elements: Vec<LayerElement>
// }
//
// struct MyVisitor;
//
// impl<'de> Visitor<'de> for MyVisitor {
//     type Value = VecLayerElements;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         // formatter.write_str("string or map")
//         formatter.write_str("string")
//     }
//
//     fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//     where
//         E: de::Error
//     {
//         Ok(FromStr::from_str(value).unwrap())
//     }
//
//     // fn visit_map<M>(self, visitor: M) -> Result<Self::Value, M::Error>
//     // where
//     //     M: MapAccess<'de>
//     // {
//     //     let tmp = Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))?;
//     //     println!("tmp = {:#?}", tmp);
//     //     // TODO:
//     //     Ok(VecLayerElements{ elements: vec![] })
//     // }
//
// }
//
// impl<'de> Deserialize<'de> for VecLayerElements {
//     fn deserialize<D>(deserializer: D) -> Result<VecLayerElements, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         deserializer.deserialize_any(MyVisitor)
//     }
// }
//
// impl FromStr for VecLayerElements {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // TODO:
//         Ok(VecLayerElements{ elements: vec![] })
//     }
// }
//
// // impl From<Map<String, String>> for Layer {
// //     fn from(_: Map<String, String>) -> Self {
// //         // TODO:
// //         Layer { elements: vec![] }
// //     }
// // }



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::sivf_misc::sivf_serde::deserialize_sivf;
//
//     #[test]
//     fn deserialize() {
//         let string: String = r#"
//
//         "#.to_string();
//         let expected: Layer = Layer::from(vec![
//             BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Overlap))
//         ]);
//         let actual: Layer = deserialize_sivf(string);
//     }
// }
