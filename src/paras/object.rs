use derive_more::From;
use serde::{Deserialize, Serialize};

/// An Object is a k8s API type.
/// It encompasses both exposed and unexposed object.
/// These are subject to mappings and validations.
#[derive(Clone, Serialize, Deserialize, From)]
pub enum Object {
    Resource(super::resource::Resource),
}
