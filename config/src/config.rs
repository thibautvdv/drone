#![allow(missing_docs)]

use crate::deserialize_size;
use serde::{Deserialize, Serialize};

/// Config object.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub memory: Memory,
    pub heap: Heap,
    pub linker: Option<Linker>,
    pub probe: Option<Probe>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Memory {
    pub flash: MemoryFlash,
    pub ram: MemoryRam,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MemoryFlash {
    #[serde(deserialize_with = "deserialize_size")]
    pub size: u32,
    pub origin: u32,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MemoryRam {
    #[serde(deserialize_with = "deserialize_size")]
    pub size: u32,
    pub origin: u32,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Heap {
    #[serde(deserialize_with = "deserialize_size")]
    pub size: u32,
    pub pools: Vec<HeapPool>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Linker {
    #[serde(default)]
    pub include: Vec<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HeapPool {
    #[serde(deserialize_with = "deserialize_size")]
    pub block: u32,
    pub capacity: u32,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Probe {
    pub gdb_client_command: String,
    pub bmp: Option<ProbeBmp>,
    pub jlink: Option<ProbeJlink>,
    pub openocd: Option<ProbeOpenocd>,
    pub swo: Option<ProbeSwo>,
    pub dso: Option<ProbeDso>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProbeBmp {
    pub device: String,
    pub gdb_endpoint: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProbeJlink {
    pub gdb_server_command: String,
    pub commander_command: String,
    pub device: String,
    pub speed: u32,
    pub port: u32,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProbeOpenocd {
    pub command: String,
    pub port: u32,
    pub arguments: Vec<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProbeSwo {
    pub reset_freq: u32,
    pub baud_rate: u32,
    pub serial_endpoint: Option<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProbeDso {
    pub baud_rate: u32,
    pub serial_endpoint: String,
}
