extern crate agnir_modding_api;
use std::path::PathBuf;

use agnir_modding_api::*;

fn main() {
    let mut plugin_handler = PluginManager::default();

    unsafe {
        match plugin_handler.load_plugin_folder(PathBuf::from("./plugins")) {
            Ok(plugins) => {
                println!("{:?}", plugins.iter().map(|p| p.name()).collect::<Vec<&'static str>>());
            },
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
}
