//! SIVF Layer

// use std::any::Any;
use std::fmt::{self, Display};
use std::str::FromStr;

use serde_derive::{Serialize, Deserialize};
use serde::{Deserializer, Deserialize};
use serde::de::{self, Visitor, MapAccess};

use crate::sivf_objects::sivf_any_object::SivfObject;
use crate::sivf_misc::blend_types::{BlendTypes, BlendType};
use crate::sivf_misc::canvas::Canvas;
use crate::sivf_misc::trait_render::TraitRender;
use crate::utils::sizes::ImageSizes;



#[derive(Clone, Debug, /*Serialize,*/ Deserialize)]
// #[serde(from="Map<String, String>")]
#[serde(transparent)]
pub struct Layer {
    // #[serde(deserialize_with="deserialize::children", flatten)]
    #[serde(flatten)]
    elements: Vec<LayerElement>,
}

#[derive(Clone, Debug, /*Serialize,*/ Deserialize)]
pub enum LayerElement {
    BlendTypes(BlendTypes),
    SivfObject(SivfObject),
}

impl Layer {

    // TODO: try return type [Self]
    pub fn new(children: Vec<LayerElement>) -> Layer {
        // child.all(sivf_object) have [.render()] is guaranteed by [SivfObject]
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
        // TODO LATER: try different approaches and measure times:
        //   - render all, then blend all
        //   - render one, blend one, repeat
        self.elements.iter().fold(
            (Canvas::new(image_sizes), BlendTypes::new()),
            |acc, child| match child {
                LayerElement::BlendTypes(blend_types) => {
                    let canvas: Canvas = acc.0;
                    (canvas, *blend_types)
                }
                LayerElement::SivfObject(sivf_object) => {
                    let blend_types: BlendTypes = acc.1;
                    (acc.0.blend_with(sivf_object.render(image_sizes), blend_types), blend_types)
                }
            }
        ).0
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




mod deserialize {
    use serde::{Deserializer, Deserialize};
    use crate::sivf_misc::blend_types::BlendType;
    use crate::sivf_objects::sivf_complex::layer::{LayerElement, MyVisitor};



    pub(crate) fn blend_type() -> BlendType {
        BlendType::Overlap
    }



    pub(crate) fn children<'de, D>(deserializer: D) -> Result<Vec<LayerElement>, D::Error>
    where
        D: Deserializer<'de>
    {
        // TODO:
        let s = "";
        // let s: Vec<LayerElement> = Deserialize::deserialize(deserializer)?;
        let s: Vec<LayerElement> = deserializer.deserialize_string(MyVisitor)?.elements;
        println!("s = {:#?}", s);
        Ok(vec![])
    }

}



impl From<String> for Layer {
    fn from(string: String) -> Self {
        todo!()
    }
}



struct VecLayerElements {
    pub elements: Vec<LayerElement>
}

struct MyVisitor;

impl<'de> Visitor<'de> for MyVisitor {
    type Value = VecLayerElements;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // formatter.write_str("string or map")
        formatter.write_str("string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(FromStr::from_str(value).unwrap())
    }

    // fn visit_map<M>(self, visitor: M) -> Result<Self::Value, M::Error>
    // where
    //     M: MapAccess<'de>
    // {
    //     let tmp = Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))?;
    //     println!("tmp = {:#?}", tmp);
    //     // TODO:
    //     Ok(VecLayerElements{ elements: vec![] })
    // }

}

impl<'de> Deserialize<'de> for VecLayerElements {
    fn deserialize<D>(deserializer: D) -> Result<VecLayerElements, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_any(MyVisitor)
    }
}

impl FromStr for VecLayerElements {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO:
        Ok(VecLayerElements{ elements: vec![] })
    }
}

// impl From<Map<String, String>> for Layer {
//     fn from(_: Map<String, String>) -> Self {
//         // TODO:
//         Layer { elements: vec![] }
//     }
// }
