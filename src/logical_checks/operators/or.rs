use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
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

impl<L, R> Check for OrCheck<L, R>
where
    L: Check,
    R: Check,
{
    fn check(&self) -> bool {
        self.lhs.check() || self.rhs.check()
    }
}

#[cfg(test)]
mod tests {
    use crate::logical_checks::CustomCheck;
    use super::*;

    #[test]
    fn test_check_is_true() {
        let true_check = CustomCheck::new(|| true);
        let false_check = CustomCheck::new(|| false);

        // Test true || false -> true
        let or_check_1 = OrCheck::new(true_check.clone(), false_check.clone());
        assert!(or_check_1.check());

        // Test false || true -> true
        let or_check_2 = OrCheck::new(false_check.clone(), true_check.clone());
        assert!(or_check_2.check());
    }

    #[test]
    fn test_check_is_false() {
        let false_check_1 = CustomCheck::new(|| false);
        let false_check_2 = CustomCheck::new(|| false);

        // Test false || false -> false
        let or_check = OrCheck::new(false_check_1, false_check_2);
        assert!(!or_check.check());
    }

    #[test]
    fn test_check_short_circuit() {
        let true_check = CustomCheck::new(|| true);
        let panic_check = CustomCheck::new(|| panic!("should not be called"));

        // Test true || (panic) -> true
        let or_check = OrCheck::new(true_check, panic_check);
        assert!(or_check.check());
    }
}

