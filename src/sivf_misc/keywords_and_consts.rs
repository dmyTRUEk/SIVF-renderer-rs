//! SIVF Keywords and Consts

#![allow(unused)]



// Global const:
pub const KW_TRUE: &str = "true";
pub const KW_FALSE: &str = "false";

// Root properties:
pub const KW_IMAGE_SIZES: &str = "image_sizes";
pub const KW_COLOR_MODEL: &str = "color_model";
pub const KW_ROOT_LAYER: &str = "root_layer";
pub const KW_VARS: &str = "vars";

// Blending and blending types:
// TODO: maybe "blend" and "combine" is the same, so combine them?
pub const KW_BLENDING: &str = "blending";
pub const KW_BLENDING_TYPE_ADD: &str = "add";
pub const KW_BLENDING_TYPE_AVG: &str = "avg";
pub const KW_BLENDING_TYPE_MIN: &str = "min";
pub const KW_BLENDING_TYPE_MAX: &str = "max";
pub const KW_BLENDING_TYPE_OVERLAP: &str = "overlap";

// Global shape properties:
pub const KW_COLOR: &str = "color";
pub const KW_INVERSE: &str = "inverse";
pub const KW_INVERSE_DEFAULT: bool = false;
pub const KW_USED: &str = "used";
pub const KW_USED_DEFAULT: bool = true;
pub const KW_XY: &str = "xy";

// Entities and their custom properties:
pub const KW_CIRCLE: &str = "circle";
pub const KW_CIRCLE_RADIUS: &str = "r";

// Combine:
pub const KW_COMBINE: &str = "combine";
pub const KW_COMBINE_FIGURES: &str = "figures";
pub const KW_COMBINE_TYPE: &str = "type";
pub const KW_COMBINE_TYPE_1PLUS2: &str = "1+2";
pub const KW_COMBINE_TYPE_2PLUS1: &str = "2+1";
pub const KW_COMBINE_TYPE_1MINUS2: &str = "1-2";
pub const KW_COMBINE_TYPE_2MINUS1: &str = "2-1";
pub const KW_COMBINE_TYPE_1PRODUCT2: &str = "1*2";
pub const KW_COMBINE_TYPE_2PRODUCT1: &str = "2*1";
pub const KW_COMBINE_TYPE_1SYMDIFF2: &str = "1%2";    // SymDiff = Symmetric Difference
pub const KW_COMBINE_TYPE_2SYMDIFF1: &str = "2%1";

pub const KW_GRADIENT: &str = "gradient";
pub const KW_GRADIENT_IS_FADING: &str = "fading";
pub const KW_GRADIENT_IS_FADING_DEFAULT: bool = true;
pub const KW_GRADIENT_POINTS: &str = "points";
pub const KW_GRADIENT_SIGMA: &str = "sigma";

pub const KW_LAYER: &str = "layer";
pub const KW_LAYER_DELTA_XY: &str = "delta_xy";

pub const KW_MESH: &str = "mesh";
pub const KW_MESH_N_XLEFT_YDOWN_XRIGHT_YUP: &str = "n_xleft_ydown_xright_yup";

pub const KW_SQUARE: &str = "square";
pub const KW_SQUARE_SIDE: &str = "side";

pub const KW_TRIANGLE: &str = "triangle";
