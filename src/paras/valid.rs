// use anyhow::Result;

// macro to wrap f(&x) -> Result<()> into
// decons(input) > f(&input)

// pub trait Validation {
//     fn validate(&self, &T) ->
// }

// impl Validation for fn(Object) -> Result<Object> where {
//     fn validate(&self, f: fn(&Self) -> Result<()>) -> Result<()> {

//     }
// }

#[cfg(test)]
mod tests {
    use crate::paras::conventions::{Name, With};

    use super::*;
    use k8s_openapi::{
        api::core::v1::PodTemplateSpec, apimachinery::pkg::apis::meta::v1::ObjectMeta,
    };

    #[test]
    fn test_validate() {
        let no_name = |x: &ObjectMeta| x.name.is_none();
        let pt: PodTemplateSpec = Default::default();
        let altered = pt.with(Name::new("foo".to_string()));

        assert!(pt.validate(no_name));
        assert!(!altered.validate(no_name));
    }

    #[test]
    fn test_validate_macro() {
        let no_name = |x: &ObjectMeta| x.name.is_none();
        let _v = validate!(no_name, Deployment, StatefulSet);
    }
}
