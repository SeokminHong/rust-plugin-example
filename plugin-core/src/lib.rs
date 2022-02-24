use libloading::{library_filename, Library};

pub trait PluginOption {}

pub fn echo(s: &str, plugins: Vec<(&str, Option<Box<dyn PluginOption>>)>) -> () {
    let result = plugins
        .into_iter()
        .fold(String::from(s), |acc, (plugin, option)| {
            let lib = unsafe { Library::new(library_filename(plugin)).unwrap() };
            let func = unsafe {
                lib.get::<fn(String, Option<Box<dyn PluginOption>>) -> String>(b"transform")
                    .unwrap()
            };
            func(acc, option)
        });
    println!("{}", result)
}
