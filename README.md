colorize
========

__libcolorize__ provide simple text colorization for terminal emulator, using ansi escape characters.

To build __libcolorize__ just do :

```Shell
> rustc lib.rs
```

__libcolorize__ is really simple to use, see this short example !

```Rust
extern mod colorize;
use colorize::AnsiColor;

pub fn main() {
    // Set some global colors
    colorize::global_fg(BrightRed);
    colorize::global_bg(Blue);
    // ^~~~ These settings are reset to default at the end.

    // You can use specific colors or style on a given str,
    // the globals colors are restored after !

    // Write a green underlined text on a yellow background !
    println!("{}", "Hello World !".greenf().underline().yellowb());

    // Use bright or normal colors
    println!("{}", "Bright Green foreground and Magenta background !".b_greenf().magentab());
}

```
