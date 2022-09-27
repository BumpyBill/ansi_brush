use ansi_brush::Style;
use parser::Lexer;
use serde::Deserialize;

mod parser;

#[derive(Deserialize)]
struct Config {
    format: String,
}

fn main() -> anyhow::Result<()> {
    let config_source = std::fs::read_to_string("./test.toml").expect("Unable to read config");
    let config: Config = toml::from_str(&config_source)?;

    let lexer = Lexer::new(&config.format);
    for token in lexer {
        println!("{:?}", token);
    }

    println!("{} {}", "Hello,".light_cyan().bold(), "World!".reset());
    // always put "reset" LAST after each style to revert the previous line's styles
    println!(
        "{} {} {} {} {} {} {} {}",
        "Bold".bold(),
        "Faint".faint().reset(),
        "Italic".italic().reset(),
        "Underline".underline().reset(),
        "Slow Blink".slow_blink().reset(),
        "Rapid Blink".rapid_blink().reset(),
        "Reverse".reverse().reset(),
        "Strike".strike().reset()
    );
    // use conclude to stop background bleeding into next lines (try it without conclude!)
    println!("{}", "Goodbye, Mars!".bg_red().underline().italic().conclude().reset());

    Ok(())
}
