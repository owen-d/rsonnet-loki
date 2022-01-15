use super::conventions::{Has, With};
use derive_more::{From, Into};
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

// Name corresponds to the labelset ("name", <component>),
// which is used to determine the component. This is then
// used to to do things such bootstrap as anti-affinity rules.
#[derive(PartialEq, From, Into)]
pub struct Name(String);

impl Name {
    pub fn key() -> String {
        "name".to_string()
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
            .and_then(|x: ObjectMeta| x.labels)
            .or_else(|| Some(Default::default()))
            .map(|ls| {
                let mut res = ls;
                res.extend([(Name::key(), x.into())]);
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
