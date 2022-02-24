use case::{Case, CasePluginOption};
use plugin_core::echo;
use replace::ReplacePluginOption;

fn main() {
    echo(
        "'Hello, world!'",
        vec![
            ("replace", || &ReplacePluginOption {
                from: '\'',
                to: "^",
            }),
            ("case", || &CasePluginOption { case: Case::Toggle }),
        ],
    );
}
