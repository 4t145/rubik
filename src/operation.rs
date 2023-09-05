use crate::transform::*;

macro_rules! export_static_transform {
    {$Type: ty:$($IDENT: ident,)*} => {
        $(
            pub const $IDENT: &dyn RubikTransform = &<$Type>::$IDENT;
        )*
    };
}

export_static_transform! {
    RubikLayerTransform :
        R, RI, R2,
        L, LI, L2,
        U, UI, U2,
        D, DI, D2,
        F, FI, F2,
        B, BI, B2,
        M, MI, M2,
        E, EI, E2,
        S, SI, S2,
}

// composed layer transform

pub const X: &dyn RubikTransform = &[R, MI, LI];
pub const XI: &dyn RubikTransform = &[RI, M, L];
pub const X2: &dyn RubikTransform = &[R2, M2, L2];

pub const Y: &dyn RubikTransform = &[U, EI, DI];
pub const YI: &dyn RubikTransform = &[UI, E, D];
pub const Y2: &dyn RubikTransform = &[U2, E2, D2];

pub const Z: &dyn RubikTransform = &[F, SI, BI];
pub const ZI: &dyn RubikTransform = &[FI, S, B];
pub const Z2: &dyn RubikTransform = &[F2, S2, B2];

pub const RR: &dyn RubikTransform = &[R, MI];
pub const RRI: &dyn RubikTransform = &[RI, M];
pub const RR2: &dyn RubikTransform = &[R2, M2];

pub const LL: &dyn RubikTransform = &[L, M];
pub const LLI: &dyn RubikTransform = &[LI, MI];
pub const LL2: &dyn RubikTransform = &[L2, M2];

pub const UU: &dyn RubikTransform = &[U, EI];
pub const UUI: &dyn RubikTransform = &[UI, E];
pub const UU2: &dyn RubikTransform = &[U2, E2];

pub const DD: &dyn RubikTransform = &[D, E];
pub const DDI: &dyn RubikTransform = &[DI, EI];
pub const DD2: &dyn RubikTransform = &[D2, E2];

pub const FF: &dyn RubikTransform = &[F, SI];
pub const FFI: &dyn RubikTransform = &[FI, S];
pub const FF2: &dyn RubikTransform = &[F2, S2];

pub const BB: &dyn RubikTransform = &[B, S];
pub const BBI: &dyn RubikTransform = &[BI, SI];
pub const BB2: &dyn RubikTransform = &[B2, S2];
