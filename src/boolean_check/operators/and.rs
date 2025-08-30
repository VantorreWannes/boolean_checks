use std::ops::{BitAnd, Not};

use crate::boolean_check::{Check, Condition, operators::not::InvertedCheck};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl<L, R> Condition for AndCheck<L, R>
where
    L: Check,
    R: Check,
{
    fn condition(&self) -> bool {
        self.lhs.check() && self.rhs.check()
    }
}

impl<L, R> Check for AndCheck<L, R>
where
    L: Check,
    R: Check,
    Self: Condition,
{
}

impl<L, R, Rhs> BitAnd<Rhs> for AndCheck<L, R>
where
    L: Check,
    R: Check,
    Rhs: Check,
{
    type Output = AndCheck<AndCheck<L, R>, Rhs>;

    fn bitand(self, rhs: Rhs) -> Self::Output {
        AndCheck::new(self, rhs)
    }
}

impl<L, R, Rhs> BitAnd<&Rhs> for &AndCheck<L, R>
where
    L: Check + Clone,
    R: Check + Clone,
    Rhs: Check + Clone,
{
    type Output = AndCheck<AndCheck<L, R>, Rhs>;

    fn bitand(self, rhs: &Rhs) -> Self::Output {
        AndCheck::new(self.clone(), rhs.clone())
    }
}

impl<L: Check, R: Check> Not for AndCheck<L, R> {
    type Output = InvertedCheck<AndCheck<L, R>>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self)
    }
}

impl<L: Check + Clone, R: Check + Clone> Not for &AndCheck<L, R> {
    type Output = InvertedCheck<AndCheck<L, R>>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self.clone())
    }
}

impl<L: Check + Clone, R: Check + Clone> From<AndCheck<L, R>> for bool {
    fn from(value: AndCheck<L, R>) -> Self {
        value.check()
    }
}

impl<L: Check + Clone, R: Check + Clone> From<&AndCheck<L, R>> for bool {
    fn from(value: &AndCheck<L, R>) -> Self {
        value.check()
    }
}
