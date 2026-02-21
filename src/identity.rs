use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
};

use uuid::Uuid;

pub fn load_or_create_identity(path: &str) -> anyhow::Result<String> {
    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer.trim().to_string())
        }

        Err(err) if err.kind() == ErrorKind::NotFound => {
            let uuid = Uuid::new_v4().to_string();
            let mut file = File::create(path)?;
            file.write_all(uuid.as_bytes())?;
            Ok(uuid)
        }
        Err(err) => Err(err.into()),
    }
}
