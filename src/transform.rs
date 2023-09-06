mod layer;
mod ptr;
mod collection;
mod repeat;
pub use layer::*;

use crate::Rubik;

pub trait RubikTransform {
    fn apply_on(&self, rubik: &mut Rubik);
}


impl Rubik {
    pub fn execute(&mut self, operation: impl RubikTransform) -> &mut Self {
        operation.apply_on(self);
        self
    }
}

#[macro_export]
macro_rules! tf {
    ($val: expr) => {
        &$val as &dyn $crate::transform::RubikTransform
    };
    ($val: expr; $times: expr) => {
        (&$val as &dyn $crate::transform::RubikTransform).repeat($times)
    };
}

pub enum RubikTransformGroup {
    Layer(RubikLayerTransform),
    Repeat(Box<Self>, usize),
    Combine(Vec<Self>),
}

impl RubikTransform for RubikTransformGroup {
    fn apply_on(&self, rubik: &mut Rubik) {
        match self {
            RubikTransformGroup::Layer(layer) => layer.apply_on(rubik),
            RubikTransformGroup::Repeat(transform, times) => {
                for _ in 0..*times {
                    transform.apply_on(rubik);
                }
            }
            RubikTransformGroup::Combine(transforms) => {
                for transform in transforms {
                    transform.apply_on(rubik);
                }
            }
        }
    }
}


impl RubikTransformGroup {
    pub fn inverse(self) -> Self {
        match self {
            RubikTransformGroup::Layer(layer) => RubikTransformGroup::Layer(layer.inverse()),
            RubikTransformGroup::Repeat(transform, times) => {
                RubikTransformGroup::Repeat(Box::new(transform.inverse()), times)
            }
            RubikTransformGroup::Combine(transforms) => {
                RubikTransformGroup::Combine(transforms.into_iter().rev().map(|t| t.inverse()).collect())
            }
        }
    }

    pub fn repeat(self, n: usize) -> Self {
        Self::Repeat(Box::new(self), n)
    }

    pub const F: Self = RubikTransformGroup::Layer(RubikLayerTransform::F);
    pub const B: Self = RubikTransformGroup::Layer(RubikLayerTransform::B);
    pub const L: Self = RubikTransformGroup::Layer(RubikLayerTransform::L);
    pub const R: Self = RubikTransformGroup::Layer(RubikLayerTransform::R);
    pub const U: Self = RubikTransformGroup::Layer(RubikLayerTransform::U);
    pub const D: Self = RubikTransformGroup::Layer(RubikLayerTransform::D);
    pub const M: Self = RubikTransformGroup::Layer(RubikLayerTransform::M);
    pub const E: Self = RubikTransformGroup::Layer(RubikLayerTransform::E);
    pub const S: Self = RubikTransformGroup::Layer(RubikLayerTransform::S);
}