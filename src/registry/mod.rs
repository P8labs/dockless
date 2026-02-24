use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegistryFile {
    version: u32,
    services: Vec<ServiceDefinition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ServiceDefinition {
    pub id: String,
    pub name: String,

    #[serde(default)]
    pub ready: bool,

    #[serde(default)]
    pub binary_path: String,

    #[serde(default)]
    pub args: Vec<String>,

    #[serde(default)]
    pub env: HashMap<String, String>,

    #[serde(default = "default_auto_restart")]
    pub auto_restart: bool,

    #[serde(default)]
    pub restart_limit: Option<u32>,

    #[serde(default)]
    pub current_version: Option<String>,

    #[serde(skip)]
    pub port: Option<u16>,
}

fn default_auto_restart() -> bool {
    true
}

pub struct RegistryManager {
    path: String,
    definitions: Vec<ServiceDefinition>,
}

impl RegistryManager {
    pub fn load_or_init(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            let empty = RegistryFile {
                version: 1,
                services: vec![],
            };

            let json = serde_json::to_string_pretty(&empty)
                .context("failed to serialize initial registry")?;

            fs::write(path, json)
                .with_context(|| format!("failed to create registry file at {}", path))?;
        }

        Self::load(path)
    }
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read registry file at {}", path))?;

        let file: RegistryFile = serde_json::from_str(&content)
            .with_context(|| format!("failed to parse registry JSON at {}", path))?;

        if file.version != 1 {
            anyhow::bail!("unsupported registry version {}", file.version);
        }

        Ok(Self {
            path: path.to_string(),
            definitions: file.services,
        })
    }

    pub fn list_definitions(&self) -> &[ServiceDefinition] {
        &self.definitions
    }

    pub fn list_definitions_mut(&mut self) -> &mut Vec<ServiceDefinition> {
        &mut self.definitions
    }

    pub fn add(&mut self, def: ServiceDefinition) -> Result<()> {
        if self.definitions.iter().any(|s| s.id == def.id) {
            anyhow::bail!("service {} already exists with port {:?}", def.id, def.port);
        }

        self.definitions.push(def);
        Ok(())
    }

    pub fn update(&mut self, id: &str, def: ServiceDefinition) -> Result<()> {
        if let Some(existing) = self.definitions.iter_mut().find(|s| s.id == id) {
            *existing = def;
            Ok(())
        } else {
            anyhow::bail!("service {} not found", id);
        }
    }

    pub fn get(&self, id: &str) -> Option<&ServiceDefinition> {
        self.definitions.iter().find(|s| s.id == id)
    }

    pub fn remove(&mut self, id: &str) -> Result<()> {
        let original_len = self.definitions.len();
        self.definitions.retain(|s| s.id != id);

        if self.definitions.len() == original_len {
            anyhow::bail!("service {} not found", id);
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let file = RegistryFile {
            version: 1,
            services: self.definitions.clone(),
        };

        let json = serde_json::to_string_pretty(&file).context("failed to serialize registry")?;

        let tmp_path = format!("{}.tmp", self.path);

        fs::write(&tmp_path, json)
            .with_context(|| format!("failed to write temp registry file {}", tmp_path))?;

        fs::rename(&tmp_path, &self.path)
            .with_context(|| format!("failed to replace registry file {}", self.path))?;

        Ok(())
    }
}
