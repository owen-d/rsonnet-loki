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

impl_fold!(Resource, Resource);
impl_fold!(Container, Container);
impl_fold!(ObjectMeta, ObjectMeta);
impl_fold!(Pod, Pod);
impl_fold!(PodTemplateSpec, PodTemplateSpec);
impl_fold!(PodSpec, PodSpec);
impl_fold!(Volume, Volume);

impl Foldable<Object> for Object {
    fn fold(self, f: fn(Object) -> Object) -> Result<Self> {
        if let Object::PodTemplateSpec(val) = self {
            return val.fold(f).map(Object::PodTemplateSpec);
        }
        bail!("unimplemented");
    }
}
