use std::{
    sync::{Arc, LazyLock, RwLock},
    time::Duration,
};

use config::{Config, File};
use notify::{Event, RecommendedWatcher, Watcher};

pub const CONFIG_PATH: &str = "app.toml";

pub static CONFIG: LazyLock<Arc<RwLock<Option<Config>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(load(CONFIG_PATH))));

pub fn load(path: &str) -> Option<Config> {
    match Config::builder().add_source(File::with_name(path)).build() {
        Ok(c) => Some(c),
        Err(e) => {
            println!("config error: {:?}", e);
            None
        }
    }
}
pub fn refresh() {
    *CONFIG.write().unwrap() = load(CONFIG_PATH);
}

pub fn show() {
    match CONFIG.read() {
        Ok(config) => {
            println!("config: {:?}", config);
        }
        Err(e) => {
            println!("config error: {:?}", e);
        }
    }
}

pub fn watch() {
    let (tx, rx) = std::sync::mpsc::channel();
    let result: Result<RecommendedWatcher, _> = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(3)),
    );
    match result {
        Ok(mut watcher) => {
            match watcher.watch(
                std::path::Path::new(CONFIG_PATH),
                notify::RecursiveMode::NonRecursive,
            ) {
                Ok(_) => (),
                Err(e) => println!("watch error: {:?}", e),
            }

            loop {
                match rx.recv() {
                    Ok(Ok(Event {
                        kind: notify::event::EventKind::Modify(_),
                        ..
                    })) => {
                        println!("refreshing configuration ...");
                        refresh();
                        show();
                    }
                    Err(e) => println!("recv error: {e:?}"),
                    _ => (),
                }
            }
        }
        Err(e) => {
            println!("watcher error: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        show();
        watch();
    }
}
