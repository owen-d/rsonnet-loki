use super::conventions::{Has, With};
use super::volume::Volumes;
use k8s_openapi::api::core::v1::{ConfigMap, ConfigMapVolumeSource, Volume};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub const CONFIG_HASH_ANNOTATION: &str = "config_hash";

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

pub fn with_config_hash<A, B>(x: A, cfgs: B) -> A
where
    A: Has<Vec<ConfigMapVolumeSource>> + With<ObjectMeta>,
    B: Has<Vec<ConfigMap>>,
{
    let sources: Vec<ConfigMapVolumeSource> = x.get().unwrap_or_default();
    let mut tmp: HashMap<String, Option<ConfigMap>> = HashMap::from_iter(
        sources
            .into_iter()
            .map(|source: ConfigMapVolumeSource| (source.name.unwrap_or_default(), None))
            .collect::<Vec<_>>(),
    );
    // intersect referenced and provided configmaps
    for cfg in cfgs.get().unwrap_or_default() {
        let key = cfg.metadata.name.clone().unwrap_or_default();
        if tmp.contains_key(&key) {
            tmp.insert(key, Some(cfg));
        }
    }

    let intersection: Vec<HashableConfigMap> = tmp
        .into_iter()
        .filter_map(|(_, v)| v.map(HashableConfigMap))
        .collect();

    // hash them all
    let h = &mut DefaultHasher::new();
    intersection.hash(h);
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

pub struct HashableConfigMap(ConfigMap);
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
