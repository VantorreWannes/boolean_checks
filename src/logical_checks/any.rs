use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, PartialEq, Eq, CheckOps)]
pub struct AnyCheck<C: Check> {
    checks: Vec<C>,
}

impl<C: Check> AnyCheck<C> {
    pub fn new(checks: Vec<C>) -> Self {
        Self { checks }
    }
}

impl<C: Check> Check for AnyCheck<C> {
    fn check(&self) -> bool {
        self.checks.iter().any(|check| check.check())
    }
}
