use anyhow::{bail, Result};
use derive_more::From;
use k8s_openapi::{
    api::{
        apps::v1 as apps,
        core::v1::{self as core, Container, Pod, PodSpec, PodTemplateSpec, Volume},
    },
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
};
use serde::{Deserialize, Serialize};

use crate::impl_fold;

use super::fold::Foldable;

/// A Resource is an exposed top level k8s API type.
/// These are what we output and send to k8s for management.
#[derive(Clone, Serialize, Deserialize, From)]
pub enum Resource {
    StatefulSet(apps::StatefulSet),
    Deployment(apps::Deployment),
    Service(core::Service),
    ConfigMap(core::ConfigMap),
    // Intended to be unused. This helps prevent unreachable errors by having a nonsense constructor
    // to match against when validate! macros act on all legitimate resource types.
    Nothing,
}

/// An Object is a k8s API type.
/// It encompasses both exposed and unexposed object.
/// These are subject to mappings and validations.
#[derive(Clone, Serialize, Deserialize, From)]
pub enum Object {
    Resource(super::resource::Resource),
    Container(Container),
    ObjectMeta(ObjectMeta),
    Pod(Pod),
    PodTemplateSpec(PodTemplateSpec),
    PodSpec(PodSpec),
    Volume(Volume),
}

// impl_fold!(Resource, Resource,);
impl_fold!(Container, Container,);
impl_fold!(ObjectMeta, ObjectMeta,);
impl_fold!(Pod, Pod,);
impl_fold!(PodTemplateSpec, PodTemplateSpec, metadata, spec);
impl_fold!(PodSpec, PodSpec,);
impl_fold!(Volume, Volume,);

impl Foldable<Object> for Object {
    fn fold(self, f: fn(Object) -> Object) -> Result<Self> {
        match self {
            Object::Resource(_) => bail!("unimplemented"),
            Object::Container(val) => val.fold(f).map(Into::into),
            Object::ObjectMeta(val) => val.fold(f).map(Into::into),
            Object::Pod(val) => val.fold(f).map(Into::into),
            Object::PodTemplateSpec(val) => val.fold(f).map(Into::into),
            Object::PodSpec(val) => val.fold(f).map(Into::into),
            Object::Volume(val) => val.fold(f).map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fold_basic() {
        let f = |x: Object| -> Object {
            if let Object::PodSpec(v) = x {
                return PodSpec {
                    dns_policy: Some("foo".to_string()),
                    ..v
                }
                .into();
            }
            x
        };

        let x = Object::PodTemplateSpec(PodTemplateSpec {
            spec: Some(Default::default()),
            ..Default::default()
        });

        let mapped = x.fold(f).unwrap();
        if let Object::PodTemplateSpec(p) = mapped {
            assert_eq!("foo".to_string(), p.spec.unwrap().dns_policy.unwrap())
        } else {
            panic!("unexpected variant")
        }
    }
}
