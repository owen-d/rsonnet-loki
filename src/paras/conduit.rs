use k8s_openapi::api::{
    apps::v1::{Deployment, StatefulSet},
    core::v1::{ConfigMap, Service},
};

use super::conventions::HasMany;

pub trait Validate<T, F: FnMut(T) -> bool> {
    fn validate(&self, f: F) -> bool;
}

impl<A, B, F> Validate<A, F> for B
where
    B: HasMany<A>,
    F: Fn(A) -> bool,
{
    fn validate(&self, f: F) -> bool {
        self.get_all().unwrap_or_default().into_iter().all(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::paras::conventions::{Name, With};

    use super::*;
    use k8s_openapi::{
        api::core::v1::PodTemplateSpec, apimachinery::pkg::apis::meta::v1::ObjectMeta,
    };

    #[test]
    fn test_validate() {
        let no_name = |x: ObjectMeta| x.name.is_none();
        let pt: PodTemplateSpec = Default::default();
        let altered = pt.clone().with(Name::new("foo".to_string()));

        assert_eq!(true, pt.validate(no_name));
        assert_eq!(false, altered.validate(no_name));
    }
}
