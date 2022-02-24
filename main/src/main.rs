use case::{Case, CasePluginOption};
use plugin_core::echo;
use replace::ReplacePluginOption;
use std::io::{stdin, Result};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    echo(
        buffer,
        vec![
            (
                "replace",
                &ReplacePluginOption {
                    from: '\'',
                    to: "^",
                },
            ),
            ("case", &CasePluginOption { case: Case::Toggle }),
        ],
    );

    Ok(())
}
