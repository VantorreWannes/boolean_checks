use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
pub struct AndCheck<L: Check, R: Check> {
    lhs: L,
    rhs: R,
}

impl<L, R> AndCheck<L, R>
where
    L: Check,
    R: Check,
{
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L, R> Check for AndCheck<L, R>
where
    L: Check,
    R: Check,
{
    fn check(&self) -> bool {
        self.lhs.check() && self.rhs.check()
    }
}
