//! Convert json/yaml to sivf struct

use evalexpr::eval;
use serde_yaml::Value;

use crate::sivf_misc::blend_types::{BlendTypes, BlendType};
use crate::sivf_misc::keywords_and_consts::*;
use crate::sivf_misc::metric_units::MetricUnit;
use crate::sivf_misc::sivf_struct::SivfStruct;
use crate::sivf_objects::sivf_any_object::SivfObject;
use crate::sivf_objects::sivf_complex::layer::{Layer, LayerElement};
use crate::sivf_objects::sivf_shapes::circle::Circle;
use crate::sivf_objects::sivf_shapes::square::Square;
use crate::utils::color::{ARGB, ColorModel, Color};
use crate::utils::sizes::ImageSizes;
use crate::utils::vec2d::Vec2d;



// const SHOW_DESERIALIZATION_PROGRESS: bool = true;
const SHOW_DESERIALIZATION_PROGRESS: bool = false;



// TODO: ? change [Value] to [HashMap<String, String>]
pub fn deserialize_to_sivf_struct(value: &Value) -> SivfStruct {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("------------------------------------------------ deserializing to SIVF STRUCT:");
        println!("{value:#?}");
    }
    if value.get(KW_IMAGE_SIZES).is_none() { panic!("{KW_IMAGE_SIZES} not found in root") }
    // if value.get(KW_COLOR_MODEL).is_none() { panic!("{KW_COLOR_MODEL} not found in root") }
    // if value.get(KW_ROOT_LAYER).is_none() { panic!("{KW_ROOT_LAYER} not found in root") }

    let image_sizes: &Value = value.get(KW_IMAGE_SIZES).unwrap();
    let (w, h): (usize, usize) = (
        image_sizes.as_sequence().unwrap().get(0).unwrap().as_u64().unwrap() as usize,
        image_sizes.as_sequence().unwrap().get(1).unwrap().as_u64().unwrap() as usize
    );

    let argb_value: &Value = &ARGB.to_value();
    let color_model_value: &Value = value.get(KW_COLOR_MODEL).unwrap_or(argb_value);
    let color_model_str: &str = color_model_value.as_str().unwrap();

    // TODO: rewrite, so it works for list of layers
    let root_layer_value = value.get(KW_ROOT_LAYER).unwrap();
    let layer_element: LayerElement = deserialize_to_layer_element(root_layer_value);
    let sivf_object: SivfObject = if let LayerElement::SivfObject(sivf_object) = layer_element { sivf_object } else { panic!() };
    let root_layer: Layer = if let SivfObject::Layer(layer) = sivf_object { layer } else { panic!() };

    SivfStruct {
        image_sizes: ImageSizes::new(w, h),
        color_model: ColorModel::from(color_model_str),
        root_layer
    }
}



trait ExtensionToValue {
    fn to_value(&self) -> Value;
}
impl ExtensionToValue for &str {
    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}
impl ExtensionToValue for String {
    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}



fn deserialize_to_layer_element(value: &Value) -> LayerElement {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("------------------------- deserializing to SIVF OBJECT:");
        println!("{value:#?}");
    }
    match value {
        value if value.is_sequence() => {
            let array = value.as_sequence().unwrap();
            let layer_elements: Vec<LayerElement> = array.iter().fold(vec![],
            |mut acc, el| {
                // println!("-------------");
                // println!("{el:#?}");
                let layer_element: LayerElement = deserialize_to_layer_element(el);
                acc.push(layer_element);
                acc
            });
            let layer: Layer = Layer::from(layer_elements);
            LayerElement::SivfObject(SivfObject::Layer(layer))
        }
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();

            let _key_layer   : &Value = &KW_LAYER.to_value();
            let key_blending: &Value = &KW_BLENDING.to_value();
            let key_circle  : &Value = &KW_CIRCLE.to_value();
            let key_square  : &Value = &KW_SQUARE.to_value();

            match map {
                map if map.contains_key(key_blending) => {
                    let value = map.get(key_blending).unwrap();
                    let blend_types: BlendTypes = deserialize_to_blend_types(value);
                    LayerElement::BlendTypes(blend_types)
                }
                // TODO: does it belongs here?
                // map if map.contains_key(key_layer) => {
                //     let value = map.get(key_layer).unwrap();
                //     let layer: LayerElement = deserialize_to_layer_element(value);
                //     LayerElement::SivfObject()
                // }
                map if map.contains_key(key_circle) => {
                    let value = map.get(key_circle).unwrap();
                    let circle: Circle = deserialize_to_circle(value);
                    LayerElement::SivfObject(SivfObject::Circle(circle))
                }
                map if map.contains_key(key_square) => {
                    let value = map.get(key_square).unwrap();
                    let square: Square = deserialize_to_square(value);
                    LayerElement::SivfObject(SivfObject::Square(square))
                }
                _ => {
                    // TODO: create list of all KW and search for similar, and if so, show it
                    println!("------");
                    println!("found unknown structure: {map:#?}");
                    let unknown_thing_name = map.iter().next().unwrap().0.as_str().unwrap();
                    todo!("{unknown_thing_name}")
                }
            }
        }
        _ => {
            panic!("unknown option: {value:#?}")
        }
    }

}



fn deserialize_to_circle(value: &Value) -> Circle {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("-------- deserializing to CIRCLE:");
        println!("{value:#?}");
    }
    match value {
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();
            let value_false = Value::Bool(false);
            Circle::new(
                deserialize_to_vec2d_metric_unit(map.get(&KW_XY.to_value()).unwrap()),
                deserialize_to_metric_units(map.get(&KW_CIRCLE_RADIUS.to_value()).unwrap()),
                deserialize_to_color(map.get(&KW_COLOR.to_value()).unwrap()),
                map.get(&KW_INVERSE.to_value()).unwrap_or(&value_false).as_bool().unwrap()
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_square(value: &Value) -> Square {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("-------- deserializing to SQUARE:");
        println!("{value:#?}");
    }
    match value {
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();
            let value_false = Value::Bool(false);
            Square::new(
                deserialize_to_vec2d_metric_unit(map.get(&KW_XY.to_value()).unwrap()),
                deserialize_to_metric_units(map.get(&KW_SQUARE_SIDE.to_value()).unwrap()),
                deserialize_to_color(map.get(&KW_COLOR.to_value()).unwrap()),
                map.get(&KW_INVERSE.to_value()).unwrap_or(&value_false).as_bool().unwrap()
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_blend_types(value: &Value) -> BlendTypes {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("-------- deserializing to BLEND TYPES:");
        println!("{value:#?}");
    }

    trait ExtensionToBlendType {
        fn to_blend_type(&self) -> BlendType;
    }
    impl ExtensionToBlendType for &str {
        fn to_blend_type(&self) -> BlendType {
            match self {
                &KW_BLENDING_TYPE_OVERLAP => { BlendType::Overlap }
                &KW_BLENDING_TYPE_ADD => { BlendType::Add }
                &KW_BLENDING_TYPE_MAX => { BlendType::Max }
                &KW_BLENDING_TYPE_MIN => { BlendType::Min }
                &KW_BLENDING_TYPE_AVG => { BlendType::Avg }
                _ => { panic!() }
            }
        }
    }

    match value {
        value if value.is_sequence() => {
            let array = value.as_sequence().unwrap();
            assert_eq!(None, array.get(2));
            BlendTypes::from(
                array.get(0).unwrap().as_str().unwrap().to_blend_type(),
                array.get(1).unwrap().as_str().unwrap().to_blend_type()
            )
        }
        _ => {
            panic!()
        }
    }
}



fn deserialize_to_vec2d_metric_unit(value: &Value) -> Vec2d<MetricUnit> {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("-------- deserializing to POSITION:");
        println!("{value:#?}");
    }
    match value {
        value if value.is_sequence() => {
            let array = value.as_sequence().unwrap();
            assert_eq!(None, array.get(2));
            Vec2d::new(
                deserialize_to_metric_units(array.get(0).unwrap()),
                deserialize_to_metric_units(array.get(1).unwrap()),
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_metric_units(value: &Value) -> MetricUnit {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("-------- deserializing to METRIC UNITS:");
        println!("{value:#?}");
    }
    trait ExtensionToF64 {
        fn to_f64(&self) -> f64;
    }
    impl ExtensionToF64 for Value {
        fn to_f64(&self) -> f64 {
            self.as_f64().unwrap_or(self.as_i64().unwrap() as f64)
        }
    }
    impl ExtensionToF64 for evalexpr::Value {
        fn to_f64(&self) -> f64 {
            self.as_float().unwrap_or(self.as_int().unwrap() as f64)
        }
    }
    match value {
        value if value.is_number() => {
            let number = value.to_f64();
            MetricUnit::Pixels(number)
        }
        value if value.is_string() => {
            let str = value.as_str().unwrap().trim();
            if str.ends_with('%') {
                let percents_str = &str[0..str.len()-1];
                // TODO:
                // assert!(str.count('%') == 1 && str.ends_with('%'));
                // todo!("eval")
                let percents_number = eval(percents_str).unwrap();
                MetricUnit::Percents(percents_number.to_f64())
            }
            else {
                let result = eval(str).unwrap();
                MetricUnit::Pixels(result.to_f64())
            }
        }
        _ => {
            panic!()
        }
    }
    // Err("".to_string())
}



fn deserialize_to_color(value: &Value) -> Color {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("-------- deserializing to METRIC UNITS:");
        println!("{value:#?}");
    }
    match value {
        value if value.is_string() => {
            let str = value.as_str().unwrap();
            assert_eq!(8, str.len());
            Color::from(str)
        }
        _ => { panic!() }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::color::{ColorModel, Color};
    use crate::utils::sizes::ImageSizes;
    use crate::utils::vec2d::Vec2d;
    use crate::sivf_misc::blend_types::{BlendTypes, BlendType};
    use crate::sivf_misc::metric_units::MetricUnit;
    use crate::sivf_objects::sivf_complex::layer::{Layer, LayerElement};
    use crate::sivf_objects::sivf_shapes::circle::Circle;

    // TODO: write a lot of tests

    #[test]
    fn minimal() {
        {
            let str: String = r#"
                image_sizes: [3840, 2160]
                color_model: ARGB
                root_layer:
                    - blending: [overlap, overlap]
            "#.to_string();
            let expected: SivfStruct = SivfStruct {
                image_sizes: ImageSizes::new(3840, 2160),
                color_model: ColorModel::ARGB,
                root_layer: Layer::from(vec![
                    LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Overlap)),
                ])
            };
            let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&str).unwrap());
            assert_eq!(expected, actual);
        }
        {
            let str: String = r#"
                image_sizes: [3840, 2160]
                color_model: RGBA
                root_layer:
                    - blending: [overlap, overlap]
            "#.to_string();
            let expected: SivfStruct = SivfStruct {
                image_sizes: ImageSizes::new(3840, 2160),
                color_model: ColorModel::RGBA,
                root_layer: Layer::from(vec![
                    LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Overlap)),
                ])
            };
            let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&str).unwrap());
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn circle() {
        let str: String = r#"
            image_sizes: [3840, 2160]
            color_model: ARGB
            root_layer:
                - blending: [overlap, overlap]
                - circle:
                    xy: [0, 0]
                    r: 1984
                    color: ff112233
        "#.to_string();
        let expected: SivfStruct = SivfStruct {
            image_sizes: ImageSizes::new(3840, 2160),
            color_model: ColorModel::ARGB,
            root_layer: Layer::from(vec![
                LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Overlap)),
                LayerElement::SivfObject(SivfObject::Circle(Circle::new(
                    Vec2d::new(MetricUnit::Pixels(0.0), MetricUnit::Pixels(0.0)),
                    MetricUnit::Pixels(1984.0),
                    Color::new(0xff, 0x11, 0x22, 0x33),
                    false
                ))),
            ])
        };
        let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&str).unwrap());
        assert_eq!(expected, actual);
    }

}
