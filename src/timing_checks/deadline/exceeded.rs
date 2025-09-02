use boolean_check_derive::CheckOps;
use chrono::{DateTime, Local, TimeDelta, TimeZone};

use crate::Check;

#[derive(Debug, Clone, PartialEq, Eq, CheckOps)]
pub struct DeadlineExceededCheck<T: TimeZone = Local> {
    deadline: DateTime<T>,
}

impl<T: TimeZone> DeadlineExceededCheck<T> {
    pub fn new(deadline: DateTime<T>) -> Self {
        Self { deadline }
    }

    fn now(&self) -> DateTime<T> {
        Local::now().with_timezone(&self.deadline.timezone())
    }

    pub fn remaining(&self) -> TimeDelta {
        self.deadline.clone() - self.now()
    }
}

impl<T: TimeZone> Check for DeadlineExceededCheck<T> {
    fn check(&self) -> bool {
        self.now() > self.deadline
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_deadline_not_exceeded() {
        let deadline = Utc::now() + Duration::milliseconds(100);
        let check = DeadlineExceededCheck::new(deadline);
        assert!(!check.check());
    }

    #[test]
    fn test_deadline_is_exceeded() {
        let deadline = Utc::now() + Duration::nanoseconds(0);
        let check = DeadlineExceededCheck::new(deadline);
        assert!(check.check());
    }

    #[test]
    fn test_remaining_time() {
        let timeout = Duration::milliseconds(100);
        let deadline = Utc::now() + timeout;
        let check = DeadlineExceededCheck::new(deadline);
        assert!(!check.check());
        assert!(check.remaining() <= timeout);
    }
}
