use ansi_brush::Style;

fn main() {
    println!("{} {}", "Hello,".light_cyan().bold(), "World!".light_red().slow_blink().conclude())
}
