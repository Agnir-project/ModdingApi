extern crate agnir_modding_api;
use std::path::PathBuf;

use agnir_modding_api::*;

fn main() {
    let plugins = unsafe {
        match PluginManager::load_plugin_folder(PathBuf::from("./plugins")) {
            Ok(plugins) => {
                println!("{:?}", plugins);
                plugins
            }
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1)
            }
        }
    };
    PluginManager::unload_plugins(plugins);
}
