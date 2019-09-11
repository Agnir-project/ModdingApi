mod plugins;
mod structs;

pub use libloading::Library;
pub use plugins::*;
pub use structs::*;

#[macro_export]
macro_rules! make_plugin {
    ($plugin_type:ty) => {
        impl From<$crate::Library> for $plugin_type {
            fn from(library: $crate::Library) -> Self {
                let mut plugin = Self::new();
                plugin.library = Some(library);
                plugin
            }
        }

        #[no_mangle]
        pub extern "C" fn _plugin_create(lib: Library) -> *mut $crate::Plugin {
            let constructor: fn(Library) -> $plugin_type = <$plugin_type>::from;

            let object = constructor(lib);
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
