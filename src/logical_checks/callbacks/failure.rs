use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
pub struct WithFailureCallbackCheck<C: Check, F>
where
    F: Fn() -> (),
{
    callback: F,
    check: C,
}

impl<C: Check, F> WithFailureCallbackCheck<C, F>
where
    F: Fn() -> (),
{
    pub fn new(check: C, callback: F) -> Self {
        Self { check, callback }
    }
}

impl<C: Check, F> Check for WithFailureCallbackCheck<C, F>
where
    F: Fn() -> (),
{
    fn check(&self) -> bool {
        let result = self.check.check();
        if !result {
            (self.callback)()
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use crate::logical_checks::CustomCheck;
    use std::cell::Cell;

    #[test]
    fn test_callback_is_called_on_failure() {
        let was_called = Cell::new(false);
        let check = CustomCheck::new(|| false);
        let with_callback = WithFailureCallbackCheck::new(check, || was_called.set(true));

        assert!(!with_callback.check());
        assert!(was_called.get());
    }

    #[test]
    fn test_callback_is_not_called_on_success() {
        let was_called = Cell::new(false);
        let check = CustomCheck::new(|| true);
        let with_callback = WithFailureCallbackCheck::new(check, || was_called.set(true));

        assert!(with_callback.check());
        assert!(!was_called.get());
    }
}
