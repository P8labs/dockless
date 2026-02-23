use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
};

use anyhow::{Context, Result};
use uuid::Uuid;

pub fn load_or_create_identity(path: &str) -> Result<String> {
    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .with_context(|| format!("failed to read node identity file at {}", path))?;

            let content = buffer.trim();

            let uid = Uuid::parse_str(content)
                .with_context(|| format!("node identity file at {} is corrupted", path))?;

            Ok(uid.to_string())
        }

        Err(err) if err.kind() == ErrorKind::NotFound => {
            let uuid = Uuid::new_v4().to_string();

            let mut file = File::create(path)
                .with_context(|| format!("failed to create node identity file at {}", path))?;

            file.write_all(uuid.as_bytes())
                .with_context(|| format!("failed to write node identity file at {}", path))?;

            Ok(uuid)
        }

        Err(err) => Err(err)
            .with_context(|| format!("unexpected error accessing node identity file at {}", path)),
    }
}
