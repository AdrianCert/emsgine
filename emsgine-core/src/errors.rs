use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct OverflowMemoryError;

impl Error for OverflowMemoryError {}

impl fmt::Display for OverflowMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OverflowMemoryError")
    }
}
