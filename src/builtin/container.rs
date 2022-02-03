use k8s_openapi::api::core::v1::Container;

use crate::paras::{
    args::Target,
    conventions::{Name, With},
};

impl With<Target> for Container {
    fn with(self, x: Target) -> Self {
        let mut cmd = self.command.clone().unwrap_or_default();
        cmd.retain(|x| Target::is(x).is_none());
        cmd.push(x.to_string());
        Self {
            command: Some(cmd),
            ..self
        }
    }
}

impl With<Name> for Container {
    fn with(self, x: Name) -> Self {
        Self {
            name: x.into(),
            ..self
        }
    }
}
