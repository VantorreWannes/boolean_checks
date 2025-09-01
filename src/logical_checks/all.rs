use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, PartialEq, Eq, CheckOps)]
pub struct AllCheck<C: Check> {
    checks: Vec<C>,
}

impl<C: Check> AllCheck<C> {
    pub fn new(checks: Vec<C>) -> Self {
        Self { checks }
    }
}

impl<C: Check> Check for AllCheck<C> {
    fn check(&self) -> bool {
        self.checks.iter().all(|check| check.check())
    }
}
