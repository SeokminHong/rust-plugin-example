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
                Some(&ReplacePluginOption {
                    from: '\'',
                    to: "^",
                }),
            ),
            ("case", None),
        ],
    );

    Ok(())
}
