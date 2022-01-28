use crate::paras::{
    fold::Foldable,
    resource::{Object, Resource},
};

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
    for m in vec![|o: Object| {
        if let Object::Container(mut c) = o {
            c.image = Some("grafana/loki:main".to_string());
            return c.into();
        };
        o
    }] {
        r.push_mapper(m);
    }
    r
}

#[derive(Clone, Default)]
pub struct Runner {
    rs: Vec<Object>,
    validations: Vec<fn(&Object) -> Result<()>>,
    mappers: Vec<fn(Object) -> Object>,
}

impl Runner {
    pub fn push(&mut self, x: Resource) {
        self.rs.push(x.into())
    }

    pub fn push_validation(&mut self, v: fn(&Object) -> Result<()>) {
        self.validations.push(v)
    }

    pub fn push_mapper(&mut self, v: fn(Object) -> Object) {
        self.mappers.push(v)
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
                    mapped = mapped.fold(*f)?;
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
