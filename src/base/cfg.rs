use std::{
    env,
    sync::{Arc, LazyLock, RwLock},
    time::Duration,
};

use config::{Config, File};
use notify::{Event, RecommendedWatcher, Watcher};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct App {
    pub version: String,
}

pub static AUTO_CONFIG: LazyLock<Arc<RwLock<Option<Settings>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(load())));

fn path() -> String {
    let current_dir = env::current_dir().expect("failed to get current directory");
    current_dir.join("app.toml").to_string_lossy().to_string()
}

fn load() -> Option<Settings> {
    match Config::builder()
        .add_source(File::with_name(&path()))
        .build()
    {
        Ok(c) => match c.try_deserialize() {
            Ok(s) => s,
            Err(e) => {
                log::error!("deserialize error: {e:?}");
                None
            }
        },
        Err(e) => {
            log::error!("config error: {e:?}");
            None
        }
    }
}
fn refresh() {
    if let Some(s) = load() {
        *AUTO_CONFIG.write().unwrap() = Some(s);
    }
}

fn show() {
    match AUTO_CONFIG.read() {
        Ok(config) => {
            println!("config: {:?}", config);
        }
        Err(e) => {
            log::error!("config error: {:?}", e);
        }
    }
}
fn watch() {
    let (tx, rx) = std::sync::mpsc::channel();
    let result: Result<RecommendedWatcher, _> = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(3)),
    );
    match result {
        Ok(mut watcher) => {
            match watcher.watch(
                std::path::Path::new(&path()),
                notify::RecursiveMode::NonRecursive,
            ) {
                Ok(_) => (),
                Err(e) => log::error!("watch error: {:?}", e),
            }

            loop {
                match rx.recv() {
                    Ok(Ok(Event {
                        kind: notify::event::EventKind::Modify(_),
                        ..
                    })) => {
                        log::info!("refreshing configuration ...");
                        refresh();
                        show();
                    }
                    Err(e) => log::error!("recv error: {e:?}"),
                    _ => (),
                }
            }
        }
        Err(e) => {
            log::error!("watcher error: {:?}", e);
        }
    }
}

pub fn init() {
    show();
    watch();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_config() {
        init();
    }
}
