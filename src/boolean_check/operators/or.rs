use std::ops::{BitAnd, BitOr, Not};

use crate::boolean_check::{
    Check, Condition,
    operators::{and::AndCheck, not::InvertedCheck},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrCheck<L: Check, R: Check> {
    lhs: L,
    rhs: R,
}

impl<L, R> OrCheck<L, R>
where
    L: Check,
    R: Check,
{
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L, R> Condition for OrCheck<L, R>
where
    L: Check,
    R: Check,
{
    fn condition(&self) -> bool {
        self.lhs.check() || self.rhs.check()
    }
}

impl<L, R> Check for OrCheck<L, R>
where
    L: Check,
    R: Check,
    Self: Condition,
{
}

impl<L, R, Rhs> BitAnd<Rhs> for OrCheck<L, R>
where
    L: Check,
    R: Check,
    Rhs: Check,
{
    type Output = AndCheck<OrCheck<L, R>, Rhs>;

    fn bitand(self, rhs: Rhs) -> Self::Output {
        AndCheck::new(self, rhs)
    }
}

impl<L, R, Rhs> BitOr<Rhs> for OrCheck<L, R>
where
    L: Check,
    R: Check,
    Rhs: Check,
{
    type Output = OrCheck<OrCheck<L, R>, Rhs>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        OrCheck::new(self, rhs)
    }
}

impl<L, R, Rhs> BitAnd<&Rhs> for &OrCheck<L, R>
where
    L: Check + Clone,
    R: Check + Clone,
    Rhs: Check + Clone,
{
    type Output = AndCheck<OrCheck<L, R>, Rhs>;

    fn bitand(self, rhs: &Rhs) -> Self::Output {
        AndCheck::new(self.clone(), rhs.clone())
    }
}

impl<L, R, Rhs> BitOr<&Rhs> for &OrCheck<L, R>
where
    L: Check + Clone,
    R: Check + Clone,
    Rhs: Check + Clone,
{
    type Output = OrCheck<OrCheck<L, R>, Rhs>;

    fn bitor(self, rhs: &Rhs) -> Self::Output {
        OrCheck::new(self.clone(), rhs.clone())
    }
}

impl<L: Check, R: Check> Not for OrCheck<L, R> {
    type Output = InvertedCheck<OrCheck<L, R>>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self)
    }
}

impl<L: Check + Clone, R: Check + Clone> Not for &OrCheck<L, R> {
    type Output = InvertedCheck<OrCheck<L, R>>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self.clone())
    }
}

impl<L: Check + Clone, R: Check + Clone> From<OrCheck<L, R>> for bool {
    fn from(value: OrCheck<L, R>) -> Self {
        value.check()
    }
}

impl<L: Check + Clone, R: Check + Clone> From<&OrCheck<L, R>> for bool {
    fn from(value: &OrCheck<L, R>) -> Self {
        value.check()
    }
}
