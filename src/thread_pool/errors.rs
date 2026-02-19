use core::fmt;

#[derive(Debug)]
pub struct PoolCreationError {
    reason: String,
}

impl PoolCreationError {
    pub fn new(reason: &str) -> Self {
        PoolCreationError {
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolCreationError: {}", self.reason)
    }
}
