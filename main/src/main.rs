use case::CasePlugin;
use plugin_core::echo;
use replace::ReplacePlugin;
use std::io::{stdin, Result};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    echo(
        buffer,
        vec![
            &ReplacePlugin {
                from: '\'',
                to: "^",
            },
            &CasePlugin::default(),
        ],
    );

    Ok(())
}
