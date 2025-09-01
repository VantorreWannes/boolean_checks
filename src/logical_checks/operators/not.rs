use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
pub struct InvertedCheck<C: Check> {
    check: C,
}

impl<C: Check> InvertedCheck<C> {
    pub fn new(check: C) -> Self {
        Self { check }
    }
}

impl<C: Check> Check for InvertedCheck<C> {
    fn check(&self) -> bool {
        !self.check.check()
    }
}
