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
    // Write a green underlined text on a yellow background !
    println!("{}", "Hello World !".greenf().underline().yellowb());
}

```