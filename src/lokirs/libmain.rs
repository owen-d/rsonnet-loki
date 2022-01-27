use crate::paras::resource::Object;

use super::ssd;
use anyhow::{Context, Result};
use serde::Serialize;
use std::io;

pub fn main() -> Result<()> {
    let ssd: ssd::SSD = Default::default();
    let mut r = runner();

    for resource in ssd.resources().into_iter() {
        r.push(resource.into());
    }

    r.run()
}

pub fn runner() -> Runner {
    let mut r: Runner = Default::default();
    for v in vec![] {
        r.push_validation(v);
    }
    r
}

#[derive(Clone, Default)]
pub struct Runner {
    rs: Vec<Object>,
    validations: Vec<fn(&Object) -> Result<()>>,
}

impl Runner {
    pub fn push(&mut self, x: Object) {
        self.rs.push(x)
    }

    pub fn push_validation(&mut self, v: fn(&Object) -> Result<()>) {
        self.validations.push(v)
    }

    pub fn validate(&self) -> Result<()> {
        for r in &self.rs {
            for v in &self.validations {
                v(r)?;
            }
        }
        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        let mut serializer = serde_yaml::Serializer::new(io::stdout());
        self.validate().context("error validatin")?;
        for r in &self.rs {
            r.serialize(&mut serializer)?;
        }

        Ok(())
    }
}
