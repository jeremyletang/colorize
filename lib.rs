// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/*!
Terminal color using ansi escape character for Rust.

```Rust
extern mod colorize;
// Import the trait implemented for &'static str and ~str
use colorize::AnsiColor;
// Import the colors for the global
use colorize::{BrightRed, Blue};

pub fn main() {
    // Set some global colors
    colorize::global_fg(BrightRed);
    colorize::global_bg(Blue);
    // ^~~~ These settings are reset to default at the end.

    // You can use specific colors or style on a given str,
    // the globals colors are restored after !

    // Write a green underlined text on a yellow background !
    println!("{}", "Hello World !".green().underline().yellowb());

    // Use bright or normal colors
    println!("{}", "Bright Green foreground and Magenta background !"
                   .b_green()
                   .magentab());
}

```
*/

#![crate_id = "colorize#0.1"]
#![desc = "Terminal color library"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

use std::cast;

/// Ansi color to set the global foreground / background color
pub enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    Grey = 37,
    Default = 39,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightGrey = 97
}

enum BgColor {
    Blackb = 40,
    Redb = 41,
    Greenb = 42,
    Yellowb = 43,
    Blueb = 44,
    Magentab = 45,
    Cyanb = 46,
    Greyb = 47,
    Defaultb = 49,
    BrightBlackb = 100,
    BrightRedb = 101,
    BrightGreenb = 102,
    BrightYellowb = 103,
    BrightBlueb = 104,
    BrightMagentab = 105,
    BrightCyanb = 106,
    BrightGreyb = 107
}

enum Style {
    Underscore = 4,
    Bold = 1,
    Blink = 5,
    Reverse = 7,
    Concealed = 8,
    Faint = 2,
    Italic = 3,
    CrossedOut = 9
}

impl internal::TermAttrib for Color {
    fn to_int(&self) -> int { *self as int }
}

impl internal::TermAttrib for BgColor {
    fn to_int(&self) -> int { *self as int }
}

impl internal::TermAttrib for Style {
    fn to_int(&self) -> int { *self as int }
}

impl BgColor {
    fn from_fg(color: Color) -> BgColor {
        unsafe { cast::transmute(color as i8 + 10) }
    }
}

mod internal {
    use std::local_data;
    use super::{Color, BgColor};

    static DEFAULT_FG: int = 39;
    static DEFAULT_BG: int = 49;

    static glob_color: local_data::Key<GlobalColor> = &local_data::Key;

    pub trait TermAttrib {
        fn to_int(&self) -> int;
    }

    pub struct GlobalColor {
        fg: int,
        bg: int
    }

    impl Drop for GlobalColor {
        fn drop(&mut self) {
            print!("\x1b[0;{};{}m", DEFAULT_FG, DEFAULT_BG)
        }
    }

    fn get_glob() -> (int, int) {
        local_data::get(glob_color, |g| {
                match g {
                    Some(ref g)    => (g.fg, g.bg),
                    None        => {
                        local_data::set(glob_color, GlobalColor {
                                fg: DEFAULT_FG, bg: DEFAULT_BG
                            });
                        (DEFAULT_FG, DEFAULT_BG)
                    }
                }
            })
    }

    pub fn global_color(fg_color: Option<Color>, bg_color: Option<BgColor>) {
        local_data::get_mut(glob_color, |g| {
                match g {
                    Some(g) => {
                        match fg_color {
                            Some(c) => g.fg = c.to_int(),
                            None    => g.fg = DEFAULT_FG
                        }
                        match bg_color {
                            Some(c) => g.bg = c.to_int(),
                            None    => g.bg = DEFAULT_BG
                        }
                    },
                    None    => {
                        local_data::set(glob_color, GlobalColor {
                                fg: if fg_color.is_some() {
                                    fg_color.unwrap().to_int()
                                } else {
                                    DEFAULT_FG
                                },
                                bg: if bg_color.is_some() {
                                    bg_color.unwrap().to_int()
                                } else {
                                    DEFAULT_BG
                                }
                            });
                    }
                }
            })
    }

    pub fn pack<T: TermAttrib>(attrib: T, mut text: ~str) -> ~str {
        if text.starts_with("\x1b[") {
            text.insert(2, format!("{};", attrib.to_int()));
        } else {
            text.insert(0, format!("\x1b[{}m", attrib.to_int()));
            let (fg, bg) = get_glob();
            text.push_str(format!("\x1b[0;{};{}m", fg, bg));
        }
        text
    }
}

/// Set a custom global foreground color
pub fn global_fg(color: Color) {
    internal::global_color(Some(color), None)
}

/// Set a custom global background color
pub fn global_bg(color: Color) {
    internal::global_color(None, Some(BgColor::from_fg(color)))
}

/// Reset the background and foreground color to the defaults colors
pub fn reset() {
    internal::global_color(Some(Default), Some(Defaultb))
}

/// Methods extension to colorize the text contained in a string
/// using a simple mathod call
pub trait AnsiColor {
    /// Foreground black
    fn black(self) -> ~str;
    /// Foreground red
    fn red(self) -> ~str;
    /// Foreground green
    fn green(self) -> ~str;
    /// Foreground yellow
    fn yellow(self) -> ~str;
    /// Foreground blue
    fn blue(self) -> ~str;
    /// Foreground magenta
    fn magenta(self) -> ~str;
    /// Foreground cyan
    fn cyan(self) -> ~str;
    /// Foreground grey
    fn grey(self) -> ~str;
    /// Foreground black bright
    fn b_black(self) -> ~str;
    /// Foreground red bright
    fn b_red(self) -> ~str;
    /// Foreground green bright
    fn b_green(self) -> ~str;
    /// Foreground yellow bright
    fn b_yellow(self) -> ~str;
    /// Foreground blue bright
    fn b_blue(self) -> ~str;
    /// Foreground magenta bright
    fn b_magenta(self) -> ~str;
    /// Foreground cyan bright
    fn b_cyan(self) -> ~str;
    /// Foreground grey bright
    fn b_grey(self) -> ~str;
    /// Foreground default
    fn default(self) -> ~str;

    /// Background black
    fn blackb(self) -> ~str;
    /// Background red
    fn redb(self) -> ~str;
    /// Background green
    fn greenb(self) -> ~str;
    /// Background yellow
    fn yellowb(self) -> ~str;
    /// Background bblue
    fn blueb(self) -> ~str;
    /// Background magenta
    fn magentab(self) -> ~str;
    /// Background cyan
    fn cyanb(self) -> ~str;
    /// Background grey
    fn greyb(self) -> ~str;
    /// Background black bright
    fn b_blackb(self) -> ~str;
    /// Background red bright
    fn b_redb(self) -> ~str;
    /// Background green bright
    fn b_greenb(self) -> ~str;
    /// Background yellow bright
    fn b_yellowb(self) -> ~str;
    /// Background bblue bright
    fn b_blueb(self) -> ~str;
    /// Background magenta bright
    fn b_magentab(self) -> ~str;
    /// Background cyan bright
    fn b_cyanb(self) -> ~str;
    /// Background grey bright
    fn b_greyb(self) -> ~str;
    /// Background default
    fn defaultb(self) -> ~str;

    /// Text underlined
    fn underscore(self) -> ~str;
    /// Bold text
    fn bold(self) -> ~str;
    /// Blink test ( Wonderful )
    fn blink(self) -> ~str;
    /// Reverse mod ON
    fn reverse(self) -> ~str;
    /// Concealed mod ON
    fn concealed(self) -> ~str;
    /// Faint mod ON
    fn faint(self) -> ~str;
    /// Italic text
    fn italic(self) -> ~str;
    /// Crossed out
    fn crossedout(self) -> ~str;
}

impl AnsiColor for ~str {
    // Foreground
    fn black(self) -> ~str { internal::pack(Black, self) }
    fn red(self) -> ~str { internal::pack(Red, self) }
    fn green(self) -> ~str { internal::pack(Green, self) }
    fn yellow(self) -> ~str { internal::pack(Yellow, self) }
    fn blue(self) -> ~str { internal::pack(Blue, self) }
    fn magenta(self) -> ~str { internal::pack(Magenta, self) }
    fn cyan(self) -> ~str { internal::pack(Cyan, self) }
    fn grey(self) -> ~str { internal::pack(Grey, self) }
    fn b_black(self) -> ~str { internal::pack(BrightBlack, self) }
    fn b_red(self) -> ~str { internal::pack(BrightRed, self) }
    fn b_green(self) -> ~str { internal::pack(BrightGreen, self) }
    fn b_yellow(self) -> ~str { internal::pack(BrightYellow, self) }
    fn b_blue(self) -> ~str { internal::pack(BrightBlue, self) }
    fn b_magenta(self) -> ~str { internal::pack(BrightMagenta, self) }
    fn b_cyan(self) -> ~str { internal::pack(BrightCyan, self) }
    fn b_grey(self) -> ~str { internal::pack(BrightGrey, self) }
    fn default(self) -> ~str { internal::pack(Default, self) }

    // Background
    fn blackb(self) -> ~str { internal::pack(Blackb, self) }
    fn redb(self) -> ~str { internal::pack(Redb, self) }
    fn greenb(self) -> ~str { internal::pack(Greenb, self) }
    fn yellowb(self) -> ~str { internal::pack(Yellowb, self) }
    fn blueb(self) -> ~str { internal::pack(Blueb, self) }
    fn magentab(self) -> ~str { internal::pack(Magentab, self) }
    fn cyanb(self) -> ~str { internal::pack(Cyanb, self) }
    fn greyb(self) -> ~str { internal::pack(Greyb, self) }
    fn b_blackb(self) -> ~str { internal::pack(BrightBlackb, self) }
    fn b_redb(self) -> ~str { internal::pack(BrightRedb, self) }
    fn b_greenb(self) -> ~str { internal::pack(BrightGreenb, self) }
    fn b_yellowb(self) -> ~str { internal::pack(BrightYellowb, self) }
    fn b_blueb(self) -> ~str { internal::pack(BrightBlueb, self) }
    fn b_magentab(self) -> ~str { internal::pack(BrightMagentab, self) }
    fn b_cyanb(self) -> ~str { internal::pack(BrightCyanb, self) }
    fn b_greyb(self) -> ~str { internal::pack(BrightGreyb, self) }
    fn defaultb(self) -> ~str { internal::pack(Defaultb, self) }

    // styles
    fn underscore(self) -> ~str { internal::pack(Underscore, self) }
    fn bold(self) -> ~str { internal::pack(Bold, self) }
    fn blink(self) -> ~str { internal::pack(Blink, self) }
    fn reverse(self) -> ~str { internal::pack(Reverse, self) }
    fn concealed(self) -> ~str { internal::pack(Concealed, self) }
    fn faint(self) -> ~str { internal::pack(Faint, self) }
    fn italic(self) -> ~str { internal::pack(Italic, self) }
    fn crossedout(self) -> ~str { internal::pack(CrossedOut, self) }
}


impl AnsiColor for &'static str {
    // Foreground
    fn black(self) -> ~str { self.into_owned().black() }
    fn red(self) -> ~str { self.into_owned().red() }
    fn green(self) -> ~str { self.into_owned().green() }
    fn yellow(self) -> ~str { self.into_owned().yellow() }
    fn blue(self) -> ~str { self.into_owned().blue() }
    fn magenta(self) -> ~str { self.into_owned().magenta() }
    fn cyan(self) -> ~str { self.into_owned().cyan() }
    fn grey(self) -> ~str { self.into_owned().grey() }
    fn default(self) -> ~str { self.into_owned().default() }
    fn b_black(self) -> ~str { self.into_owned().b_black() }
    fn b_red(self) -> ~str { self.into_owned().b_red() }
    fn b_green(self) -> ~str { self.into_owned().b_green() }
    fn b_yellow(self) -> ~str { self.into_owned().b_yellow() }
    fn b_blue(self) -> ~str { self.into_owned().b_blue() }
    fn b_magenta(self) -> ~str { self.into_owned().b_magenta() }
    fn b_cyan(self) -> ~str { self.into_owned().b_cyan() }
    fn b_grey(self) -> ~str { self.into_owned().b_grey() }

    // Background
    fn blackb(self) -> ~str { self.into_owned().blackb() }
    fn redb(self) -> ~str { self.into_owned().redb() }
    fn greenb(self) -> ~str { self.into_owned().greenb() }
    fn yellowb(self) -> ~str { self.into_owned().yellowb() }
    fn blueb(self) -> ~str { self.into_owned().blueb() }
    fn magentab(self) -> ~str { self.into_owned().magentab() }
    fn cyanb(self) -> ~str { self.into_owned().cyanb() }
    fn greyb(self) -> ~str { self.into_owned().greyb() }
    fn defaultb(self) -> ~str { self.into_owned().defaultb() }
    fn b_blackb(self) -> ~str { self.into_owned().b_blackb() }
    fn b_redb(self) -> ~str { self.into_owned().b_redb() }
    fn b_greenb(self) -> ~str { self.into_owned().b_greenb() }
    fn b_yellowb(self) -> ~str { self.into_owned().b_yellowb() }
    fn b_blueb(self) -> ~str { self.into_owned().b_blueb() }
    fn b_magentab(self) -> ~str { self.into_owned().b_magentab() }
    fn b_cyanb(self) -> ~str { self.into_owned().b_cyanb() }
    fn b_greyb(self) -> ~str { self.into_owned().b_greyb() }

    // styles
    fn underscore(self) -> ~str { self.into_owned().underscore() }
    fn bold(self) -> ~str { self.into_owned().bold() }
    fn blink(self) -> ~str { self.into_owned().blink() }
    fn reverse(self) -> ~str { self.into_owned().reverse() }
    fn concealed(self) -> ~str { self.into_owned().concealed() }
    fn faint(self) -> ~str { self.into_owned().faint() }
    fn italic(self) -> ~str { self.into_owned().italic() }
    fn crossedout(self) -> ~str { self.into_owned().crossedout() }
}
