use derive_more::{From, Into};
use k8s_openapi::api::core::v1::ConfigMap;

use std::hash::{Hash, Hasher};

pub const CONFIG_HASH_ANNOTATION: &str = "config_hash";

// pub fn with_config_hash<T>(cfgs: Vec<HashableConfigMap>, x: T) -> T
// where
//     T: Has<ObjectMeta> + With<ObjectMeta>,
// {
//     let h = &mut DefaultHasher::new();
//     cfgs.hash(h);
//     let hash = h.finish();

//     let ls = x
//         .get()
//         .and_then(|x: ObjectMeta| x.annotations)
//         .or_else(|| Some(Default::default()))
//         .map(|ls| {
//             let mut res = ls;
//             res.extend([(CONFIG_HASH_ANNOTATION.to_string(), format!("{:x}", hash))]);
//             res
//         });

//     x.with(ObjectMeta {
//         annotations: ls,
//         ..x.get().unwrap_or_default()
//     })
// }

#[derive(From, Into, Clone)]
pub struct HashableConfigMap(ConfigMap);

impl HashableConfigMap {
    pub fn new(x: ConfigMap) -> Self {
        Self(x)
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
