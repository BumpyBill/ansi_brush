# ansi_brush

> ANSI Styling for Rust

## Examples

```rs
use ansi_brush::Style;

fn main()  {
    println!("{} {}", "Hello,".light_cyan(), "World!".reset());
    // use conclude to stop background bleeding into next lines (try it without conclude!)
    println!("{}", "Goodbye, Mars!".red().conclude());
}
```

### TODO

- [x] 8 colors
- [x] 16 colors
- [x] Macro
- [x] 256 colors
- [x] Backgrounds
- [ ] Styling
  - [ ] Bold
  - [ ] Faint
  - [ ] Italic
  - [ ] Underline
  - [ ] Slow blink
  - [ ] Rapid Blink
