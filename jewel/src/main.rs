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

    println!("{} {}", "Hello,".light_cyan(), "World!".reset());
    println!("{}", "Goodbye, Mars!".red().conclude());

    Ok(())
}
