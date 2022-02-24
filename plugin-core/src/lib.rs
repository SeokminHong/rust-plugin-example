use libloading::{library_filename, Library};

pub trait PluginOption {}

pub fn echo<'a, S: Into<String>>(s: S, plugins: Vec<(&str, &'a dyn PluginOption)>) -> () {
    let result = plugins.into_iter().fold(s.into(), |acc, (plugin, option)| {
        let lib = unsafe { Library::new(library_filename(plugin)).unwrap() };
        let func = unsafe {
            lib.get::<fn(String, &'a dyn PluginOption) -> String>(b"transform")
                .unwrap()
        };
        func(acc, option)
    });
    println!("{}", result)
}
