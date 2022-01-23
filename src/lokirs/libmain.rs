use anyhow::{bail, Result};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::Serialize;
use std::io;

use super::ssd;
use crate::{
    paras::{resource::Resource, valid::Validation},
    validate,
};

pub fn main() -> Result<()> {
    let ssd: ssd::SSD = Default::default();
    let mut r = runner();

    for resource in ssd.resources().into_iter() {
        r.push_resource(resource);
    }

    r.run()
}

pub fn runner() -> Runner {
    let mut r: Runner = Default::default();
    for v in vec![validate!(
        |x: &ObjectMeta| x.name.is_some(),
        Deployment,
        StatefulSet,
        Service,
        ConfigMap
    )] {
        r.push_validation(v);
    }
    r
}

#[derive(Clone, Default)]
pub struct Runner {
    rs: Vec<Resource>,
    validations: Vec<fn(&Resource) -> bool>,
}

impl Runner {
    pub fn push_resource(&mut self, x: Resource) {
        self.rs.push(x)
    }

    pub fn push_validation(&mut self, v: fn(&Resource) -> bool) {
        self.validations.push(v)
    }

    pub fn validate(&self) -> bool {
        for r in &self.rs {
            for v in &self.validations {
                if !v(r) {
                    return false;
                }
            }
        }
        true
    }

    pub fn run(&self) -> Result<()> {
        let mut serializer = serde_yaml::Serializer::new(io::stdout());
        if !self.validate() {
            bail!("error validating");
        }

        for r in &self.rs {
            r.serialize(&mut serializer)?;
        }

        Ok(())
    }
}
