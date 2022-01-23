use super::conventions::HasMany;

#[macro_export]
macro_rules! validate {
    ( $validation: expr, $($constructor:ident ),+ ) => {
        {
            use $crate::paras::resource::Resource;
            |r: &Resource| {
                match r {
                    $(
                        Resource::$constructor(val) => val.validate($validation),
                    )*
                        _ => true
                }
            }

        }
    };
}

pub trait Validation<T, F: Fn(&T) -> bool> {
    fn validate(&self, f: F) -> bool;
}

impl<A, B, F> Validation<A, F> for B
where
    B: HasMany<A>,
    F: Fn(&A) -> bool,
{
    fn validate(&self, f: F) -> bool {
        self.get_all().unwrap_or_default().iter().all(f)
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
