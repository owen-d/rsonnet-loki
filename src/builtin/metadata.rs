use crate::paras::conventions::{Has, Name};
use k8s_openapi::api::core::v1::Volume;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use maplit::btreemap;

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

impl<A> Has<Option<Name>> for A
where
    A: Has<ObjectMeta>,
{
    fn has(&self) -> Option<Name> {
        self.has()
            .labels
            .map(|ls: std::collections::BTreeMap<String, String>| ls[&Name::key()].clone().into())
    }
}
