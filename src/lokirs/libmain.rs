use anyhow::Result;
use serde::Serialize;
use std::io;

use super::read;
pub fn main() -> Result<()> {
    let mut serializer = serde_yaml::Serializer::new(io::stdout());
    let reads: read::Reads = Default::default();

    reads.svc().serialize(&mut serializer)?;
    reads.config_map().serialize(&mut serializer)?;
    reads.deployment().serialize(&mut serializer)?;
    Ok(())
}
