use std::{borrow::Cow, rc::Rc, sync::Arc};

use super::RubikTransform;
use crate::Rubik;
impl<T> RubikTransform for &T
where
    T: RubikTransform + ?Sized,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        (*self).apply_on(rubik)
    }
}

impl<T> RubikTransform for Box<T>
where
    T: RubikTransform + ?Sized,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        self.as_ref().apply_on(rubik)
    }
}

impl<T> RubikTransform for Rc<T>
where
    T: RubikTransform + ?Sized,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        self.as_ref().apply_on(rubik)
    }
}

impl<T> RubikTransform for Arc<T>
where
    T: RubikTransform + ?Sized,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        self.as_ref().apply_on(rubik)
    }
}

impl<T> RubikTransform for Cow<'_, T>
where
    T: RubikTransform + Clone + ?Sized,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        self.as_ref().apply_on(rubik)
    }
}

impl<T> RubikTransform for dyn AsRef<T>
where
    T: RubikTransform + ?Sized,
{
    fn apply_on(&self, rubik: &mut Rubik) {
        self.as_ref().apply_on(rubik)
    }
}