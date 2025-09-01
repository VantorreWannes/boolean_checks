use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
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

impl<L, R> Check for AndCheck<L, R>
where
    L: Check,
    R: Check,
{
    fn check(&self) -> bool {
        self.lhs.check() && self.rhs.check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use crate::logical_checks::CustomCheck;

    #[test]
    fn test_check_is_true_when_both_are_true() {
        let true_check = CustomCheck::new(|| true);
        let another_true_check = CustomCheck::new(|| true);
        let and_check = AndCheck::new(true_check, another_true_check);
        assert!(and_check.check());
    }

    #[test]
    fn test_check_is_false_when_one_is_false() {
        let true_check = CustomCheck::new(|| true);
        let false_check = CustomCheck::new(|| false);

        // true && false -> false
        let and_check_1 = AndCheck::new(true_check.clone(), false_check.clone());
        assert!(!and_check_1.check());

        // false && true -> false
        let and_check_2 = AndCheck::new(false_check.clone(), true_check.clone());
        assert!(!and_check_2.check());
    }

    #[test]
    fn test_check_short_circuit() {
        let false_check = CustomCheck::new(|| false);
        let panic_check = CustomCheck::new(|| panic!("should not be called"));
        let and_check = AndCheck::new(false_check, panic_check);
        // should not panic
        assert!(!and_check.check());
    }
}
