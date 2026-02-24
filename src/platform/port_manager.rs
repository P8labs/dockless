use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

const DEFAULT_PORT_RANGE_START: u16 = 8100;
const DEFAULT_PORT_RANGE_END: u16 = 8999;

#[derive(Serialize, Deserialize)]
struct PortsFile {
    version: u32,
    port_range_start: u16,
    port_range_end: u16,
    allocations: HashMap<String, u16>,
}

pub struct PortManager {
    path: String,
    port_range_start: u16,
    port_range_end: u16,
    allocations: HashMap<String, u16>,
}

impl PortManager {
    pub fn load_or_init(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            let empty = PortsFile {
                version: 1,
                port_range_start: DEFAULT_PORT_RANGE_START,
                port_range_end: DEFAULT_PORT_RANGE_END,
                allocations: HashMap::new(),
            };

            let json = serde_json::to_string_pretty(&empty)
                .context("failed to serialize initial ports file")?;

            fs::write(path, json)
                .with_context(|| format!("failed to create ports file at {}", path))?;
        }

        Self::load(path)
    }

    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read ports file at {}", path))?;

        let file: PortsFile = serde_json::from_str(&content)
            .with_context(|| format!("failed to parse ports JSON at {}", path))?;

        if file.version != 1 {
            anyhow::bail!("unsupported ports file version {}", file.version);
        }

        Ok(Self {
            path: path.to_string(),
            port_range_start: file.port_range_start,
            port_range_end: file.port_range_end,
            allocations: file.allocations,
        })
    }

    pub fn allocate(&mut self, service_id: &str) -> Result<u16> {
        if let Some(&port) = self.allocations.get(service_id) {
            return Ok(port);
        }

        let allocated_ports: std::collections::HashSet<u16> =
            self.allocations.values().copied().collect();

        for port in self.port_range_start..=self.port_range_end {
            if !allocated_ports.contains(&port) {
                self.allocations.insert(service_id.to_string(), port);
                self.save()?;
                return Ok(port);
            }
        }

        anyhow::bail!(
            "no available ports in range {}..{}",
            self.port_range_start,
            self.port_range_end
        )
    }

    pub fn deallocate(&mut self, service_id: &str) -> Result<()> {
        if self.allocations.remove(service_id).is_some() {
            self.save()?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn get_port(&self, service_id: &str) -> Option<u16> {
        self.allocations.get(service_id).copied()
    }

    pub fn all_allocations(&self) -> &HashMap<String, u16> {
        &self.allocations
    }

    fn save(&self) -> Result<()> {
        let file = PortsFile {
            version: 1,
            port_range_start: self.port_range_start,
            port_range_end: self.port_range_end,
            allocations: self.allocations.clone(),
        };

        let json = serde_json::to_string_pretty(&file).context("failed to serialize ports file")?;

        let tmp_path = format!("{}.tmp", self.path);

        fs::write(&tmp_path, json)
            .with_context(|| format!("failed to write temp ports file {}", tmp_path))?;

        fs::rename(&tmp_path, &self.path)
            .with_context(|| format!("failed to replace ports file {}", self.path))?;

        Ok(())
    }
}
