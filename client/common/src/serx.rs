use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct SearchQuery {}

#[derive(Deserialize, Serialize)]
struct SerX {
    server: String,
}

impl SerX {
    fn from_config() -> Option<SerX> {
        use std::{io::Read, path::PathBuf};

        // config file location = ~/.config/ze-gourm-db.toml
        let mut cfgpath = PathBuf::new();
        cfgpath.push(dirs::config_dir()?);
        cfgpath.push("ze-gourm-db.toml");
        let cfgpath = cfgpath;

        let mut s = String::new();
        std::fs::File::open(cfgpath)
            .ok()?
            .read_to_string(&mut s)
            .ok()?;
        Some(toml::from_str(&s).ok()?)
    }

    fn from_env() -> Option<SerX> {
        Some(Self {
            server: std::env::var("ZE_GOURM_DB_SERVER").ok()?,
        })
    }

    pub fn try_new() -> Option<SerX> {
        Self::from_config().or_else(Self::from_env)
    }
}
