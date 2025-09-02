use boolean_check_derive::CheckOps;
use chrono::{TimeDelta, Utc};

use crate::{Check, timing_checks::deadline::exceeded::DeadlineExceededCheck};

#[derive(Debug, Clone, PartialEq, Eq, CheckOps)]
pub struct TimeoutExceededCheck {
    deadline_exceeded_check: DeadlineExceededCheck<Utc>,
    timeout: TimeDelta,
}

impl TimeoutExceededCheck {
    pub fn new(timeout: TimeDelta) -> Self {
        let deadline = Utc::now() + timeout;
        let deadline_exceeded_check = DeadlineExceededCheck::<Utc>::new(deadline);
        Self {
            deadline_exceeded_check,
            timeout,
        }
    }

    pub fn reset(&mut self) {
        let deadline = Utc::now() + self.timeout;
        self.deadline_exceeded_check = DeadlineExceededCheck::<Utc>::new(deadline);
    }

    pub fn remaining(&self) -> TimeDelta {
        self.deadline_exceeded_check.remaining()
    }
}

impl Check for TimeoutExceededCheck {
    fn check(&self) -> bool {
        self.deadline_exceeded_check.check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::thread;

    #[test]
    fn test_timeout_not_exceeded() {
        let check = TimeoutExceededCheck::new(Duration::milliseconds(100));
        assert!(!check.check());
    }

    #[test]
    fn test_timeout_exceeded() {
        let timeout = Duration::milliseconds(100);
        let check = TimeoutExceededCheck::new(timeout.clone());
        thread::sleep(timeout.to_std().unwrap());
        assert!(check.check());
    }

    #[test]
    fn test_reset_works() {
        let timeout = Duration::milliseconds(100);
        let mut check = TimeoutExceededCheck::new(timeout.clone());
        thread::sleep(timeout.to_std().unwrap());
        assert!(check.check());
        check.reset();
        assert!(!check.check());
    }

    #[test]
    fn test_remaining_time_on_timeout() {
        let timeout = Duration::milliseconds(100);
        let check = TimeoutExceededCheck::new(timeout);
        assert!(!check.check());
        assert!(check.remaining() <= timeout);
    }
}
