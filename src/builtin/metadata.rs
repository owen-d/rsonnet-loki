use crate::paras::conventions::{Has, Name, With};
use k8s_openapi::api::core::v1::{PodTemplateSpec, Volume};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use maplit::btreemap;

impl<T> Has<ObjectMeta> for T
where
    T: Has<PodTemplateSpec>,
{
    fn get(&self) -> Option<ObjectMeta> {
        self.get().and_then(|x| x.metadata)
    }
}

impl<T> With<ObjectMeta> for T
where
    T: With<PodTemplateSpec>,
{
    fn with(&self, x: ObjectMeta) -> Self {
        self.with(PodTemplateSpec {
            metadata: Some(x),
            ..self.get().unwrap_or_default()
        })
    }
}

impl From<Name> for LabelSelector {
    fn from(n: Name) -> Self {
        LabelSelector {
            match_labels: btreemap! {
                Name::key() => n.0,
            }
            .into(),
            ..Default::default()
        }
    }
}

impl From<Name> for ObjectMeta {
    fn from(x: Name) -> Self {
        ObjectMeta {
            labels: Some(btreemap! {
                Name::key() => String::from(x.clone()),
            }),
            name: Some(String::from(x)),
            ..Default::default()
        }
    }
}

impl From<Volume> for Name {
    fn from(v: Volume) -> Self {
        Self::new(v.name)
    }
}

impl<T> Has<Name> for T
where
    T: Has<ObjectMeta>,
{
    fn get(&self) -> Option<Name> {
        self.get()
            .and_then(|x| x.labels)
            .map(|ls: std::collections::BTreeMap<String, String>| ls[&Name::key()].clone().into())
    }
}

impl<T> With<Name> for T
where
    T: With<ObjectMeta>,
{
    fn with(&self, x: Name) -> Self {
        let ls = self
            .get()
            .or(Default::default())
            .and_then(|x: ObjectMeta| x.labels)
            .or(Default::default())
            .map(|ls| {
                let mut res = ls;
                res.extend([(Name::key(), x.clone().into())]);
                res
            });

        self.with(ObjectMeta {
            labels: ls,
            name: Some(x.to_string()),
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
        let seeded = tpl.with(Name::new("foo".to_string()));
        assert_eq!(Some(Name::new("foo".to_string())), seeded.get());
    }
}
