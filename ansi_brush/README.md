# ansi_brush

Ultra lightweight ANSI styling

## Examples

```rust
use ansi_brush::Style;

fn main()  {
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
}
```

### TODO

- [x] 8 colors
- [x] 16 colors
- [x] Macro
- [ ] 256 colors
- [x] Backgrounds
- [x] Styling
- [ ] RGB colors
