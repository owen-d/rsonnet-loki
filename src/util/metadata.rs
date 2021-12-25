use super::conventions::{Has, HasMut};
use k8s_openapi::api::core::v1::PodTemplateSpec;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

impl<T> Has<ObjectMeta> for T
where
    T: Has<PodTemplateSpec>,
{
    fn get(&self) -> Option<ObjectMeta> {
        self.get().and_then(|x| x.metadata)
    }
}

impl<T> HasMut<ObjectMeta> for T
where
    T: HasMut<PodTemplateSpec>,
{
    fn with(&self, x: ObjectMeta) -> Self {
        self.with(PodTemplateSpec {
            metadata: Some(x),
            ..self.get().unwrap_or_default().clone()
        })
    }
}

pub type Name = String;
pub const K8S_NAME_KEY: &str = "name";

impl<T> Has<Name> for T
where
    T: Has<ObjectMeta>,
{
    fn get(&self) -> Option<Name> {
        self.get()
            .and_then(|x| x.labels)
            .map(|ls: std::collections::BTreeMap<String, String>| ls[K8S_NAME_KEY].clone())
    }
}

impl<T> HasMut<Name> for T
where
    T: HasMut<ObjectMeta>,
{
    fn with(&self, x: Name) -> Self {
        let ls = self
            .get()
            .and_then(|x: ObjectMeta| x.labels)
            .or(Some(Default::default()))
            .map(|ls| {
                let mut res = ls.clone();
                res.extend([(K8S_NAME_KEY.to_string(), x)]);
                res
            });

        self.with(ObjectMeta {
            labels: ls,
            ..self.get().unwrap_or_default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_name_roundtrip() {
        let tpl: PodTemplateSpec = Default::default();
        let seeded = tpl.with("foo".to_string() as Name);
        assert_eq!(Some("foo".to_string()), seeded.get());
    }
}
