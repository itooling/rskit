pub mod tools;

use std::{
    any::Any,
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use fast_log::{
    consts::LogSize,
    error::LogError,
    plugin::{
        file_split::{KeepType, Rolling, RollingType},
        packer::LogPacker,
    },
    Config, Logger,
};
use log::LevelFilter;
use serde::{Deserialize, Serialize};

type Cache = HashMap<String, Box<dyn Any + Send + Sync>>;

pub static CACHE: LazyLock<RwLock<Cache>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn cache_set<V>(k: &str, v: V) -> Option<V>
where
    V: Any + Send + Sync + Clone,
{
    let mut map = CACHE.write().unwrap();
    map.insert(k.to_string(), Box::new(v))?
        .downcast_ref::<V>()
        .cloned()
}

pub fn cache_get<V>(k: &str) -> Option<V>
where
    V: Any + Send + Sync + Clone,
{
    let map = CACHE.read().unwrap();
    map.get(k)?.downcast_ref::<V>().cloned()
}

pub struct Log {
    pub chan: Option<usize>,
    pub path: String,
    pub roll: Rolling,
    pub keep: KeepType,
    pub packer: LogPacker,
    pub level: LevelFilter,
}

impl Default for Log {
    fn default() -> Self {
        let dir = match std::env::current_dir() {
            Ok(p) => p.to_str().expect("current dir error").to_string(),
            Err(_) => "./".to_string(),
        };
        Log {
            chan: Some(100000),
            path: format!("{}/logs/app.log", dir),
            roll: Rolling::new(RollingType::BySize(LogSize::MB(100))),
            keep: KeepType::KeepNum(10),
            packer: LogPacker {},
            level: log::LevelFilter::Info,
        }
    }
}

impl Log {
    pub fn init(&self) -> Result<&'static Logger, LogError> {
        fast_log::init(
            Config::new()
                .level(self.level)
                .chan_len(self.chan)
                .console(),
        )
    }

    pub fn init_file(&self) -> Result<&'static Logger, LogError> {
        fast_log::init(
            Config::new()
                .level(self.level)
                .chan_len(self.chan)
                .file(&self.path)
                .console(),
        )
    }

    pub fn init_split(self) -> Result<&'static Logger, LogError> {
        fast_log::init(
            fast_log::Config::new()
                .level(self.level)
                .chan_len(self.chan)
                .file_split(&self.path, self.roll, self.keep, self.packer)
                .console(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct App {
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    app: App,
}

pub fn init_config() {
    let name = std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());
    match config::Config::builder()
        .add_source(config::File::with_name(&format!("{}.toml", name)).required(false))
        .add_source(config::File::with_name("app.toml"))
        .build()
    {
        Ok(cfg) => match cfg.try_deserialize::<Settings>() {
            Ok(s) => {
                println!("settings: {:?}", s);
            }
            Err(e) => log::error!("deserialize config error: {:?}", e),
        },
        Err(e) => {
            log::error!("init config error: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        use std::{thread, time::Duration};
        Log::default().init().unwrap();
        log::info!("init log ...");
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_config() {
        init_config();
        log::info!("init config ...");
    }
}
