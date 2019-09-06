
pub trait Plugin<'a, 'b> {

    fn get_name(&self) -> String;

    fn get_modid(&self) -> String;

    fn get_version(&self) -> String;

    fn register_systems(&self, db: specs::DispatcherBuilder<'a, 'b>) -> specs::DispatcherBuilder<'a, 'b>;

}

#[repr(C)]
#[derive(Debug)]
pub struct PluginMetadata(String, String, String);

#[repr(C)]
pub struct PluginHandler<'a, 'b> {
    pub mods: Vec<Box<dyn Plugin<'a, 'b>>>,
    pub libs: Vec<libloading::Library>
}

impl<'a, 'b> std::fmt::Debug for PluginHandler<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mods {:?}", self.get_metadatas())
    }
}

impl<'a, 'b> PluginHandler<'a, 'b> {

    pub fn register_library(&mut self, library_path: std::path::PathBuf) -> libloading::Result<()> {
        let lib = libloading::Library::new(library_path)?;
        unsafe {
            let func: libloading::Symbol<extern "C" fn(ph: &mut PluginHandler<'a, 'b>)> = lib.get(b"init_plugin")?;
            func(self);
        }
        println!("{:?}", self);
        self.libs.push(lib);
        Ok(())
    }

    pub fn register_mod(&mut self, plugin: Box<dyn Plugin<'a, 'b>>) {
        self.mods.push(plugin);
    }

    pub fn get_metadatas(&self) -> Vec<PluginMetadata> {
        self.mods.iter().map(|p| {
            PluginMetadata(p.get_name(), p.get_modid(), p.get_version())
        }).collect()
    }

}