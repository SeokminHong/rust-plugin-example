use plugin_core::echo;
use replace::ReplacePluginOption;

fn main() {
    echo(
        "'Hello, world!'",
        vec![(
            "replace",
            Some(Box::new(ReplacePluginOption {
                from: '\'',
                to: "^",
            })),
        )],
    );
}
