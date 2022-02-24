use libloading::{library_filename, Library};

pub trait Plugin {
    fn name(&self) -> &str;
}

pub fn echo<S: Into<String>>(s: S, plugins: Vec<&'_ dyn Plugin>) -> () {
    let result = plugins.into_iter().fold(s.into(), |acc, plugin| {
        let lib = unsafe { Library::new(library_filename(plugin.name())).unwrap() };
        let func = unsafe {
            lib.get::<fn(String, &'_ dyn Plugin) -> String>(b"transform")
                .unwrap()
        };
        func(acc, plugin)
    });
    println!("{}", result)
}
