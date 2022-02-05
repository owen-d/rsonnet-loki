use crate::paras::{
    fold::Folder,
    matches::Matches,
    resource::{Object, Resource},
};

use super::ssd;
use anyhow::{Context, Result};
use k8s_openapi::api::{apps::v1::StatefulSet, core::v1::Container};
use serde::Serialize;
use std::io;

pub fn main() -> Result<()> {
    let ssd: ssd::SSD = Default::default();
    let mut r: Runner = Default::default();

    r.push_mapper(Box::new(|mut c: Container| {
        c.image = Some("wa".to_string());
        c
    }));

    r.push_mapper(Box::new(|s: StatefulSet| s));

    for resource in ssd.resources().into_iter() {
        r.push(resource);
    }

    r.run()
}

#[derive(Default)]
pub struct Runner {
    rs: Vec<Object>,
    validations: Vec<Box<dyn Fn(&Object) -> Result<()>>>,
    mappers: Vec<Box<dyn Folder<Object>>>,
}

impl Runner {
    pub fn push(&mut self, x: Resource) {
        self.rs.push(x.into())
    }

    pub fn push_validation(&mut self, f: Box<dyn Fn(&Object) -> Result<()>>) {
        self.validations.push(f)
    }

    pub fn push_mapper<A: 'static>(&mut self, f: Box<dyn Fn(A) -> A>)
    where
        Object: From<A> + Matches<A>,
    {
        self.mappers.push(Box::new(f))
    }

    pub fn validate(&self) -> Result<()> {
        for r in &self.rs {
            for v in &self.validations {
                v(r)?;
            }
        }
        Ok(())
    }

    pub fn map(&mut self) -> Result<()> {
        let r: Result<Vec<Object>> = self
            .rs
            .drain(..)
            .map(|x: Object| -> Result<Object> {
                let mut mapped = x;
                for f in &self.mappers {
                    mapped = f.apply(mapped)?;
                }
                Ok(mapped)
            })
            .collect();

        match r {
            Ok(xs) => {
                self.rs = xs;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut serializer = serde_yaml::Serializer::new(io::stdout());
        self.map().context("error mapping resources")?;
        self.validate().context("error validatin")?;
        for r in &self.rs {
            r.serialize(&mut serializer)?;
        }

        Ok(())
    }
}
