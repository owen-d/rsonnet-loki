use anyhow::Result;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
// use serde::Serialize;
// use std::io;

// use super::ssd;
use crate::{
    paras::{resource::Resource, valid::Validation},
    validate,
};

pub fn main() -> Result<()> {
    // let mut serializer = serde_yaml::Serializer::new(io::stdout());
    // let ssd: ssd::SSD = Default::default();

    // let resources: Vec<Resource> = vec![
    //     ssd.read_svc().into(),
    //     ssd.config_map().into(),
    //     ssd.read_deployment().into(),
    //     ssd.write_svc().into(),
    //     ssd.write_sts().into(),
    // ];

    // ssd.read_svc().serialize(&mut serializer)?;
    // ssd.config_map().serialize(&mut serializer)?;
    // ssd.read_deployment().serialize(&mut serializer)?;
    // ssd.write_svc().serialize(&mut serializer)?;
    // ssd.write_sts().serialize(&mut serializer)?;
    Ok(())
}

pub fn runner() -> Runner {
    let mut r: Runner = Default::default();
    for v in vec![validate!(
        |x: &ObjectMeta| x.name.is_some(),
        Deploy,
        Sts,
        Svc,
        CfgMap
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

    pub fn run(&mut self) -> bool {
        for r in &self.rs {
            for v in &self.validations {
                if !v(r) {
                    return false;
                }
            }
        }
        true
    }
}
