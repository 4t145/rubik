use std::sync::Arc;

use crate::prelude::RubikLayerTransform;

use super::{RubikSolveState, RubikSolver, TransferableState};

pub trait SaSolver {
    type State: TransferableState + Clone;
    type Energy: PartialOrd + Clone;
    type Temprature: PartialOrd;
    fn energy(state: &Self::State) -> Self::Energy;
    fn random_transform(e0: &Self::Energy, e: &Self::Energy, temp: &Self::Temprature) -> bool;
}

pub fn solve_sa<S: SaSolver>(
    mut state: S::State,
    target_energy: S::Energy,
    temp_curve: impl Iterator<Item = S::Temprature>,
) -> S::State {
    let energy_0 = S::energy(&state);
    let mut energy = energy_0.clone();
    for temp in temp_curve {
        if energy <= target_energy {
            break;
        } else {
            let new_state = state.clone().random_transfer();
            if S::random_transform(&energy_0, &energy, &temp) {
                state = new_state;
                energy = S::energy(&state);
            }
        }
    }
    state
}

pub struct SaUnitRubikSolver;

impl SaSolver for SaUnitRubikSolver {
    type State = RubikSolveState;

    type Energy = f64;

    type Temprature = f64;

    fn energy(state: &Self::State) -> Self::Energy {
        state.rubik.entropy()
    }

    fn random_transform(e0: &Self::Energy, e: &Self::Energy, temp: &Self::Temprature) -> bool {
        let delta = e - e0;
        let p = (delta / temp).exp();
        rand::random::<f64>() < p
    }
}

pub struct SaRubikSolver;
impl RubikSolver for SaRubikSolver {
    fn solve(&self, mut rubik: crate::Rubik) -> RubikSolveState {
        let op_set = [
            &RubikLayerTransform::R,
            &RubikLayerTransform::L,
            &RubikLayerTransform::F,
            &RubikLayerTransform::U,
            &RubikLayerTransform::D,
            &RubikLayerTransform::B,
            &RubikLayerTransform::RI,
            &RubikLayerTransform::LI,
            &RubikLayerTransform::FI,
            &RubikLayerTransform::UI,
            &RubikLayerTransform::DI,
            &RubikLayerTransform::BI,
            &RubikLayerTransform::R2,
            &RubikLayerTransform::L2,
            &RubikLayerTransform::F2,
            &RubikLayerTransform::U2,
            &RubikLayerTransform::D2,
            &RubikLayerTransform::B2,
            &RubikLayerTransform::E,
            &RubikLayerTransform::S,
            &RubikLayerTransform::M,
            &RubikLayerTransform::EI,
            &RubikLayerTransform::SI,
            &RubikLayerTransform::MI,
            &RubikLayerTransform::E2,
            &RubikLayerTransform::S2,
            &RubikLayerTransform::M2,
        ];
        let init_op = op_set[rand::random::<usize>() % op_set.len()];
        init_op.apply_on(&mut rubik);
        let mut state = RubikSolveState {
            rubik,
            op_set: Arc::new(op_set),
            from: None,
        };
        let curve = (0..100).rev().map(|x| (x as f64));
        for _round in 0..50 {
            let mut round_state = state;
            for _idx in 0..50 {
                let new_state =
                    solve_sa::<SaUnitRubikSolver>(round_state.clone(), 0.0, curve.clone());
                if new_state.rubik.entropy() < round_state.rubik.entropy() {
                    round_state = new_state;
                }
            }
            state = round_state;
        }
        state
    }
}
