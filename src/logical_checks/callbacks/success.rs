use boolean_check_derive::CheckOps;

use crate::Check;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
pub struct WithSuccessCallbackCheck<C: Check, S>
where
    S: Fn() -> (),
{
    callback: S,
    check: C,
}

impl<C: Check, S> WithSuccessCallbackCheck<C, S>
where
    S: Fn() -> (),
{
    pub fn new(check: C, callback: S) -> Self {
        Self { check, callback }
    }
}

impl<C: Check, S> Check for WithSuccessCallbackCheck<C, S>
where
    S: Fn() -> (),
{
    fn check(&self) -> bool {
        let result = self.check.check();
        if result {
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
    fn test_callback_is_called_on_success() {
        let was_called = Cell::new(false);
        let check = CustomCheck::new(|| true);
        let with_callback = WithSuccessCallbackCheck::new(check, || was_called.set(true));

        assert!(with_callback.check());
        assert!(was_called.get());
    }

    #[test]
    fn test_callback_is_not_called_on_failure() {
        let was_called = Cell::new(false);
        let check = CustomCheck::new(|| false);
        let with_callback = WithSuccessCallbackCheck::new(check, || was_called.set(true));

        assert!(!with_callback.check());
        assert!(!was_called.get());
    }
}
