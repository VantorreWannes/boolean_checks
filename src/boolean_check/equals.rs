use crate::boolean_check::{
    Check,
    operators::{and::AndCheck, not::InvertedCheck, or::OrCheck},
};
use std::ops::{BitAnd, BitOr, Not};

#[derive(Debug, Clone, Copy)]
pub struct EqualsCheck<L: PartialEq<R> + Check, R: Check> {
    lhs: L,
    rhs: R,
}

impl<L: PartialEq<R> + Check, R: Check> EqualsCheck<L, R> {
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L: PartialEq<R> + Check, R: Check> Check for EqualsCheck<L, R> {
    fn check(&self) -> bool {
        self.lhs.check() == self.rhs.check()
    }
}

pub fn equals<L: PartialEq<R> + Check, R: Check>(lhs: L, rhs: R) -> EqualsCheck<L, R> {
    EqualsCheck::new(lhs, rhs)
}

impl<L, R, Rhs> BitAnd<Rhs> for EqualsCheck<L, R>
where
    L: PartialEq<R> + Check + Clone,
    R: Clone + Check,
    Rhs: Check,
{
    type Output = AndCheck<Self, Rhs>;

    fn bitand(self, rhs: Rhs) -> Self::Output {
        AndCheck::new(self, rhs)
    }
}

impl<L, R, Rhs> BitOr<Rhs> for EqualsCheck<L, R>
where
    L: PartialEq<R> + Check + Clone,
    R: Clone + Check,
    Rhs: Check,
{
    type Output = OrCheck<Self, Rhs>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        OrCheck::new(self, rhs)
    }
}

impl<L, R> Not for EqualsCheck<L, R>
where
    L: PartialEq<R> + Clone + Check,
    R: Clone + Check,
{
    type Output = InvertedCheck<Self>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self)
    }
}
