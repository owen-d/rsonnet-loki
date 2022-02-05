use k8s_openapi::api::core::v1::{Volume, VolumeMount};

use crate::paras::conventions::{Has, Name};

use super::configmap::HashableConfigMap;

pub type Volumes = Vec<Volume>;
pub type VolumeMounts = Vec<VolumeMount>;

impl From<HashableConfigMap> for Volume {
    fn from(x: HashableConfigMap) -> Self {
        let n: Option<Name> = x.0.has();
        Volume {
            name: n.unwrap_or_default().into(),
            config_map: Some(x.into()),
            ..Default::default()
        }
    }
}

impl Has<Name> for Volume {
    fn has(&self) -> Name {
        Name::new(self.name.clone())
    }
}
