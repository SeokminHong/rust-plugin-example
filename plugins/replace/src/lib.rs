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
pub fn transform<'a, 'b>(s: String, option: Option<&'a ReplacePluginOption<'b>>) -> String {
    let ReplacePluginOption { from, to } = *option.unwrap_or(&ReplacePluginOption::default());
    s.replace(from, to)
}
