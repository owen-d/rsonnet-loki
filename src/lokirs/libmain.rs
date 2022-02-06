use crate::paras::{
    fold::Folder,
    matches::Matches,
    resource::{Object, Resource},
    valid::Validator,
};

use super::ssd;
use anyhow::{bail, Context, Result};
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

    r.push_validation(Box::new(|c: &Container| match c.args {
        Some(_) => Ok(()),
        None => {
            bail!("arguments required for container");
        }
    }));

    for resource in ssd.resources().into_iter() {
        r.push(resource);
    }

    r.run()
}

#[derive(Default)]
pub struct Runner {
    rs: Vec<Object>,
    validations: Vec<Box<dyn Validator<Object>>>,
    mappers: Vec<Box<dyn Folder<Object, Object>>>,
}

impl Runner {
    pub fn push(&mut self, x: Resource) {
        self.rs.push(x.into())
    }

    pub fn push_validation<A: 'static>(&mut self, f: Box<dyn Fn(&A) -> Result<()>>)
    where
        Object: Matches<A>,
    {
        self.validations.push(Box::new(f))
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
                v.validate(r)?;
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
        self.map().context("error transforming resources")?;
        self.validate().context("error validating resources")?;
        for r in &self.rs {
            r.serialize(&mut serializer)?;
        }

        Ok(())
    }
}
