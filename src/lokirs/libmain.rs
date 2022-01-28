use crate::{
    map,
    paras::{
        fold::Foldable,
        resource::{Object, Resource},
    },
};

use super::ssd;
use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::Container;
use serde::Serialize;
use std::io;

pub fn main() -> Result<()> {
    let ssd: ssd::SSD = Default::default();
    let mut r: Runner = Default::default();
    // Manual way, yuck!
    r.push_mapper(Box::new(|o: Object| {
        if let Object::Container(mut c) = o {
            c.image = Some("grafana/loki:main".to_string());
            return c.into();
        };
        o
    }));
    // Much better!
    let f = |mut c: Container| {
        c.image = Some("grafana/loki:oops".to_string());
        c
    };
    r.push_mapper(map!(f, Object::Container));

    // New hotness
    // Any container will have it's image changed, whether
    // it's embedded in a deployment, statefulset, etc.
    // This will also work for your own CRDs.
    r.push_mapper(map!(
        |mut c: Container| {
            c.image = Some("grafana/loki:oops".to_string());
            c
        },
        Object::Container
    ));

    for resource in ssd.resources().into_iter() {
        r.push(resource.into());
    }

    r.run()
}

#[derive(Default)]
pub struct Runner {
    rs: Vec<Object>,
    validations: Vec<Box<dyn Fn(&Object) -> Result<()>>>,
    mappers: Vec<Box<dyn Fn(Object) -> Object>>,
}

impl Runner {
    pub fn push(&mut self, x: Resource) {
        self.rs.push(x.into())
    }

    pub fn push_validation(&mut self, f: Box<dyn Fn(&Object) -> Result<()>>) {
        self.validations.push(f)
    }

    pub fn push_mapper(&mut self, f: Box<dyn Fn(Object) -> Object>) {
        self.mappers.push(f)
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
                    mapped = mapped.fold(&*f)?;
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
