pub mod logical_checks;
pub use boolean_check_derive::CheckOps;

use crate::logical_checks::operators::{and::AndCheck, not::InvertedCheck, or::OrCheck};

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

    fn invert<R: Check>(self) -> InvertedCheck<Self>
    where
        Self: Sized,
    {
        InvertedCheck::new(self)
    }
}
