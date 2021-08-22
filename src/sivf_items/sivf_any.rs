//! used for array of SIVF items

use crate::sivf_items::sivf_complex::layer::Layer;
use crate::sivf_items::sivf_objects::circle::SivfCircle;



pub enum SivfAny {
    // complex:
    Layer(Layer),

    // objects:
    Circle(SivfCircle),

}
