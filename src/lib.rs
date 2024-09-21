pub mod base;

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

#[derive(Default)]
pub struct Configs<T: Serialize + Deserialize<'static> + Default> {
    config: T,
}

impl<T: Serialize + Deserialize<'static> + Default> Configs<T> {
    pub fn new() -> Self {
        Configs::default()
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
                    self.config = s;
                    return Some(&self.config);
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
        Log::default().init().unwrap();
        log::info!("init log ...");
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_config() {
        let mut settings = Configs::<Settings>::new();
        let config = settings.init(None).unwrap();
        log::info!("version:{}", config.app.version);
    }
}
