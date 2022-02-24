use libloading::{library_filename, Library};

pub trait PluginOption {}

pub fn echo<'a>(s: &str, plugins: Vec<(&str, fn() -> &'a dyn PluginOption)>) -> () {
    let result = plugins
        .into_iter()
        .fold(String::from(s), |acc, (plugin, option)| {
            let lib = unsafe { Library::new(library_filename(plugin)).unwrap() };
            let func = unsafe {
                lib.get::<fn(String, fn() -> &'a dyn PluginOption) -> String>(b"transform")
                    .unwrap()
            };
            func(acc, option)
        });
    println!("{}", result)
}
