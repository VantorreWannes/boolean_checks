use crate::Check;
use boolean_check_derive::CheckOps;
use std::rc::Rc;

pub mod all;
pub mod any;
pub mod callbacks;
pub mod operators;

#[derive(Clone, CheckOps)]
pub struct CustomCheck {
    condition: Rc<dyn Fn() -> bool>,
}

impl CustomCheck {
    pub fn new<C>(condition: C) -> Self
    where
        C: Fn() -> bool + 'static,
    {
        Self {
            condition: Rc::new(condition),
        }
    }
}

impl Check for CustomCheck {
    fn check(&self) -> bool {
        (self.condition)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let original = CustomCheck::new(|| true);
        let cloned = original.clone();
        assert!(original.check());
        assert!(cloned.check());
    }

    #[test]
    fn custom_check() {
        let check = CustomCheck::new(|| true);
        assert!(check.check());
    }

    #[test]
    fn custom_check_with_capture() {
        let x = 5;
        let check = CustomCheck::new(move || x > 0);
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
