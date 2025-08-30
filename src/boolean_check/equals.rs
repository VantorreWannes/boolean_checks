use crate::boolean_check::{
    Check, Condition,
    operators::{and::AndCheck, not::InvertedCheck, or::OrCheck},
};
use std::ops::{BitAnd, BitOr, Not};

#[derive(Debug, Clone, Copy)]
pub struct EqualsCheck<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> EqualsCheck<L, R> {
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L, R> Condition for EqualsCheck<L, R>
where
    L: PartialEq<R>,
{
    fn condition(&self) -> bool {
        self.lhs == self.rhs
    }
}

impl<L, R> Check for EqualsCheck<L, R>
where
    L: PartialEq<R> + Clone,
    R: Clone,
{
}

pub fn equals<L, R>(lhs: L, rhs: R) -> EqualsCheck<L, R> {
    EqualsCheck::new(lhs, rhs)
}

impl<L, R, Rhs> BitAnd<Rhs> for EqualsCheck<L, R>
where
    L: PartialEq<R> + Clone,
    R: Clone,
    Rhs: Check,
{
    type Output = AndCheck<Self, Rhs>;

    fn bitand(self, rhs: Rhs) -> Self::Output {
        AndCheck::new(self, rhs)
    }
}

impl<L, R, Rhs> BitOr<Rhs> for EqualsCheck<L, R>
where
    L: PartialEq<R> + Clone,
    R: Clone,
    Rhs: Check,
{
    type Output = OrCheck<Self, Rhs>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        OrCheck::new(self, rhs)
    }
}

impl<L, R> Not for EqualsCheck<L, R>
where
    L: PartialEq<R> + Clone,
    R: Clone,
{
    type Output = InvertedCheck<Self>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self)
    }
}
