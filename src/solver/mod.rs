use std::{rc::Rc, sync::Arc, vec::IntoIter};

use crate::{prelude::RubikLayerTransform, Rubik};

pub trait RubikSolver {
    fn solve(&self, rubik: Rubik) -> RubikSolveState;
}

pub trait TransferableState {
    type Iter: Iterator<Item = Self>;
    fn neighbors(self) -> Self::Iter;
    fn random_transfer(self) -> Self
    where
        Self: Sized,
    {
        use rand::distributions::{Distribution, Uniform};
        let neighbors = self.neighbors().collect::<Vec<_>>();
        let idx = Uniform::new(0, neighbors.len()).sample(&mut rand::rngs::ThreadRng::default());
        neighbors.into_iter().nth(idx).expect("shouldn't outbound")
    }
}

#[derive(Clone)]
pub struct RubikSolveState {
    rubik: Rubik,
    op_set: Arc<[&'static RubikLayerTransform]>,
    from: Option<(Arc<Self>, &'static RubikLayerTransform,)>,
}

impl RubikSolveState {
    pub fn new(rubik: Rubik, op_set: Arc<[&'static RubikLayerTransform]>) -> Self {
        Self {
            rubik,
            op_set,
            from: None,
        }
    } 
    pub fn collect(self) -> (Rubik, Vec<&'static RubikLayerTransform>) {
        let rubik = self.rubik;
        let mut from = self.from;
        let mut ops = vec![];
        while let Some((source, op)) = from {
            ops.push(op);
            from = source.from.clone();
        }
        ops.reverse();
        (rubik, ops)
    }

    pub fn rubik() {

    }
}

impl TransferableState for RubikSolveState {
    type Iter = std::vec::IntoIter<RubikSolveState>;

    fn neighbors(self) -> Self::Iter {
        let arc_self = Arc::new(self);
        arc_self
            .clone()
            .op_set
            .iter()
            .map(move |op| {
                let mut rubik = arc_self.rubik.clone();
                op.apply_on(&mut rubik);
                Self {
                    rubik,
                    op_set: arc_self.op_set.clone(),
                    from: Some((arc_self.clone(), *op)),
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn random_transfer(self) -> Self
    where
        Self: Sized,
    {
        let arc_self = Arc::new(self);
        let mut rubik = arc_self.rubik.clone();
        let op = arc_self.op_set[rand::random::<usize>() % arc_self.op_set.len()];
        op.apply_on(&mut rubik);
        Self {
            rubik,
            op_set: arc_self.op_set.clone(),
            from: Some((arc_self, op)),
        }
    }
}
pub mod sa;
pub mod shuffle;
pub mod thistlethwaite;
pub mod ida_star;