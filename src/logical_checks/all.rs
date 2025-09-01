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

#[cfg(test)]
mod tests {
    use crate::logical_checks::CustomCheck;
    use crate::Check;
    use super::*;

    #[test]
    fn test_all_true_is_true() {
        let checks = vec![
            CustomCheck::new(|| true),
            CustomCheck::new(|| true),
        ];
        let all_check = AllCheck::new(checks);
        assert!(all_check.check());
    }

    #[test]
    fn test_any_false_is_false() {
        let checks = vec![
            CustomCheck::new(|| true),
            CustomCheck::new(|| false),
        ];
        let all_check = AllCheck::new(checks);
        assert!(!all_check.check());
    }

    #[test]
    fn test_empty_is_true() {
        let checks: Vec<CustomCheck> = vec![];
        let all_check = AllCheck::new(checks);
        assert!(all_check.check());
    }

    #[test]
    fn test_short_circuits() {
        let checks = vec![
            CustomCheck::new(|| false),
            CustomCheck::new(|| panic!("should not be called")),
        ];
        let all_check = AllCheck::new(checks);
        assert!(!all_check.check());
    }
}

