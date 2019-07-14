use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct SerX {
    pub server: String,
}

impl SerX {
    fn get_cfgpath() -> Option<std::path::PathBuf> {
        // config file location = ~/.config/ze-gourm-db.toml
        let mut cfgpath = std::path::PathBuf::new();
        cfgpath.push(dirs::config_dir()?);
        cfgpath.push("ze-gourm-db.toml");
        Some(cfgpath)
    }

    fn from_config() -> Option<SerX> {
        use std::io::Read;
        let cfgpath = Self::get_cfgpath()?;

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

    pub fn write_config(&self) -> Option<()> {
        let cfgpath = Self::get_cfgpath()?;
        let s = toml::to_string(&self).ok()?;
        std::fs::write(cfgpath, s).ok()?;
        Some(())
    }

    pub fn search(&self, q: &str) -> Result<Vec<usize>, failure::Error> {
        let mut fq = HashMap::new();
        fq.insert("endpoint", "search");
        fq.insert("query", q);
        let resp: Vec<usize> = Client::new().post(&self.server)
            .json(&fq)
            .send()?
            .json()?;
        Ok(resp)
    }
}
