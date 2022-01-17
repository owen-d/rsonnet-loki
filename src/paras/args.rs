use derive_more::From;
use k8s_openapi::api::core::v1::Container;

use crate::builtin::Has;

/// Target is used for subcomponent selection and explicitly maps to the `-target=<target>` CLI flag.
#[derive(PartialEq, From, Clone, Default, Debug)]
pub struct Target(pub String);
impl Target {
    pub fn new(x: String) -> Self {
        Target(x)
    }
}

impl<T: Has<Container>> Has<Target> for T {
    fn get(&self) -> Option<Target> {
        self.get()
            .and_then(|c: Container| c.command)
            .and_then(|xs: Vec<String>| {
                xs.iter()
                    .filter_map(|s| {
                        // match `--target` and `-target`
                        let trimmed = s.trim_start_matches('-');
                        if !trimmed.starts_with("target=") {
                            return None;
                        }
                        Some(trimmed.trim_start_matches("target=").to_string())
                    })
                    .take(1)
                    .collect::<Vec<String>>()
                    .first()
                    .map(|x: &String| -> Target { Target::new(x.to_owned()) })
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
        assert_eq!(Some("bonk".to_string().into()), tgt)
    }
}
