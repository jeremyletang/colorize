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

use std::mem;

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
        unsafe { mem::transmute(color as i8 + 10) }
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

    #[deriving(Clone)]
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
        match glob_color.get() {
            Some(g)    => (g.fg, g.bg),
            None        => {
                glob_color.replace(Some(GlobalColor {
                        fg: DEFAULT_FG, bg: DEFAULT_BG
                }));
                (DEFAULT_FG, DEFAULT_BG)
            }
        }
    }

    pub fn global_color(fg_color: Option<Color>, bg_color: Option<BgColor>) {
        let is_some = glob_color.get().is_some();

        if is_some {
            let mut g = (*glob_color).get().unwrap().clone();
            match fg_color {
                Some(c) => g.fg = c.to_int(),
                None    => g.fg = DEFAULT_FG
            }
            match bg_color {
                Some(c) => g.bg = c.to_int(),
                None    => g.bg = DEFAULT_BG
            }
            glob_color.replace(Some(g));
        } else {
            glob_color.replace(Some(GlobalColor {
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
                }));
        }
    }

    pub fn pack<T: TermAttrib>(attrib: T, mut text: StrBuf) -> StrBuf {
        if text.as_slice().starts_with("\x1b[") {
            unsafe {
                text.shift_byte();
                text.shift_byte();
            }
            let tmp = text;
            text = StrBuf::from_str("\x1b[");
            text.push_str(format!("{};", attrib.to_int()));
            text.push_str(tmp.as_slice());
        } else {
            let tmp = text;
            text = StrBuf::from_owned_str(format!("\x1b[{}m", attrib.to_int()));
            text.push_str(tmp.as_slice());
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
    fn black(self) -> StrBuf;
    /// Foreground red
    fn red(self) -> StrBuf;
    /// Foreground green
    fn green(self) -> StrBuf;
    /// Foreground yellow
    fn yellow(self) -> StrBuf;
    /// Foreground blue
    fn blue(self) -> StrBuf;
    /// Foreground magenta
    fn magenta(self) -> StrBuf;
    /// Foreground cyan
    fn cyan(self) -> StrBuf;
    /// Foreground grey
    fn grey(self) -> StrBuf;
    /// Foreground black bright
    fn b_black(self) -> StrBuf;
    /// Foreground red bright
    fn b_red(self) -> StrBuf;
    /// Foreground green bright
    fn b_green(self) -> StrBuf;
    /// Foreground yellow bright
    fn b_yellow(self) -> StrBuf;
    /// Foreground blue bright
    fn b_blue(self) -> StrBuf;
    /// Foreground magenta bright
    fn b_magenta(self) -> StrBuf;
    /// Foreground cyan bright
    fn b_cyan(self) -> StrBuf;
    /// Foreground grey bright
    fn b_grey(self) -> StrBuf;
    /// Foreground default
    fn default(self) -> StrBuf;

    /// Background black
    fn blackb(self) -> StrBuf;
    /// Background red
    fn redb(self) -> StrBuf;
    /// Background green
    fn greenb(self) -> StrBuf;
    /// Background yellow
    fn yellowb(self) -> StrBuf;
    /// Background bblue
    fn blueb(self) -> StrBuf;
    /// Background magenta
    fn magentab(self) -> StrBuf;
    /// Background cyan
    fn cyanb(self) -> StrBuf;
    /// Background grey
    fn greyb(self) -> StrBuf;
    /// Background black bright
    fn b_blackb(self) -> StrBuf;
    /// Background red bright
    fn b_redb(self) -> StrBuf;
    /// Background green bright
    fn b_greenb(self) -> StrBuf;
    /// Background yellow bright
    fn b_yellowb(self) -> StrBuf;
    /// Background bblue bright
    fn b_blueb(self) -> StrBuf;
    /// Background magenta bright
    fn b_magentab(self) -> StrBuf;
    /// Background cyan bright
    fn b_cyanb(self) -> StrBuf;
    /// Background grey bright
    fn b_greyb(self) -> StrBuf;
    /// Background default
    fn defaultb(self) -> StrBuf;

    /// Text underlined
    fn underscore(self) -> StrBuf;
    /// Bold text
    fn bold(self) -> StrBuf;
    /// Blink test ( Wonderful )
    fn blink(self) -> StrBuf;
    /// Reverse mod ON
    fn reverse(self) -> StrBuf;
    /// Concealed mod ON
    fn concealed(self) -> StrBuf;
    /// Faint mod ON
    fn faint(self) -> StrBuf;
    /// Italic text
    fn italic(self) -> StrBuf;
    /// Crossed out
    fn crossedout(self) -> StrBuf;
}

impl AnsiColor for ~str {
    // Foreground
    fn black(self) -> StrBuf { self.into_strbuf().black() }
    fn red(self) -> StrBuf { self.into_strbuf().red() }
    fn green(self) -> StrBuf { self.into_strbuf().green() }
    fn yellow(self) -> StrBuf { self.into_strbuf().yellow() }
    fn blue(self) -> StrBuf { self.into_strbuf().blue() }
    fn magenta(self) -> StrBuf { self.into_strbuf().magenta() }
    fn cyan(self) -> StrBuf { self.into_strbuf().cyan() }
    fn grey(self) -> StrBuf { self.into_strbuf().grey() }
    fn b_black(self) -> StrBuf { self.into_strbuf().b_black() }
    fn b_red(self) -> StrBuf { self.into_strbuf().b_red() }
    fn b_green(self) -> StrBuf { self.into_strbuf().b_green() }
    fn b_yellow(self) -> StrBuf { self.into_strbuf().b_yellow() }
    fn b_blue(self) -> StrBuf { self.into_strbuf().b_blue() }
    fn b_magenta(self) -> StrBuf { self.into_strbuf().b_magenta() }
    fn b_cyan(self) -> StrBuf { self.into_strbuf().b_cyan() }
    fn b_grey(self) -> StrBuf { self.into_strbuf().b_grey() }
    fn default(self) -> StrBuf { self.into_strbuf().default() }

    // Background
    fn blackb(self) -> StrBuf { self.into_strbuf().blackb() }
    fn redb(self) -> StrBuf { self.into_strbuf().redb() }
    fn greenb(self) -> StrBuf { self.into_strbuf().greenb() }
    fn yellowb(self) -> StrBuf { self.into_strbuf().yellowb() }
    fn blueb(self) -> StrBuf { self.into_strbuf().blueb() }
    fn magentab(self) -> StrBuf { self.into_strbuf().magentab() }
    fn cyanb(self) -> StrBuf { self.into_strbuf().cyanb() }
    fn greyb(self) -> StrBuf { self.into_strbuf().greyb() }
    fn b_blackb(self) -> StrBuf { self.into_strbuf().b_blackb() }
    fn b_redb(self) -> StrBuf { self.into_strbuf().b_redb() }
    fn b_greenb(self) -> StrBuf { self.into_strbuf().b_greenb() }
    fn b_yellowb(self) -> StrBuf { self.into_strbuf().b_yellowb() }
    fn b_blueb(self) -> StrBuf { self.into_strbuf().b_blueb() }
    fn b_magentab(self) -> StrBuf { self.into_strbuf().b_magentab() }
    fn b_cyanb(self) -> StrBuf { self.into_strbuf().b_cyanb() }
    fn b_greyb(self) -> StrBuf { self.into_strbuf().b_greyb() }
    fn defaultb(self) -> StrBuf { self.into_strbuf().defaultb() }

    // styles
    fn underscore(self) -> StrBuf { self.into_strbuf().underscore() }
    fn bold(self) -> StrBuf { self.into_strbuf().bold() }
    fn blink(self) -> StrBuf { self.into_strbuf().blink() }
    fn reverse(self) -> StrBuf { self.into_strbuf().reverse() }
    fn concealed(self) -> StrBuf { self.into_strbuf().concealed() }
    fn faint(self) -> StrBuf { self.into_strbuf().faint() }
    fn italic(self) -> StrBuf { self.into_strbuf().italic() }
    fn crossedout(self) -> StrBuf { self.into_strbuf().crossedout() }
}

impl AnsiColor for StrBuf {
    // Foreground
    fn black(self) -> StrBuf { internal::pack(Black, self) }
    fn red(self) -> StrBuf { internal::pack(Red, self) }
    fn green(self) -> StrBuf { internal::pack(Green, self) }
    fn yellow(self) -> StrBuf { internal::pack(Yellow, self) }
    fn blue(self) -> StrBuf { internal::pack(Blue, self) }
    fn magenta(self) -> StrBuf { internal::pack(Magenta, self) }
    fn cyan(self) -> StrBuf { internal::pack(Cyan, self) }
    fn grey(self) -> StrBuf { internal::pack(Grey, self) }
    fn b_black(self) -> StrBuf { internal::pack(BrightBlack, self) }
    fn b_red(self) -> StrBuf { internal::pack(BrightRed, self) }
    fn b_green(self) -> StrBuf { internal::pack(BrightGreen, self) }
    fn b_yellow(self) -> StrBuf { internal::pack(BrightYellow, self) }
    fn b_blue(self) -> StrBuf { internal::pack(BrightBlue, self) }
    fn b_magenta(self) -> StrBuf { internal::pack(BrightMagenta, self) }
    fn b_cyan(self) -> StrBuf { internal::pack(BrightCyan, self) }
    fn b_grey(self) -> StrBuf { internal::pack(BrightGrey, self) }
    fn default(self) -> StrBuf { internal::pack(Default, self) }

    // Background
    fn blackb(self) -> StrBuf { internal::pack(Blackb, self) }
    fn redb(self) -> StrBuf { internal::pack(Redb, self) }
    fn greenb(self) -> StrBuf { internal::pack(Greenb, self) }
    fn yellowb(self) -> StrBuf { internal::pack(Yellowb, self) }
    fn blueb(self) -> StrBuf { internal::pack(Blueb, self) }
    fn magentab(self) -> StrBuf { internal::pack(Magentab, self) }
    fn cyanb(self) -> StrBuf { internal::pack(Cyanb, self) }
    fn greyb(self) -> StrBuf { internal::pack(Greyb, self) }
    fn b_blackb(self) -> StrBuf { internal::pack(BrightBlackb, self) }
    fn b_redb(self) -> StrBuf { internal::pack(BrightRedb, self) }
    fn b_greenb(self) -> StrBuf { internal::pack(BrightGreenb, self) }
    fn b_yellowb(self) -> StrBuf { internal::pack(BrightYellowb, self) }
    fn b_blueb(self) -> StrBuf { internal::pack(BrightBlueb, self) }
    fn b_magentab(self) -> StrBuf { internal::pack(BrightMagentab, self) }
    fn b_cyanb(self) -> StrBuf { internal::pack(BrightCyanb, self) }
    fn b_greyb(self) -> StrBuf { internal::pack(BrightGreyb, self) }
    fn defaultb(self) -> StrBuf { internal::pack(Defaultb, self) }

    // styles
    fn underscore(self) -> StrBuf { internal::pack(Underscore, self) }
    fn bold(self) -> StrBuf { internal::pack(Bold, self) }
    fn blink(self) -> StrBuf { internal::pack(Blink, self) }
    fn reverse(self) -> StrBuf { internal::pack(Reverse, self) }
    fn concealed(self) -> StrBuf { internal::pack(Concealed, self) }
    fn faint(self) -> StrBuf { internal::pack(Faint, self) }
    fn italic(self) -> StrBuf { internal::pack(Italic, self) }
    fn crossedout(self) -> StrBuf { internal::pack(CrossedOut, self) }
}

impl AnsiColor for &'static str {
    // Foreground
    fn black(self) -> StrBuf { StrBuf::from_str(self).black() }
    fn red(self) -> StrBuf { StrBuf::from_str(self).red() }
    fn green(self) -> StrBuf { StrBuf::from_str(self).green() }
    fn yellow(self) -> StrBuf { StrBuf::from_str(self).yellow() }
    fn blue(self) -> StrBuf { StrBuf::from_str(self).blue() }
    fn magenta(self) -> StrBuf { StrBuf::from_str(self).magenta() }
    fn cyan(self) -> StrBuf { StrBuf::from_str(self).cyan() }
    fn grey(self) -> StrBuf { StrBuf::from_str(self).grey() }
    fn default(self) -> StrBuf { StrBuf::from_str(self).default() }
    fn b_black(self) -> StrBuf { StrBuf::from_str(self).b_black() }
    fn b_red(self) -> StrBuf { StrBuf::from_str(self).b_red() }
    fn b_green(self) -> StrBuf { StrBuf::from_str(self).b_green() }
    fn b_yellow(self) -> StrBuf { StrBuf::from_str(self).b_yellow() }
    fn b_blue(self) -> StrBuf { StrBuf::from_str(self).b_blue() }
    fn b_magenta(self) -> StrBuf { StrBuf::from_str(self).b_magenta() }
    fn b_cyan(self) -> StrBuf { StrBuf::from_str(self).b_cyan() }
    fn b_grey(self) -> StrBuf { StrBuf::from_str(self).b_grey() }

    // Background
    fn blackb(self) -> StrBuf { StrBuf::from_str(self).blackb() }
    fn redb(self) -> StrBuf { StrBuf::from_str(self).redb() }
    fn greenb(self) -> StrBuf { StrBuf::from_str(self).greenb() }
    fn yellowb(self) -> StrBuf { StrBuf::from_str(self).yellowb() }
    fn blueb(self) -> StrBuf { StrBuf::from_str(self).blueb() }
    fn magentab(self) -> StrBuf { StrBuf::from_str(self).magentab() }
    fn cyanb(self) -> StrBuf { StrBuf::from_str(self).cyanb() }
    fn greyb(self) -> StrBuf { StrBuf::from_str(self).greyb() }
    fn defaultb(self) -> StrBuf { StrBuf::from_str(self).defaultb() }
    fn b_blackb(self) -> StrBuf { StrBuf::from_str(self).b_blackb() }
    fn b_redb(self) -> StrBuf { StrBuf::from_str(self).b_redb() }
    fn b_greenb(self) -> StrBuf { StrBuf::from_str(self).b_greenb() }
    fn b_yellowb(self) -> StrBuf { StrBuf::from_str(self).b_yellowb() }
    fn b_blueb(self) -> StrBuf { StrBuf::from_str(self).b_blueb() }
    fn b_magentab(self) -> StrBuf { StrBuf::from_str(self).b_magentab() }
    fn b_cyanb(self) -> StrBuf { StrBuf::from_str(self).b_cyanb() }
    fn b_greyb(self) -> StrBuf { StrBuf::from_str(self).b_greyb() }

    // styles
    fn underscore(self) -> StrBuf { StrBuf::from_str(self).underscore() }
    fn bold(self) -> StrBuf { StrBuf::from_str(self).bold() }
    fn blink(self) -> StrBuf { StrBuf::from_str(self).blink() }
    fn reverse(self) -> StrBuf { StrBuf::from_str(self).reverse() }
    fn concealed(self) -> StrBuf { StrBuf::from_str(self).concealed() }
    fn faint(self) -> StrBuf { StrBuf::from_str(self).faint() }
    fn italic(self) -> StrBuf { StrBuf::from_str(self).italic() }
    fn crossedout(self) -> StrBuf { StrBuf::from_str(self).crossedout() }
}
