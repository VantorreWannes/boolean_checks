use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
pub struct InvertedCheck<C: Check> {
    check: C,
}

impl<C: Check> InvertedCheck<C> {
    pub fn new(check: C) -> Self {
        Self { check }
    }
}

impl<C: Check> Check for InvertedCheck<C> {
    fn check(&self) -> bool {
        !self.check.check()
    }
}

#[cfg(test)]
mod tests {
    use crate::logical_checks::CustomCheck;
    use crate::Check;
    use super::*;

    #[test]
    fn test_inverts_true_to_false() {
        let true_check = CustomCheck::new(|| true);
        let inverted = InvertedCheck::new(true_check);
        assert!(!inverted.check());
    }

    #[test]
    fn test_inverts_false_to_true() {
        let false_check = CustomCheck::new(|| false);
        let inverted = InvertedCheck::new(false_check);
        assert!(inverted.check());
    }
}

