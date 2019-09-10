use crate::structs::Component;
use libloading::{Library, Symbol};
use std::any::Any;
use std::collections::HashMap;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fmt::Debug;
use std::path::PathBuf;

pub type PluginCreate<'a> = unsafe fn() -> *mut dyn Plugin;
pub type ComponentsHashMap = Vec<HashMap<String, Box<dyn Component>>>;
pub type SystemSymbol = unsafe fn(data: ComponentsHashMap);
pub type LoadedPlugin = Box<dyn Plugin>;

#[cfg(windows)]
fn is_lib(path: PathBuf) -> bool {
    match path.extension() {
        Some(extension) => {
            extension == "dll"
        },
        _ => false
    }
}

#[cfg(unix)]
fn is_lib(path: PathBuf) -> bool {
    match path.extension() {
        Some(extension) => {
            extension == "so"
        },
        _ => false
    }
}


pub trait Plugin: Any + Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn plugin_id(&self) -> &'static str;

    fn version(&self) -> &'static str;

    fn on_load(&self);

    fn systems(&self) -> Vec<&'static str>;
}

#[derive(Default)]
pub struct PluginManager<'symbol> {
    plugins: Vec<LoadedPlugin>,
    systems: HashMap<&'static str, Symbol<'symbol, SystemSymbol>>,
    loaded_libraries: HashMap<String, Library>,
}

impl<'symbol> PluginManager<'symbol> {
    pub unsafe fn load_plugin_folder(&mut self, folder_path: PathBuf) -> Result<Vec<Box<dyn Plugin>>, &'static str> {
        let plugins = read_dir(folder_path)
            .map_err(|e| "Cannot scan directory for plugins.")?
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
            .filter(|e| is_lib(e.path()))
            .filter_map(|e| {
                self.load_plugin(e.path()).ok()
            })
            .collect::<Vec<Box<dyn Plugin>>>();
        Ok(plugins)
    }

    pub unsafe fn load_plugin(&mut self, filename: PathBuf) -> Result<Box<dyn Plugin>, &'static str> {
        let lib = Library::new(filename).map_err(|_| "Unable to load the plugin")?;
        let constructor: Symbol<PluginCreate> = lib
            .get(b"_plugin_create")
            .map_err(|_| "The `_plugin_create` symbol wasn't found.")?;
        let boxed_raw = constructor();
        let plugin = Box::from_raw(boxed_raw);

        self.loaded_libraries
            .insert(plugin.plugin_id().to_string(), lib);
        Ok(plugin)
    }

    pub fn on_load(&mut self) -> Result<(), &'static str> {
        for plugin in &self.plugins {
            plugin.on_load();
            println!("Loaded plugin: {}", plugin.name());
        }
        Ok(())
    }

    pub unsafe fn register_systems<'b>(&'b mut self) -> Result<(), &'static str> {
        Ok(())
    }
 
    pub fn load_systems(&mut self) {
        for system in &self.systems {
            unsafe {
                system.1(vec![]);
            }
        }
    }
}
