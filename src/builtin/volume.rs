use k8s_openapi::api::core::v1::{Volume, VolumeMount};

pub type Volumes = Vec<Volume>;
pub type VolumeMounts = Vec<VolumeMount>;
