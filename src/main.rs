mod config;

use anyhow::{Ok, Result, Context as _};
use clap::Parser;
use config::{read_config, Kaomoji};
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about)]
struct Echo {
    strings: Vec<String>,

    #[arg(short, long)]
    newline: bool,

    #[arg(short, long)]
    escape: bool,
}

fn main() -> Result<()> {
    let echo = Echo::parse();
    let config = read_config()?;

    let Kaomoji {
        name: _,
        kaomoji,
        speech_bubble_left,
        speech_bubble_right,
    } = config
        .presets
        .iter()
        .find(|x| x.name == config.default)
        .with_context(|| "Failed to find kaomoji")?;
    let speech_bubble_left =  speech_bubble_left.clone().unwrap_or("".to_string());
    let speech_bubble_right = speech_bubble_right.clone().unwrap_or("".to_string());

    if echo.newline {
        for s in echo.strings {
            println!("{}{}{}{}", kaomoji, speech_bubble_left, s, speech_bubble_right);
        }
    } else {
        print!("{}{}{}{}", kaomoji, speech_bubble_left, echo.strings.iter().join(" "), speech_bubble_right);
    }

    Ok(())
}
