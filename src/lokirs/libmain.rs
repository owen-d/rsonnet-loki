use anyhow::Result;
use serde::Serialize;
use std::io;

use super::ssd;

pub fn main() -> Result<()> {
    let mut serializer = serde_yaml::Serializer::new(io::stdout());
    let ssd: ssd::SSD = Default::default();

    ssd.read_svc().serialize(&mut serializer)?;
    ssd.config_map().serialize(&mut serializer)?;
    ssd.read_deployment().serialize(&mut serializer)?;
    ssd.write_svc().serialize(&mut serializer)?;
    ssd.write_sts().serialize(&mut serializer)?;
    Ok(())
}
