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