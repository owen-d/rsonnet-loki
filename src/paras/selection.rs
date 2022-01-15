use crate::builtin::{Has, Name};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use maplit::btreemap;

impl<T> Has<LabelSelector> for T
where
    T: Has<Name>,
{
    fn get(&self) -> Option<LabelSelector> {
        self.get().map(|name| LabelSelector {
            match_labels: Some(btreemap! {
                Name::key() => name.into(),
            }),
            ..Default::default()
        })
    }
}
