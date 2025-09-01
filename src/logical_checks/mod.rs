use boolean_check_derive::CheckOps;

use crate::Check;

pub mod all;
pub mod any;
pub mod callbacks;
pub mod operators;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CheckOps)]
pub struct CustomCheck<C>
where
    C: Fn() -> bool,
{
    condition: C,
}

impl<C> CustomCheck<C>
where
    C: Fn() -> bool,
{
    pub fn new(condition: C) -> Self {
        Self { condition }
    }
}

impl<C> Check for CustomCheck<C>
where
    C: Fn() -> bool,
{
    fn check(&self) -> bool {
        (self.condition)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_check() {
        let check = CustomCheck::new(|| true);
        assert!(check.check());
    }

    #[test]
    fn invert() {
        let check = CustomCheck::new(|| true);
        assert_eq!((!check).check(), false);
    }

    #[test]
    fn or() {
        let true_check = CustomCheck::new(|| true);
        let false_check = CustomCheck::new(|| false);
        let check = true_check | false_check;
        assert!(check.check());
    }
}
