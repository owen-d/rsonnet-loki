use std::fmt;

use super::conventions::Name;

/// Target is used for subcomponent selection and explicitly maps to the `-target=<target>` CLI flag.
#[derive(PartialEq, Clone, Default, Debug)]
pub struct Target(String);
impl Target {
    pub fn new(x: String) -> Self {
        Target(x)
    }

    /// Detect whether subject is a target
    pub fn is(s: &str) -> Option<Target> {
        // match `--target` and `-target`
        let trimmed = s.trim_start_matches('-');
        if !trimmed.starts_with("target=") {
            return None;
        }
        Some(Self::new(trimmed.trim_start_matches("target=").to_string()))
    }
}

impl From<Name> for Target {
    fn from(n: Name) -> Self {
        Self::new(n.into())
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-target={}", self.0)
    }
}
