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
