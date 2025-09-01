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
