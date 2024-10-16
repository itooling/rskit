#[cfg(feature = "default")]
pub mod base;

#[cfg(feature = "default")]
pub use base::*;

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

pub use log;

#[cfg(debug_assertions)]
pub const RUST_LOG: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
pub const RUST_LOG: LevelFilter = LevelFilter::Info;

pub struct Log {
    pub chan: Option<usize>,
    pub path: String,
    pub roll: Rolling,
    pub keep: KeepType,
    pub packer: LogPacker,
    pub level: LevelFilter,
}

impl Log {
    pub fn new() -> Self {
        let dir = match std::env::current_dir() {
            Ok(p) => p.to_str().expect("current dir error").to_string(),
            Err(_) => "./".to_string(),
        };
        Log {
            chan: Some(100000),
            path: format!("{}/log/app.log", dir),
            roll: Rolling::new(RollingType::BySize(LogSize::MB(100))),
            keep: KeepType::KeepNum(10),
            packer: LogPacker {},
            level: RUST_LOG,
        }
    }

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

pub struct Configs<T: Serialize + Deserialize<'static>> {
    config: Option<T>,
}

impl<T: Serialize + Deserialize<'static>> Configs<T> {
    pub fn new() -> Self {
        Configs { config: None }
    }
    pub fn init(&mut self, name: Option<String>) -> Option<&T> {
        match config::Config::builder()
            .add_source(
                config::File::with_name(&format!("{}.toml", name.unwrap_or("app".to_string())))
                    .required(false),
            )
            .build()
        {
            Ok(cfg) => match cfg.try_deserialize::<T>() {
                Ok(s) => {
                    self.config = Some(s);
                    return self.config.as_ref();
                }
                Err(e) => {
                    log::error!("deserialize config error: {:?}", e);
                    None
                }
            },
            Err(e) => {
                log::error!("init config error: {:?}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Settings {
        app: App,
    }
    #[derive(Debug, Serialize, Deserialize, Default)]
    struct App {
        version: String,
    }

    #[test]
    fn test_log() {
        use std::{thread, time::Duration};
        Log::new().init_file().unwrap();
        log::info!("init log ...");
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_config() {
        let mut config = Configs::<Settings>::new();
        let settings = config.init(None).unwrap();
        println!("version: {}", settings.app.version);
    }
}
