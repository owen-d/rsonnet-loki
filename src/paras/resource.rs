use anyhow::{bail, Result};
use derive_more::From;
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec, StatefulSet, StatefulSetSpec},
        core::v1::{
            Affinity, ConfigMap, Container, Pod, PodSpec, PodTemplateSpec, Service, ServiceSpec,
            Volume,
        },
    },
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
};
use serde::{Deserialize, Serialize};

use crate::{impl_fold, impl_from_chain};

use super::fold::Foldable;

/// A Resource is an exposed top level k8s API type.
/// These are what we output and send to k8s for management.
#[derive(Clone, Serialize, Deserialize, From)]
pub enum Resource {
    StatefulSet(StatefulSet),
    Deployment(Deployment),
    Service(Service),
    ConfigMap(ConfigMap),
    // Intended to be unused. This helps prevent unreachable errors by having a nonsense constructor
    // to match against when validate! macros act on all legitimate resource types.
    Nothing,
}

// sts
impl_from_chain!(StatefulSet, Resource, Object);
impl_fold!(
    StatefulSet,
    [Resource::StatefulSet, Object::Resource],
    metadata,
    spec
);
// dep
impl_from_chain!(Deployment, Resource, Object);
impl_fold!(
    Deployment,
    [Resource::Deployment, Object::Resource],
    metadata,
    spec
);
// svc
impl_from_chain!(Service, Resource, Object);
impl_fold!(
    Service,
    [Resource::Service, Object::Resource],
    metadata,
    spec
);
// cfg
impl_from_chain!(ConfigMap, Resource, Object);
impl_fold!(ConfigMap, [Resource::ConfigMap, Object::Resource], metadata);

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
    DeploymentSpec(DeploymentSpec),
    StatefulSetSpec(StatefulSetSpec),
    Affinity(Affinity),
    ServiceSpec(ServiceSpec),
}

impl_fold!(Container, [Object::Container]);
impl_fold!(ObjectMeta, ObjectMeta);
impl_fold!(Pod, Pod, metadata, spec);
impl_fold!(PodTemplateSpec, PodTemplateSpec, metadata, spec);
impl_fold!(PodSpec, PodSpec, containers, affinity, volumes);
impl_fold!(Volume, Volume);
impl_fold!(Affinity, Affinity);
impl_fold!(StatefulSetSpec, StatefulSetSpec, template);
impl_fold!(DeploymentSpec, DeploymentSpec, template);
impl_fold!(ServiceSpec, ServiceSpec);

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
            Object::Affinity(val) => val.fold(f).map(Into::into),
            Object::StatefulSetSpec(val) => val.fold(f).map(Into::into),
            Object::DeploymentSpec(val) => val.fold(f).map(Into::into),
            Object::ServiceSpec(val) => val.fold(f).map(Into::into),
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
