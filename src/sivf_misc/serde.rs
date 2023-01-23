//! Convert json/yaml to sivf struct

use std::collections::HashMap;

use serde_yaml::Value;

use crate::{
    sivf_misc::{
        blending::{BlendTypes, BlendType},
        color::{ARGB, Color, ColorModel},
        keywords_and_consts::*,
        metric_units::{MetricUnit, Axis},
        simple_expr_eval::{eval_expr, eval_expr_with_vals},
        sivf_struct::SivfStruct,
        sizes::ImageSizes,
        vals::Vals,
    },
    sivf_objects::{
        complex::{
            gradient::{Gradient, GradientPoint},
            layer::{Layer, LayerElement},
        },
        shapes::{
            circle::Circle,
            rectangle::Rectangle,
            square::Square,
            triangle::Triangle,
        },
        sivf_object::SivfObject,
    },
    utils::{
        extensions::{
            str::{ExtensionCountChars, ExtensionsSplitOutsideBrackets},
            vec::ExtensionCollectToVec,
        },
        vec2d::Vec2d,
    },
};



const SHOW_DESERIALIZATION_PROGRESS: bool = true;
// const SHOW_DESERIALIZATION_PROGRESS: bool = false;



// TODO(refactor): move all deserialize funcs to their corresponding files.
pub fn deserialize_to_sivf_struct(value: &Value) -> SivfStruct {
    if SHOW_DESERIALIZATION_PROGRESS {
        // TODO(refactor): extract `"-".repeat(N)` to function.
        println!();
        println!("{} deserializing to SIVF STRUCT:", "-".repeat(48));
        println!("{value:#?}");
    }
    let image_sizes = deserialize_to_image_sizes(value);
    let color_model = deserialize_to_color_model(value);
    let vals = deserialize_to_vals(value);

    // TODO: rewrite, so it works for list of layers (without root layer?).
    let root_layer_value = value.get(KW_ROOT_LAYER).expect(&format!("{KW_ROOT_LAYER} not found in root"));
    let layer_element: LayerElement = deserialize_to_layer_element(root_layer_value, Some(&vals));
    let sivf_object: SivfObject =
        if let LayerElement::SivfObject(sivf_object) =
            layer_element { sivf_object } else { panic!() };
    let root_layer: Layer = if let SivfObject::Layer(layer) = sivf_object { layer } else { panic!() };

    SivfStruct {
        image_sizes,
        color_model,
        vals,
        root_layer,
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



fn deserialize_to_image_sizes(value: &Value) -> ImageSizes {
    let image_sizes: &Value = value.get(KW_IMAGE_SIZES).expect(&format!("{KW_IMAGE_SIZES} not found in root"));
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to IMAGE SIZES:", "-".repeat(25));
        println!("{image_sizes:#?}");
    }
    let (w, h): (usize, usize) = (
        image_sizes.as_sequence().unwrap().get(0).unwrap().as_u64().unwrap() as usize,
        image_sizes.as_sequence().unwrap().get(1).unwrap().as_u64().unwrap() as usize
    );
    ImageSizes::new(w, h)
}


fn deserialize_to_color_model(value: &Value) -> ColorModel {
    // if value.get(KW_COLOR_MODEL).is_none() { panic!("{KW_COLOR_MODEL} not found in root") }
    let color_model: &Value = &ARGB.to_value();
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to COLOR MODEL:", "-".repeat(25));
        println!("{color_model:#?}");
    }
    let color_model_value: &Value = value.get(KW_COLOR_MODEL).unwrap_or(color_model);
    let color_model_str: &str = color_model_value.as_str().unwrap();
    ColorModel::from(color_model_str)
}


fn deserialize_to_vals(value: &Value) -> Vals {
    let value_vals = value.get(KW_VALS);
    let vals = match value_vals {
        None => { Vec::new() }
        Some(value_vals) => {
            if SHOW_DESERIALIZATION_PROGRESS {
                println!("{} deserializing to VALS:", "-".repeat(25));
                println!("{value_vals:#?}");
            }
            let value_vals_as_seq = value_vals.as_sequence().unwrap();
            value_vals_as_seq.iter().fold(Vec::with_capacity(value_vals_as_seq.len()),
                |mut acc, el| {
                    let key_value = el.as_mapping().unwrap();

                    let key: String = {
                        let mut keys = key_value.keys();
                        assert_eq!(1, keys.len());
                        keys.next().unwrap().as_str().unwrap().to_string()
                    };

                    let val = {
                        let mut values = key_value.values();
                        assert_eq!(1, values.len());
                        let val = values.next().unwrap().clone();
                        eval_expr_with_vals(
                            if let Some(val) = val.as_str() {
                                val.to_string()
                            } else {
                                val.to_f64().to_string()
                            },
                            Some(&acc)
                        )
                    };

                    acc.push((key, val));
                    acc
                }
            )
        }
    };
    let mut vals = vals.iter().collect_to_vec();
    vals.sort_by(|l, r| r.0.len().cmp(&l.0.len()));
    let vals: Vals = vals.iter().map(|&kv| kv.clone()).collect_to_vec();
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("vals: {vals:?}");
    }
    vals
}



// TODO(refactor): remove all `panic!` and use `Option`+`.unwrap()` instead.
fn deserialize_to_layer_element(value: &Value, vals: Option<&Vals>) -> LayerElement {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to LAYER ELEMENT:", "-".repeat(25));
        println!("{value:#?}");
    }
    match value {
        value if value.is_sequence() => {
            let array = value.as_sequence().unwrap();
            let layer_elements: Vec<LayerElement> = array.iter().fold(vec![],
            |mut acc, el| {
                // println!("{}", "-".repeat(12));
                // println!("{el:#?}");
                let layer_element: LayerElement = deserialize_to_layer_element(el, vals);
                acc.push(layer_element);
                acc
            });
            let layer: Layer = Layer::from(layer_elements);
            LayerElement::SivfObject(SivfObject::Layer(layer))
        }
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();

            let _key_layer   : &Value = &KW_LAYER.to_value();
            let key_gradient : &Value = &KW_GRADIENT.to_value();
            let key_blending : &Value = &KW_BLENDING.to_value();
            let key_circle   : &Value = &KW_CIRCLE.to_value();
            let key_rectangle: &Value = &KW_RECTANGLE.to_value();
            let key_square   : &Value = &KW_SQUARE.to_value();
            let key_triangle : &Value = &KW_TRIANGLE.to_value();

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
                    let circle: Circle = deserialize_to_circle(value, vals);
                    LayerElement::SivfObject(SivfObject::Circle(circle))
                }
                map if map.contains_key(key_rectangle) => {
                    let value = map.get(key_rectangle).unwrap();
                    let rectangle: Rectangle = deserialize_to_rectangle(value, vals);
                    LayerElement::SivfObject(SivfObject::Rectangle(rectangle))
                }
                map if map.contains_key(key_square) => {
                    let value = map.get(key_square).unwrap();
                    let square: Square = deserialize_to_square(value, vals);
                    LayerElement::SivfObject(SivfObject::Square(square))
                }
                map if map.contains_key(key_triangle) => {
                    let value = map.get(key_triangle).unwrap();
                    let triangle: Triangle = deserialize_to_triangle(value, vals);
                    LayerElement::SivfObject(SivfObject::Triangle(triangle))
                }
                map if map.contains_key(key_gradient) => {
                    let value = map.get(key_gradient).unwrap();
                    let gradient: Gradient = deserialize_to_gradient(value, vals);
                    LayerElement::SivfObject(SivfObject::Gradient(gradient))
                }
                _ => {
                    // TODO: create list of all KW and search for similar, and if so, show it.
                    println!("{}", "-".repeat(6));
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



const VALUE_TRUE : &Value = &Value::Bool(true);
const VALUE_FALSE: &Value = &Value::Bool(false);


fn deserialize_to_color(value: &Value) -> Color {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to COLOR:", "-".repeat(8));
        println!("{value:#?}");
    }
    let res = match value {
        value if value.is_string() || value.is_u64() => {
            let s: &str = &match value {
                value if value.is_string() => { value.as_str().unwrap().to_string() }
                value if value.is_u64() => { value.as_u64().unwrap().to_string() }
                _ => { unimplemented!() }
            };
            if s.count_chars(',') == 0 {
                assert_eq!(8, s.len());
                Color::from(s)
            }
            else {
                let parts: Vec<String> = s.split_outside_brackets(',', '(', ')');
                assert_eq!(4, parts.len());
                Color::new(
                    (eval_expr(&parts[0])).max(0_f64).min(255_f64) as u8,
                    (eval_expr(&parts[1])).max(0_f64).min(255_f64) as u8,
                    (eval_expr(&parts[2])).max(0_f64).min(255_f64) as u8,
                    (eval_expr(&parts[3])).max(0_f64).min(255_f64) as u8,
                )
            }
        }
        _ => { panic!() }
    };
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("res: {res:?}");
    }
    res
}



fn deserialize_to_blend_types(value: &Value) -> BlendTypes {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to BLEND TYPES:", "-".repeat(8));
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
        _ => { panic!() }
    }
}



trait ExtToF64 {
    fn to_f64(&self) -> f64;
}
impl ExtToF64 for Value {
    fn to_f64(&self) -> f64 {
        self.as_f64().unwrap_or_else(|| self.as_i64().unwrap() as f64)
    }
}
trait ExtToF64Safe {
    fn to_f64_safe(&self) -> Option<f64>;
}
impl ExtToF64Safe for Value {
    fn to_f64_safe(&self) -> Option<f64> {
        match self.as_f64() {
            Some(x) => { Some(x) }
            None => {
                self.as_i64().map(|x| x as f64)
            }
        }
    }
}

fn deserialize_to_metric_unit(value: &Value, vals: Option<&Vals>) -> MetricUnit {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to METRIC UNITS:", "-".repeat(8));
        println!("{value:#?}");
    }
    let res = match value {
        value if value.is_number() => {
            let number: f64 = value.to_f64();
            MetricUnit::Pixels(number)
        }
        value if value.is_string() => {
            let s: &str = value.as_str().unwrap().trim();
            match s {
                s if s.ends_with('%') => {
                    assert!(s.count_chars('%') == 1);
                    let percents_str: &str = &s[0..s.len()-1];
                    if SHOW_DESERIALIZATION_PROGRESS {
                        println!("{} STARTING EVAL: `{percents_str}`", "-".repeat(5));
                    }
                    let percents_number: f64 = eval_expr_with_vals(percents_str, vals);
                    MetricUnit::Percents(percents_number, None)
                }
                s if s.ends_with("%x") || s.ends_with("%w") => {
                    assert!(s.count_chars('%') == 1);
                    let percents_str: &str = &s[0..s.len()-2];
                    if SHOW_DESERIALIZATION_PROGRESS {
                        println!("{} STARTING EVAL: `{percents_str}`", "-".repeat(5));
                    }
                    let percents_number: f64 = eval_expr_with_vals(percents_str, vals);
                    MetricUnit::Percents(percents_number, Some(Axis::X))
                }
                s if s.ends_with("%y") || s.ends_with("%h") => {
                    assert!(s.count_chars('%') == 1);
                    let percents_str: &str = &s[0..s.len()-2];
                    if SHOW_DESERIALIZATION_PROGRESS {
                        println!("{} STARTING EVAL: `{percents_str}`", "-".repeat(5));
                    }
                    let percents_number: f64 = eval_expr_with_vals(percents_str, vals);
                    MetricUnit::Percents(percents_number, Some(Axis::Y))
                }
                _ => {
                    let result: f64 = eval_expr_with_vals(s, vals);
                    MetricUnit::Pixels(result)
                }
            }
        }
        _ => { panic!() }
    };
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("res: {res:?}");
    }
    res
}



fn deserialize_to_vec2d_metric_unit(value: &Value, vals: Option<&Vals>) -> Vec2d<MetricUnit> {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to POSITION (Vec2d of MetricUnit):", "-".repeat(8));
        println!("{value:#?}");
    }
    match value {
        value if value.is_sequence() => {
            let array = value.as_sequence().unwrap();
            assert_eq!(None, array.get(2));
            Vec2d::new(
                deserialize_to_metric_unit(array.get(0).unwrap(), vals),
                deserialize_to_metric_unit(array.get(1).unwrap(), vals),
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_circle(value: &Value, vals: Option<&Vals>) -> Circle {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to CIRCLE:", "-".repeat(8));
        println!("{value:#?}");
    }
    match value {
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();
            Circle::new(
                deserialize_to_vec2d_metric_unit(map.get(&KW_XY.to_value()).unwrap(), vals),
                deserialize_to_metric_unit(map.get(&KW_CIRCLE_RADIUS.to_value()).unwrap(), vals),
                deserialize_to_color(map.get(&KW_COLOR.to_value()).unwrap()),
                map.get(&KW_INVERSE.to_value()).unwrap_or(&VALUE_FALSE).as_bool().unwrap()
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_rectangle(value: &Value, vals: Option<&Vals>) -> Rectangle {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to RECTANGLE:", "-".repeat(8));
        println!("{value:#?}");
    }
    match value {
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();
            Rectangle::new(
                deserialize_to_vec2d_metric_unit(map.get(&KW_XY.to_value()).unwrap(), vals),
                deserialize_to_vec2d_metric_unit(map.get(&KW_RECTANGLE_WH.to_value()).unwrap(), vals),
                deserialize_to_color(map.get(&KW_COLOR.to_value()).unwrap()),
                map.get(&KW_INVERSE.to_value()).unwrap_or(&VALUE_FALSE).as_bool().unwrap()
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_square(value: &Value, vals: Option<&Vals>) -> Square {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to SQUARE:", "-".repeat(8));
        println!("{value:#?}");
    }
    match value {
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();
            Square::new(
                deserialize_to_vec2d_metric_unit(map.get(&KW_XY.to_value()).unwrap(), vals),
                deserialize_to_metric_unit(map.get(&KW_SQUARE_SIDE.to_value()).unwrap(), vals),
                deserialize_to_color(map.get(&KW_COLOR.to_value()).unwrap()),
                map.get(&KW_INVERSE.to_value()).unwrap_or(&VALUE_FALSE).as_bool().unwrap()
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_triangle(value: &Value, vals: Option<&Vals>) -> Triangle {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to TRIANGLE:", "-".repeat(8));
        println!("{value:#?}");
    }
    match value {
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();
            Triangle::new(
                deserialize_to_vec2d_metric_unit(map.get(&KW_TRIANGLE_P1.to_value()).unwrap(), vals),
                deserialize_to_vec2d_metric_unit(map.get(&KW_TRIANGLE_P2.to_value()).unwrap(), vals),
                deserialize_to_vec2d_metric_unit(map.get(&KW_TRIANGLE_P3.to_value()).unwrap(), vals),
                deserialize_to_color(map.get(&KW_COLOR.to_value()).unwrap()),
                map.get(&KW_INVERSE.to_value()).unwrap_or(&VALUE_FALSE).as_bool().unwrap()
            )
        }
        _ => { panic!() }
    }
}



fn deserialize_to_gradient(value: &Value, vals: Option<&Vals>) -> Gradient {
    if SHOW_DESERIALIZATION_PROGRESS {
        println!("{} deserializing to GRADIENT:", "-".repeat(8));
        println!("{value:#?}");
    }
    match value {
        value if value.is_mapping() => {
            let map = value.as_mapping().unwrap();
            let points: Vec<GradientPoint<MetricUnit>> = map
                .get(&KW_GRADIENT_POINTS.to_value()).unwrap()
                .as_sequence().unwrap()
                .chunks_exact(3).collect_to_vec()
                // TODO: assert remainder is zero.
                .iter()
                .map(|p|(
                    deserialize_to_vec2d_metric_unit(&p[0], vals),
                    deserialize_to_color(&p[1]),
                    deserialize_to_metric_unit(&p[2], vals)
                ))
                .map(|(pos, color, sigma)| GradientPoint::new(pos, sigma, color))
                .collect();
            Gradient::new(
                points,
                deserialize_to_color(map.get(&KW_COLOR.to_value()).unwrap()),
                map.get(&KW_GRADIENT_IS_FADING.to_value()).unwrap_or(&VALUE_TRUE).as_bool().unwrap()
            )
        }
        _ => { panic!() }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        sivf_misc::{
            blending::{BlendTypes, BlendType},
            color::{ColorModel, Color},
            metric_units::MetricUnit,
            sizes::ImageSizes,
        },
        sivf_objects::{
            complex::layer::{Layer, LayerElement},
            shapes::circle::Circle,
        },
        utils::vec2d::Vec2d,
    };

    // TODO(tests): write A LOT of tests

    #[test]
    fn deserialize_to_metric_units_pixels() {
        let test_cases: Vec<(MetricUnit, &str)> = vec![
            (MetricUnit::Pixels(0.0), "0"),
            (MetricUnit::Pixels(7.0), "7"),
            (MetricUnit::Pixels(7.0), "7.0"),
            (MetricUnit::Pixels(7.0), "7."),
            (MetricUnit::Pixels(0.7), ".7"),
            (MetricUnit::Pixels(7.645), "7.645"),
            (MetricUnit::Pixels(4.7), "1.2+3.5"),
            (MetricUnit::Pixels(1.4142135623730951), "sqrt(2)"),
            (MetricUnit::Pixels(8.122417494872465), "sqrt(2)+3*sqrt(5)"),
            (MetricUnit::Pixels(8.122417494872465), "(sqrt(2)+3*sqrt(5))"),
            (MetricUnit::Pixels(8.122417494872465), "((((((((((sqrt(((((2))))))))+((((3))))*sqrt(((((5))))))))))))"),
        ];
        for (ans, input) in test_cases {
            assert_eq!(ans, deserialize_to_metric_unit(&input.to_value(), None));
        }
    }

    #[test]
    fn deserialize_to_metric_units_percents() {
        let test_cases: Vec<(MetricUnit, &str)> = vec![
            (MetricUnit::Percents(0.0, None), "0%"),
            (MetricUnit::Percents(7.0, None), "7%"),
            (MetricUnit::Percents(7.0, None), "7.0%"),
            (MetricUnit::Percents(7.654, None), "7.654%"),
            (MetricUnit::Percents(4.7, None), "(1.2+3.5)%"),
            (MetricUnit::Percents(1.4142135623730951, None), "sqrt(2)%"),
            (MetricUnit::Percents(8.122417494872465, None), "sqrt(2)+3*sqrt(5)%"),
            (MetricUnit::Percents(8.122417494872465, None), "(sqrt(2)+3*sqrt(5))%"),
            (MetricUnit::Percents(8.122417494872465, None), "((((((((((sqrt(((((2))))))))+((((3))))*sqrt(((((5))))))))))))%"),
            (MetricUnit::Percents(0.0, Some(Axis::X)), "0%x"),
            (MetricUnit::Percents(7.0, Some(Axis::X)), "7%x"),
            (MetricUnit::Percents(7.0, Some(Axis::X)), "7.0%x"),
            (MetricUnit::Percents(7.654, Some(Axis::X)), "7.654%x"),
            (MetricUnit::Percents(4.7, Some(Axis::X)), "(1.2+3.5)%x"),
            (MetricUnit::Percents(1.4142135623730951, Some(Axis::X)), "sqrt(2)%x"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::X)), "sqrt(2)+3*sqrt(5)%x"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::X)), "(sqrt(2)+3*sqrt(5))%x"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::X)), "((((((((((sqrt(((((2))))))))+((((3))))*sqrt(((((5))))))))))))%x"),
            (MetricUnit::Percents(0.0, Some(Axis::X)), "0%w"),
            (MetricUnit::Percents(7.0, Some(Axis::X)), "7%w"),
            (MetricUnit::Percents(7.0, Some(Axis::X)), "7.0%w"),
            (MetricUnit::Percents(7.654, Some(Axis::X)), "7.654%w"),
            (MetricUnit::Percents(4.7, Some(Axis::X)), "(1.2+3.5)%w"),
            (MetricUnit::Percents(1.4142135623730951, Some(Axis::X)), "sqrt(2)%w"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::X)), "sqrt(2)+3*sqrt(5)%w"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::X)), "(sqrt(2)+3*sqrt(5))%w"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::X)), "((((((((((sqrt(((((2))))))))+((((3))))*sqrt(((((5))))))))))))%w"),
            (MetricUnit::Percents(0.0, Some(Axis::Y)), "0%y"),
            (MetricUnit::Percents(7.0, Some(Axis::Y)), "7%y"),
            (MetricUnit::Percents(7.0, Some(Axis::Y)), "7.0%y"),
            (MetricUnit::Percents(7.654, Some(Axis::Y)), "7.654%y"),
            (MetricUnit::Percents(4.7, Some(Axis::Y)), "(1.2+3.5)%y"),
            (MetricUnit::Percents(1.4142135623730951, Some(Axis::Y)), "sqrt(2)%y"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::Y)), "sqrt(2)+3*sqrt(5)%y"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::Y)), "(sqrt(2)+3*sqrt(5))%y"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::Y)), "((((((((((sqrt(((((2))))))))+((((3))))*sqrt(((((5))))))))))))%y"),
            (MetricUnit::Percents(0.0, Some(Axis::Y)), "0%h"),
            (MetricUnit::Percents(7.0, Some(Axis::Y)), "7%h"),
            (MetricUnit::Percents(7.0, Some(Axis::Y)), "7.0%h"),
            (MetricUnit::Percents(7.654, Some(Axis::Y)), "7.654%h"),
            (MetricUnit::Percents(4.7, Some(Axis::Y)), "(1.2+3.5)%h"),
            (MetricUnit::Percents(1.4142135623730951, Some(Axis::Y)), "sqrt(2)%h"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::Y)), "sqrt(2)+3*sqrt(5)%h"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::Y)), "(sqrt(2)+3*sqrt(5))%h"),
            (MetricUnit::Percents(8.122417494872465, Some(Axis::Y)), "((((((((((sqrt(((((2))))))))+((((3))))*sqrt(((((5))))))))))))%h"),
        ];
        for (ans, input) in test_cases {
            assert_eq!(ans, deserialize_to_metric_unit(&input.to_value(), None));
        }
    }

    #[test]
    fn minimal() {
        {
            let s: String = r#"
                image_sizes: [3840, 2160]
                color_model: ARGB
                root_layer:
                  - blending: [overlap, add]
            "#.to_string();
            let expected: SivfStruct = SivfStruct {
                image_sizes: ImageSizes::new(3840, 2160),
                color_model: ColorModel::ARGB,
                vals: Vec::new(),
                root_layer: Layer::from(vec![
                    LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Add)),
                ])
            };
            let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&s).unwrap());
            assert_eq!(expected, actual);
        }
        {
            let s: String = r#"
                image_sizes: [3840, 2160]
                color_model: RGBA
                root_layer:
                  - blending: [overlap, add]
            "#.to_string();
            let expected: SivfStruct = SivfStruct {
                image_sizes: ImageSizes::new(3840, 2160),
                color_model: ColorModel::RGBA,
                vals: Vec::new(),
                root_layer: Layer::from(vec![
                    LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Add)),
                ])
            };
            let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&s).unwrap());
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn circle() {
        let s: String = r#"
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
            vals: Vec::new(),
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
        let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&s).unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn rectangle() {
        let s: String = r#"
            image_sizes: [3840, 2160]
            color_model: ARGB
            root_layer:
              - blending: [overlap, overlap]
              - rectangle:
                  xy: [0, 0]
                  wh: [2022, 1011]
                  color: ff112233
        "#.to_string();
        let expected: SivfStruct = SivfStruct {
            image_sizes: ImageSizes::new(3840, 2160),
            color_model: ColorModel::ARGB,
            vals: Vec::new(),
            root_layer: Layer::from(vec![
                LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Overlap)),
                LayerElement::SivfObject(SivfObject::Rectangle(Rectangle::new(
                    Vec2d::new(MetricUnit::Pixels(0.0), MetricUnit::Pixels(0.0)),
                    Vec2d::new(MetricUnit::Pixels(2022.0), MetricUnit::Pixels(1011.0)),
                    Color::new(0xff, 0x11, 0x22, 0x33),
                    false
                ))),
            ])
        };
        let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&s).unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn square() {
        let s: String = r#"
            image_sizes: [3840, 2160]
            color_model: ARGB
            root_layer:
              - blending: [overlap, overlap]
              - square:
                  xy: [0, 0]
                  side: 2022
                  color: ff112233
        "#.to_string();
        let expected: SivfStruct = SivfStruct {
            image_sizes: ImageSizes::new(3840, 2160),
            color_model: ColorModel::ARGB,
            vals: Vec::new(),
            root_layer: Layer::from(vec![
                LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Overlap)),
                LayerElement::SivfObject(SivfObject::Square(Square::new(
                    Vec2d::new(MetricUnit::Pixels(0.0), MetricUnit::Pixels(0.0)),
                    MetricUnit::Pixels(2022.0),
                    Color::new(0xff, 0x11, 0x22, 0x33),
                    false
                ))),
            ])
        };
        let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&s).unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn triangle() {
        let s: String = r#"
            image_sizes: [3840, 2160]
            color_model: ARGB
            root_layer:
              - blending: [overlap, overlap]
              - triangle:
                  p1: [-10, -99]
                  p2: [27, "67%"]
                  p3: ["43%", 83]
                  color: ff112233
        "#.to_string();
        let expected: SivfStruct = SivfStruct {
            image_sizes: ImageSizes::new(3840, 2160),
            color_model: ColorModel::ARGB,
            vals: Vec::new(),
            root_layer: Layer::from(vec![
                LayerElement::BlendTypes(BlendTypes::from(BlendType::Overlap, BlendType::Overlap)),
                LayerElement::SivfObject(SivfObject::Triangle(Triangle::new(
                    Vec2d::new(MetricUnit::Pixels(-10.0), MetricUnit::Pixels(-99.0)),
                    Vec2d::new(MetricUnit::Pixels(27.0), MetricUnit::Percents(67.0, None)),
                    Vec2d::new(MetricUnit::Percents(43.0, None), MetricUnit::Pixels(83.0)),
                    Color::new(0xff, 0x11, 0x22, 0x33),
                    false
                ))),
            ])
        };
        let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&s).unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn vals() {
        let s: String = r#"
            image_sizes: [3840, 2160]
            color_model: ARGB
            vals:
              - v1: 145
              - v2: 2.0
              - v3: -0.999
              - v4: v1-187
            root_layer:
              - triangle:
                  p1: ["v1+v2", "v1-v2"]
                  p2: ["v1*v3", "v2%"]
                  p3: ["v2*sin(v3)", "v1*(v2+sin(v2-v3+v1/v4))-v4"]
                  color: ff112233
        "#.to_string();
        let expected: SivfStruct = SivfStruct {
            image_sizes: ImageSizes::new(3840, 2160),
            color_model: ColorModel::ARGB,
            vals: vec![
                ("v1", 145.0),
                ("v2", 2.0),
                ("v3", -0.999),
                ("v4", -42.0),
            ].iter().map(|(k, v)| (k.to_string(), *v)).collect_to_vec(),
            root_layer: Layer::from(vec![
                LayerElement::SivfObject(SivfObject::Triangle(Triangle::new(
                    Vec2d::new(MetricUnit::Pixels(145.0+2.0), MetricUnit::Pixels(145.0-2.0)),
                    Vec2d::new(MetricUnit::Pixels(145.0*-0.999), MetricUnit::Percents(2.0, None)),
                    Vec2d::new(MetricUnit::Pixels(2.0*(-0.999_f64).sin()), MetricUnit::Pixels(145.0*(2.0+(2.0-(-0.999)+145.0/-42.0_f64).sin())-(-42.0))),
                    Color::new(0xff, 0x11, 0x22, 0x33),
                    false
                ))),
            ])
        };
        let actual: SivfStruct = SivfStruct::from(&serde_yaml::from_str(&s).unwrap());
        assert_eq!(expected, actual);
    }

}

