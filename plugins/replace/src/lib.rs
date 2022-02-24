use plugin_core::Plugin;

#[derive(Debug)]
pub struct ReplacePlugin<'a> {
    pub from: char,
    pub to: &'a str,
}

impl<'a> Plugin for ReplacePlugin<'a> {
    fn name(&self) -> &str {
        option_env!("CARGO_CRATE_NAME").expect("Failed to get crate name")
    }
}

impl<'a> Default for ReplacePlugin<'a> {
    fn default() -> Self {
        ReplacePlugin {
            from: '\'',
            to: "\"",
        }
    }
}

#[no_mangle]
pub fn transform(s: String, option: &'_ ReplacePlugin<'_>) -> String {
    let ReplacePlugin { from, to } = *option;
    s.replace(from, to)
}
