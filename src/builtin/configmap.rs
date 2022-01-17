use super::conventions::{Has, With};
use super::volume::Volumes;
use super::Name;
use derive_more::{From, Into};
use k8s_openapi::api::core::v1::{ConfigMap, ConfigMapVolumeSource, Volume};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::hash_map::DefaultHasher;

use std::hash::{Hash, Hasher};

pub const CONFIG_HASH_ANNOTATION: &str = "config_hash";

// Volume -> ConfigMapVolumeSource
impl<T> Has<Vec<ConfigMapVolumeSource>> for T
where
    T: Has<Volumes>,
{
    fn get(&self) -> Option<Vec<ConfigMapVolumeSource>> {
        self.get().map(|vs| {
            vs.into_iter()
                .filter_map(|v: Volume| v.config_map)
                .collect()
        })
    }
}

pub fn with_config_hash<T>(cfgs: Vec<HashableConfigMap>, x: T) -> T
where
    T: Has<ObjectMeta> + With<ObjectMeta>,
{
    let h = &mut DefaultHasher::new();
    cfgs.hash(h);
    let hash = h.finish();

    let ls = x
        .get()
        .and_then(|x: ObjectMeta| x.annotations)
        .or_else(|| Some(Default::default()))
        .map(|ls| {
            let mut res = ls;
            res.extend([(CONFIG_HASH_ANNOTATION.to_string(), format!("{:x}", hash))]);
            res
        });

    x.with(ObjectMeta {
        annotations: ls,
        ..x.get().unwrap_or_default()
    })
}

impl Has<ObjectMeta> for ConfigMap {
    fn get(&self) -> Option<ObjectMeta> {
        Some(self.metadata.clone())
    }
}

#[derive(From, Into, Clone)]
pub struct HashableConfigMap(ConfigMap);

impl HashableConfigMap {
    pub fn new(x: ConfigMap) -> Self {
        Self(x)
    }
}

impl From<HashableConfigMap> for Volume {
    fn from(x: HashableConfigMap) -> Self {
        let n: Name = x.0.get().unwrap_or_default();
        Volume {
            name: n.into(),
            ..Default::default()
        }
    }
}

impl From<HashableConfigMap> for ConfigMapVolumeSource {
    fn from(x: HashableConfigMap) -> Self {
        ConfigMapVolumeSource {
            name: x.0.get().map(|n: Name| n.into()),
            ..Default::default()
        }
    }
}

impl Hash for HashableConfigMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(data) = &self.0.binary_data {
            for (k, v) in data {
                k.hash(state);
                v.0.hash(state);
            }
        }
        self.0.data.hash(state);
    }
}
