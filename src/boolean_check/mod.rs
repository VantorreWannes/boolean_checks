// ExceedsDeadline
// ExceedsTimeout

use std::ops::{BitAnd, BitOr, Not};

use crate::boolean_check::{
    equals::EqualsCheck,
    operators::{and::AndCheck, not::InvertedCheck, or::OrCheck},
};

pub mod equals;
pub mod operators;

pub trait Check {
    fn check(&self) -> bool;

    fn and<R: Check>(self, rhs: R) -> AndCheck<Self, R>
    where
        Self: Sized,
    {
        AndCheck::new(self, rhs)
    }

    fn or<R: Check>(self, rhs: R) -> OrCheck<Self, R>
    where
        Self: Sized,
    {
        OrCheck::new(self, rhs)
    }

    fn equals<R: Check>(self, rhs: R) -> EqualsCheck<Self, R>
    where
        Self: PartialEq<R> + Sized,
    {
        EqualsCheck::new(self, rhs)
    }

    fn invert<R: Check>(self) -> InvertedCheck<Self>
    where
        Self: Sized,
    {
        InvertedCheck::new(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomCheck<C: Fn() -> bool> {
    condition: C,
}

impl<C> CustomCheck<C>
where
    C: Fn() -> bool,
{
    pub fn new(condition: C) -> Self {
        Self { condition }
    }
}

impl<C> Check for CustomCheck<C>
where
    C: Fn() -> bool,
{
    fn check(&self) -> bool {
        (self.condition)()
    }
}

impl<C, Rhs> BitAnd<Rhs> for CustomCheck<C>
where
    C: Fn() -> bool,
    Rhs: Check,
{
    type Output = AndCheck<Self, Rhs>;

    fn bitand(self, rhs: Rhs) -> Self::Output {
        AndCheck::new(self, rhs)
    }
}

impl<C, Rhs> BitOr<Rhs> for CustomCheck<C>
where
    C: Fn() -> bool,
    Rhs: Check,
{
    type Output = OrCheck<Self, Rhs>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        OrCheck::new(self, rhs)
    }
}

impl<C, Rhs> BitAnd<&Rhs> for &CustomCheck<C>
where
    C: Fn() -> bool + Clone,
    Rhs: Check + Clone,
    Self: Clone,
{
    type Output = AndCheck<CustomCheck<C>, Rhs>;

    fn bitand(self, rhs: &Rhs) -> Self::Output {
        AndCheck::new(self.clone(), rhs.clone())
    }
}

impl<C, Rhs> BitOr<&Rhs> for &CustomCheck<C>
where
    C: Fn() -> bool + Clone,
    Rhs: Check + Clone,
    Self: Clone,
{
    type Output = OrCheck<CustomCheck<C>, Rhs>;

    fn bitor(self, rhs: &Rhs) -> Self::Output {
        OrCheck::new(self.clone(), rhs.clone())
    }
}

impl<C> Not for CustomCheck<C>
where
    C: Fn() -> bool,
{
    type Output = InvertedCheck<CustomCheck<C>>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self)
    }
}

impl<C: Fn() -> bool + Clone> Not for &CustomCheck<C> {
    type Output = InvertedCheck<CustomCheck<C>>;

    fn not(self) -> Self::Output {
        InvertedCheck::new(self.clone())
    }
}

impl<C: Fn() -> bool> From<CustomCheck<C>> for bool {
    fn from(value: CustomCheck<C>) -> Self {
        value.check()
    }
}

impl<C: Fn() -> bool> From<&CustomCheck<C>> for bool {
    fn from(value: &CustomCheck<C>) -> Self {
        value.check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true() {
        let value = true;
        let check = CustomCheck::new(move || value);
        assert!(check.check())
    }

    #[test]
    fn test_custom_check() {
        let true_check = CustomCheck::new(|| true);
        let false_check = CustomCheck::new(|| false);
        let check = true_check | false_check;
        assert!(check.check());
    }
}
