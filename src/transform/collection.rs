use std::collections::{VecDeque, LinkedList, BTreeSet, BinaryHeap};

use crate::Rubik;

use super::RubikTransform;

impl<T> RubikTransform for [T]
where
    T: RubikTransform,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        for t in self {
            t.apply_on(rubik)
        }
    }
}

impl<T, const N: usize> RubikTransform for [T; N]
where
    T: RubikTransform,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        for t in self {
            t.apply_on(rubik)
        }
    }
}

impl<T> RubikTransform for Vec<T>
where
    T: RubikTransform,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        for t in self {
            t.apply_on(rubik)
        }
    }
}

impl<T> RubikTransform for VecDeque<T>
where
    T: RubikTransform,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        for t in self {
            t.apply_on(rubik)
        }
    }
}

impl<T> RubikTransform for LinkedList<T>
where
    T: RubikTransform,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        for t in self {
            t.apply_on(rubik)
        }
    }
}

impl<T> RubikTransform for BTreeSet<T>
where
    T: RubikTransform,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        for t in self {
            t.apply_on(rubik)
        }
    }
}

impl<T> RubikTransform for BinaryHeap<T>
where
    T: RubikTransform,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        for t in self {
            t.apply_on(rubik)
        }
    }
}