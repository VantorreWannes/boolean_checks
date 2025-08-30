use std::ops::{BitAnd, BitOr, Not};

use crate::boolean_check::{
    Check, Condition,
    operators::{and::AndCheck, or::OrCheck},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvertedCheck<C: Check> {
    check: C,
}

impl<C: Check> InvertedCheck<C> {
    pub fn new(check: C) -> Self {
        Self { check }
    }
}

impl<C: Check> Condition for InvertedCheck<C> {
    fn condition(&self) -> bool {
        !self.check.check()
    }
}

impl<C: Check> Check for InvertedCheck<C> {}

impl<C, Rhs> BitAnd<Rhs> for InvertedCheck<C>
where
    C: Check,
    Rhs: Check,
{
    type Output = AndCheck<InvertedCheck<C>, Rhs>;

    fn bitand(self, rhs: Rhs) -> Self::Output {
        AndCheck::new(self, rhs)
    }
}

impl<C, Rhs> BitOr<Rhs> for InvertedCheck<C>
where
    C: Check,
    Rhs: Check,
{
    type Output = OrCheck<InvertedCheck<C>, Rhs>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        OrCheck::new(self, rhs)
    }
}

impl<C, Rhs> BitAnd<&Rhs> for &InvertedCheck<C>
where
    C: Check + Clone,
    Rhs: Check + Clone,
{
    type Output = AndCheck<InvertedCheck<C>, Rhs>;

    fn bitand(self, rhs: &Rhs) -> Self::Output {
        AndCheck::new(self.clone(), rhs.clone())
    }
}

impl<C, Rhs> BitOr<&Rhs> for &InvertedCheck<C>
where
    C: Check + Clone,
    Rhs: Check + Clone,
{
    type Output = OrCheck<InvertedCheck<C>, Rhs>;

    fn bitor(self, rhs: &Rhs) -> Self::Output {
        OrCheck::new(self.clone(), rhs.clone())
    }
}

impl<C: Check> Not for InvertedCheck<C> {
    type Output = C;
    fn not(self) -> Self::Output {
        self.check
    }
}

impl<'a, C: Check + Clone> Not for &'a InvertedCheck<C> {
    type Output = C;
    fn not(self) -> Self::Output {
        self.check.clone()
    }
}

impl<C: Check> From<InvertedCheck<C>> for bool {
    fn from(value: InvertedCheck<C>) -> Self {
        value.check()
    }
}

impl<C: Check> From<&InvertedCheck<C>> for bool {
    fn from(value: &InvertedCheck<C>) -> Self {
        value.check()
    }
}
