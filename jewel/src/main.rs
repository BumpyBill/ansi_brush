use parser::Lexer;
use serde::Deserialize;
use ansi_brush::Style;

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

    println!("{}", "Goodbye, Mars!".black());
    println!("{}", "Hello, World!".light_black());
    println!("{}", "Goodbye, Mars!".red());
    println!("{}", "Hello, World!".light_red());
    println!("{}", "Goodbye, Mars!".green());
    println!("{}", "Hello, World!".light_green());
    println!("{}", "Goodbye, Mars!".blue());
    println!("{}", "Hello, World!".light_blue());
    println!("{}", "Goodbye, Mars!".magenta());
    println!("{}", "Hello, World!".light_magenta());
    println!("{}", "Goodbye, Mars!".cyan());
    println!("{}", "Hello, World!".light_cyan());
    println!("{}", "Goodbye, Mars!".white());
    println!("{}", "Hello, World!".light_white());
    println!(
        "{} {}",
        "Hello,".red(),
        "World!".reset()
    );

    Ok(())
}
