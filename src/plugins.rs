use crate::structs::Component;
use libloading::{Library, Symbol};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::path::PathBuf;

pub type PluginCreate<'a> = unsafe fn(Library) -> *mut dyn Plugin;
pub type ComponentsHashMap = Vec<HashMap<String, Box<dyn Component>>>;
pub type SystemSymbol = unsafe fn(data: ComponentsHashMap);
pub type LoadedPlugin = Box<dyn Plugin>;

#[cfg(windows)]
fn is_lib(path: PathBuf) -> bool {
    match path.extension() {
        Some(extension) => extension == "dll",
        _ => false,
    }
}

#[cfg(unix)]
fn is_lib(path: PathBuf) -> bool {
    match path.extension() {
        Some(extension) => extension == "so",
        _ => false,
    }
}

pub trait Plugin: Any + Send + Sync + Debug {

    fn new() -> Self where Self: Sized;

    fn name(&self) -> &'static str;

    fn plugin_id(&self) -> &'static str;

    fn version(&self) -> &'static str;

    fn on_load(&self);

    fn systems(&self) -> Vec<&'static str>;

    fn take_library(&mut self) -> Library;
}

pub struct PluginManager;

impl PluginManager {
    pub unsafe fn load_plugin_folder(
        folder_path: PathBuf,
    ) -> Result<Vec<LoadedPlugin>, &'static str> {
        let plugins = read_dir(folder_path)
            .map_err(|e| "Cannot scan directory for plugins.")?
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
            .filter(|e| is_lib(e.path()))
            .filter_map(|e| Self::load_plugin(e.path()).ok())
            .collect::<Vec<LoadedPlugin>>();
        Ok(plugins)
    }

    pub unsafe fn load_plugin(filename: PathBuf) -> Result<LoadedPlugin, &'static str> {
        let lib = Library::new(filename).map_err(|_| "Unable to load the plugin")?;
        let constructor: Symbol<PluginCreate> = lib
            .get(b"_plugin_create")
            .map_err(|_| "The `_plugin_create` symbol wasn't found.")?;
        let boxed_raw = constructor(lib);
        let plugin = Box::from_raw(boxed_raw);

        Ok(plugin)
    }

    pub fn load(plugins: &Vec<LoadedPlugin>) -> Result<(), &'static str> {
        for plugin in plugins {
            plugin.on_load();
            println!("Loaded plugin: {}", plugin.name());
        }
        Ok(())
    }

    pub fn unload_plugins(mut plugins: Vec<LoadedPlugin>) -> Vec<Library> {
        plugins.iter_mut().map(|p| p.take_library()).collect()
    }
}
