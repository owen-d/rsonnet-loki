use anyhow::Result;
use serde::Serialize;
use std::io;

use super::ssd;
use crate::paras::conduit::Resource;

pub fn main() -> Result<()> {
    let mut serializer = serde_yaml::Serializer::new(io::stdout());
    let ssd: ssd::SSD = Default::default();

    let resources: Vec<Resource> = vec![
        ssd.read_svc().into(),
        ssd.config_map().into(),
        ssd.read_deployment().into(),
        ssd.write_svc().into(),
        ssd.write_sts().into(),
    ];

    // ssd.read_svc().serialize(&mut serializer)?;
    // ssd.config_map().serialize(&mut serializer)?;
    // ssd.read_deployment().serialize(&mut serializer)?;
    // ssd.write_svc().serialize(&mut serializer)?;
    // ssd.write_sts().serialize(&mut serializer)?;
    Ok(())
}
