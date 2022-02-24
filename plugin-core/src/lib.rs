use libloading::{library_filename, Library};

pub trait PluginOption {}

pub fn echo<S: Into<String>>(s: S, plugins: Vec<(&str, Option<&'_ dyn PluginOption>)>) -> () {
    let result = plugins.into_iter().fold(s.into(), |acc, (plugin, option)| {
        let lib = unsafe { Library::new(library_filename(plugin)).unwrap() };
        let func = unsafe {
            lib.get::<fn(String, Option<&'_ dyn PluginOption>) -> String>(b"transform")
                .unwrap()
        };
        func(acc, option)
    });
    println!("{}", result)
}
