use crate::paras::conventions::{Has, Name, With};
use k8s_openapi::api::core::v1::Container;

use super::VolumeMounts;

impl Has<Name> for Container {
    fn get(&self) -> Option<Name> {
        Some(self.name.clone().into())
    }
}

impl With<Name> for Container {
    fn with(&self, x: Name) -> Self {
        Self {
            name: x.into(),
            ..self.clone()
        }
    }
}

impl Has<VolumeMounts> for Container {
    fn get(&self) -> Option<VolumeMounts> {
        self.volume_mounts.clone()
    }
}

impl With<VolumeMounts> for Container {
    fn with(&self, x: VolumeMounts) -> Self {
        Self {
            volume_mounts: Some(x),
            ..self.clone()
        }
    }
}
