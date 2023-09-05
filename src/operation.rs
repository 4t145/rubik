use crate::*;

pub const R: &dyn RubikTransform = &RubikLayerTransform::R;
pub const R_: &dyn RubikTransform = &RubikLayerTransform::R.inverse();
pub const L: &dyn RubikTransform = &RubikLayerTransform::L;
pub const L_: &dyn RubikTransform = &RubikLayerTransform::L.inverse();
pub const U: &dyn RubikTransform = &RubikLayerTransform::U;
pub const U_: &dyn RubikTransform = &RubikLayerTransform::U.inverse();
pub const D: &dyn RubikTransform = &RubikLayerTransform::D;
pub const D_: &dyn RubikTransform = &RubikLayerTransform::D.inverse();
pub const F: &dyn RubikTransform = &RubikLayerTransform::F;
pub const F_: &dyn RubikTransform = &RubikLayerTransform::F.inverse();
pub const B: &dyn RubikTransform = &RubikLayerTransform::B;
pub const B_: &dyn RubikTransform = &RubikLayerTransform::B.inverse();