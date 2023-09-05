use super::RubikTransform;

pub struct TransfromRepeat<'r> {
    pub transform: &'r dyn RubikTransform,
    pub times: usize,
}

impl dyn RubikTransform {
    pub fn repeat(&self, times: usize) -> TransfromRepeat<'_>  {
        TransfromRepeat {
            transform: self,
            times,
        }
    }
}

impl<'r> RubikTransform for TransfromRepeat<'r> {
    fn apply_on(&self, rubik: &mut crate::Rubik) {
        for _ in 0..self.times {
            self.transform.apply_on(rubik);
        }
    }
}