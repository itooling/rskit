#[cfg(test)]
use std::{thread, time::Duration};

use fast_log::{
    consts::LogSize,
    error::LogError,
    plugin::{
        file_split::{KeepType, Rolling, RollingType},
        packer::LogPacker,
    },
    Config, Logger,
};

pub mod base;
pub mod crypto;
pub mod err;
pub mod sd;
pub mod sha;

#[cfg(feature = "ecdh")]
pub mod ecdh;

pub struct Log<'a> {
    pub file_path: &'a str,
    pub rolling: Rolling,
    pub keep_type: KeepType,
    pub packer: LogPacker,
}
impl Default for Log<'_> {
    fn default() -> Self {
        Log {
            file_path: "log/app.log",
            rolling: Rolling::new(RollingType::BySize(LogSize::MB(100))),
            keep_type: KeepType::All,
            packer: LogPacker {},
        }
    }
}

impl Log<'_> {
    pub fn init(&self) -> Result<&'static Logger, LogError> {
        fast_log::init(
            Config::new()
                .chan_len(Some(100000))
                .file("log/app.log")
                .console(),
        )
    }

    pub fn init_split(self) -> Result<&'static Logger, LogError> {
        fast_log::init(
            fast_log::Config::new()
                .console()
                .chan_len(Some(100000))
                .file_split(self.file_path, self.rolling, self.keep_type, self.packer),
        )
    }
}

#[test]
fn test_log() {
    Log::default().init().unwrap();
    log::info!("test log ...");
    thread::sleep(Duration::from_secs(1));
}
