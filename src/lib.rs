pub mod tools;

use fast_log::{
    consts::LogSize,
    error::LogError,
    plugin::{
        file_split::{KeepType, Rolling, RollingType},
        packer::LogPacker,
    },
    Config, Logger,
};
use serde::{Deserialize, Serialize};

pub struct Log<'a> {
    pub chan: Option<usize>,
    pub path: &'a str,
    pub roll: Rolling,
    pub keep: KeepType,
    pub packer: LogPacker,
    pub level: log::LevelFilter,
}
impl Default for Log<'_> {
    fn default() -> Self {
        Log {
            chan: Some(100000),
            path: "./logs/app.log",
            roll: Rolling::new(RollingType::BySize(LogSize::MB(100))),
            keep: KeepType::KeepNum(10),
            packer: LogPacker {},
            level: log::LevelFilter::Info,
        }
    }
}

impl Log<'_> {
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
                .file(self.path)
                .console(),
        )
    }

    pub fn init_split(self) -> Result<&'static Logger, LogError> {
        fast_log::init(
            fast_log::Config::new()
                .level(self.level)
                .chan_len(self.chan)
                .file_split(self.path, self.roll, self.keep, self.packer)
                .console(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    host: String,
    port: usize,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    database: Database,
}

pub fn init_config() {
    let name = std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());
    match config::Config::builder()
        .add_source(config::File::with_name("app.toml").required(false))
        .add_source(config::File::with_name(&format!("{}.toml", name)))
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
        log::info!("test log ...");
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_config() {
        init_config();
    }
}
