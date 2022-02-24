use plugin_core::PluginOption;

#[derive(Debug)]
pub struct ReplacePluginOption<'a> {
    pub from: char,
    pub to: &'a str,
}

impl<'a> PluginOption for ReplacePluginOption<'a> {}

impl<'a> Default for ReplacePluginOption<'a> {
    fn default() -> Self {
        ReplacePluginOption {
            from: '\'',
            to: "\"",
        }
    }
}

#[no_mangle]
pub fn transform(s: String, option: Option<Box<ReplacePluginOption>>) -> String {
    let option = option.unwrap_or_default();
    s.replace(option.from, option.to)
}
