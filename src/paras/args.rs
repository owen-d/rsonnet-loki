use std::fmt;

use k8s_openapi::api::core::v1::Container;

use crate::builtin::{Has, With};

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

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-target={}", self.0)
    }
}

impl<T: Has<Container>> Has<Target> for T {
    fn get(&self) -> Option<Target> {
        self.get()
            .and_then(|c: Container| c.command)
            .and_then(|xs: Vec<String>| {
                xs.iter()
                    .filter_map(|s| Target::is(s.as_ref()))
                    .take(1)
                    .collect::<Vec<Target>>()
                    .first()
                    .map(|x| x.to_owned())
            })
    }
}

impl<T> With<Target> for T
where
    T: With<Container>,
{
    fn with(&self, x: Target) -> Self {
        let container: Container = self.get().unwrap_or_default();
        let mut cmd = container.command.clone().unwrap_or_default();
        cmd.retain(|x| Target::is(x).is_none());
        cmd.push(x.to_string());

        self.with(Container {
            command: Some(cmd),
            ..container
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_target() {
        let c = Container {
            command: vec![
                "--foo=bar".to_string(),
                "-target=bonk".to_string(),
                "-baz".to_string(),
                "buzz".to_string(),
            ]
            .into(),
            ..Default::default()
        };

        let tgt: Option<Target> = c.get();
        assert_eq!(Some(Target::new("bonk".to_string())), tgt)
    }

    #[test]
    fn test_with_target() {
        let c = Container {
            command: Some(vec![
                "--target=ingester".to_string(),
                "-other=bar".to_string(),
            ]),
            ..Default::default()
        };

        let next = c.with(Target::new(String::from("distributor")));

        assert_eq!(
            Some(vec![
                "-other=bar".to_string(),
                "-target=distributor".to_string(),
            ]),
            next.command
        )
    }
}
