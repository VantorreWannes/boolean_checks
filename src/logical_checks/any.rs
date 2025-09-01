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

#[cfg(test)]
mod tests {
    use crate::logical_checks::CustomCheck;
    use crate::Check;
    use super::*;

    #[test]
    fn test_any_true_is_true() {
        let checks = vec![
            CustomCheck::new(|| false),
            CustomCheck::new(|| true),
        ];
        let any_check = AnyCheck::new(checks);
        assert!(any_check.check());
    }

    #[test]
    fn test_all_false_is_false() {
        let checks = vec![
            CustomCheck::new(|| false),
            CustomCheck::new(|| false),
        ];
        let any_check = AnyCheck::new(checks);
        assert!(!any_check.check());
    }

    #[test]
    fn test_empty_is_false() {
        let checks: Vec<CustomCheck> = vec![];
        let any_check = AnyCheck::new(checks);
        assert!(!any_check.check());
    }

    #[test]
    fn test_short_circuits() {
        let checks = vec![
            CustomCheck::new(|| true),
            CustomCheck::new(|| panic!("should not be called")),
        ];
        let any_check = AnyCheck::new(checks);
        assert!(any_check.check());
    }
}
