mod layer;
pub use layer::*;

use crate::Rubik;

impl Rubik {
    pub fn execute(&mut self, operation: &RubikTransform) -> &mut Self {
        operation.apply_on(self);
        self
    }
}

#[macro_export]
macro_rules! tf {
    ($val: expr) => {
        $crate::transform::RubikTransform::from($val)
    };
    ($($val: expr),*$(,)?) => {
        $crate::transform::RubikTransform::Combine(
            vec![$($crate::transform::RubikTransform::from($val)),*]
        )
    };
    ($val: expr; $times: expr) => {
        ($crate::transform::RubikTransform::from($val)).repeat($times)
    };
}

#[derive(Debug, Clone)]
pub enum RubikTransform {
    Layer(RubikLayerTransform),
    Repeat(Box<Self>, usize),
    Combine(Vec<Self>),
}

impl RubikTransform {
    fn apply_on(&self, rubik: &mut Rubik) {
        match self {
            RubikTransform::Layer(layer) => layer.apply_on(rubik),
            RubikTransform::Repeat(transform, times) => {
                for _ in 0..*times {
                    transform.apply_on(rubik);
                }
            }
            RubikTransform::Combine(transforms) => {
                for transform in transforms {
                    transform.apply_on(rubik);
                }
            }
        }
    }
}

impl RubikTransform {
    pub fn inverse(self) -> Self {
        match self {
            RubikTransform::Layer(layer) => RubikTransform::Layer(layer.inverse()),
            RubikTransform::Repeat(transform, times) => {
                RubikTransform::Repeat(Box::new(transform.inverse()), times)
            }
            RubikTransform::Combine(transforms) => {
                RubikTransform::Combine(transforms.into_iter().rev().map(|t| t.inverse()).collect())
            }
        }
    }

    pub fn repeat(self, n: usize) -> Self {
        Self::Repeat(Box::new(self), n)
    }
}

impl<T> From<Vec<T>> for RubikTransform
where
    T: Into<RubikTransform>,
{
    fn from(val: Vec<T>) -> Self {
        RubikTransform::Combine(val.into_iter().map(Into::into).collect())
    }
}

impl<T, const N: usize> From<[T; N]> for RubikTransform
where
    T: Into<RubikTransform>,
{
    fn from(val: [T; N]) -> Self {
        RubikTransform::Combine(val.into_iter().map(Into::into).collect())
    }
}

impl<T> From<(T,)> for RubikTransform
where
    T: Into<RubikTransform>,
{
    fn from(val: (T,)) -> Self {
        val.0.into()
    }
}

macro_rules! derive_tuple {
    ($T: ident,) => {};
    ($TN: ident, $($T: ident,)+) => {
        derive_tuple!($($T,)+);
        derive_tuple!(@gen $TN,$($T,)+);
    };
    (@gen $($T: ident,)*) => {
        #[allow(non_snake_case)]
        impl<$($T),*> From<($($T,)*)> for RubikTransform
        where $($T: Into<RubikTransform>),*
        {
            fn from(($($T,)*): ($($T,)*)) -> Self {
                RubikTransform::Combine(vec![$($T.into()),*])
            }
        }

    };
}

derive_tuple! {
    T15, T14, T13, T12, T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1, T0,
}

pub const F: RubikLayerTransform = RubikLayerTransform::F;
pub const B: RubikLayerTransform = RubikLayerTransform::B;
pub const L: RubikLayerTransform = RubikLayerTransform::L;
pub const R: RubikLayerTransform = RubikLayerTransform::R;
pub const U: RubikLayerTransform = RubikLayerTransform::U;
pub const D: RubikLayerTransform = RubikLayerTransform::D;
pub const M: RubikLayerTransform = RubikLayerTransform::M;
pub const E: RubikLayerTransform = RubikLayerTransform::E;
pub const S: RubikLayerTransform = RubikLayerTransform::S;

pub const FI: RubikLayerTransform = RubikLayerTransform::FI;
pub const BI: RubikLayerTransform = RubikLayerTransform::BI;
pub const LI: RubikLayerTransform = RubikLayerTransform::LI;
pub const RI: RubikLayerTransform = RubikLayerTransform::RI;
pub const UI: RubikLayerTransform = RubikLayerTransform::UI;
pub const DI: RubikLayerTransform = RubikLayerTransform::DI;
pub const MI: RubikLayerTransform = RubikLayerTransform::MI;
pub const EI: RubikLayerTransform = RubikLayerTransform::EI;
pub const SI: RubikLayerTransform = RubikLayerTransform::SI;

pub const F2: RubikLayerTransform = RubikLayerTransform::F2;
pub const B2: RubikLayerTransform = RubikLayerTransform::B2;
pub const L2: RubikLayerTransform = RubikLayerTransform::L2;
pub const R2: RubikLayerTransform = RubikLayerTransform::R2;
pub const U2: RubikLayerTransform = RubikLayerTransform::U2;
pub const D2: RubikLayerTransform = RubikLayerTransform::D2;
pub const M2: RubikLayerTransform = RubikLayerTransform::M2;
pub const E2: RubikLayerTransform = RubikLayerTransform::E2;
pub const S2: RubikLayerTransform = RubikLayerTransform::S2;
