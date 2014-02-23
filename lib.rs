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

#[crate_id = "colorize#0.1"];
#[desc = "Terminal color library"];
#[license = "MIT"];
#[crate_type = "dylib"];
#[crate_type = "rlib"];

static BLACK: &'static str = "30";
static RED: &'static str = "31";
static GREEN: &'static str = "32";
static YELLOW: &'static str = "33";
static BLUE: &'static str = "34";
static MAGENTA: &'static str = "35";
static CYAN: &'static str = "36";
static WHITE: &'static str = "37";

static BLACKB: &'static str = "40";
static REDB: &'static str = "41";
static GREENB: &'static str = "42";
static YELLOWB: &'static str = "43";
static BLUEB: &'static str = "44";
static MAGENTAB: &'static str = "45";
static CYANB: &'static str = "46";
static WHITEB: &'static str = "47";

static UNDERSCORE: &'static str = "4";
static BOLD: &'static str = "1";
static BLINK: &'static str = "5";
static REVERSE: &'static str = "7";
static CONCEALED: &'static str = "8";
static FAINT: &'static str = "2";
static ITALIC: &'static str = "3";
static CROSSEDOUT: &'static str = "9";

pub trait AnsiColor {
    /// Foreground black
    fn blackf(self) -> ~str;
    /// Foreground red
    fn redf(self) -> ~str;
    /// Foreground green
    fn greenf(self) -> ~str;
    /// Foreground yellow
    fn yellowf(self) -> ~str;
    /// Foreground blue
    fn bluef(self) -> ~str;
    /// Foreground magenta
    fn magentaf(self) -> ~str;
    /// Foreground cyan
    fn cyanf(self) -> ~str;
    /// Foreground white
    fn whitef(self) -> ~str;

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
    /// Background white
    fn whiteb(self) -> ~str;

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
    // Faint mod ON
    fn faint(self) -> ~str;
    // Italic text
    fn italic(self) -> ~str;
    // Crossed out
    fn crossedout(self) -> ~str;
}


fn finalize(color: &str, mut text: ~str) -> ~str {
    if text.starts_with("\x1b[") {
        text.insert(2, format!("{};", color));
    } else {
        text.insert(0, format!("\x1b[{}m", color));
        text.push_str("\x1b[0;39;49m");
    }
    text
}

impl AnsiColor for ~str {
    // Foreground
    fn blackf(self) -> ~str { finalize(BLACK, self) }
    fn redf(self) -> ~str { finalize(RED, self) }
    fn greenf(self) -> ~str { finalize(GREEN, self) }
    fn yellowf(self) -> ~str { finalize(YELLOW, self) }
    fn bluef(self) -> ~str { finalize(BLUE, self) }
    fn magentaf(self) -> ~str { finalize(MAGENTA, self) }
    fn cyanf(self) -> ~str { finalize(CYAN, self) }
    fn whitef(self) -> ~str { finalize(WHITE, self) }
    // Background
    fn blackb(self) -> ~str { finalize(BLACKB, self) }
    fn redb(self) -> ~str { finalize(REDB, self) }
    fn greenb(self) -> ~str { finalize(GREENB, self) }
    fn yellowb(self) -> ~str { finalize(YELLOWB, self) }
    fn blueb(self) -> ~str { finalize(BLUEB, self) }
    fn magentab(self) -> ~str { finalize(MAGENTAB, self) }
    fn cyanb(self) -> ~str { finalize(CYANB, self) }
    fn whiteb(self) -> ~str { finalize(WHITEB, self) }

    // styles
    fn underscore(self) -> ~str { finalize(UNDERSCORE, self) }
    fn bold(self) -> ~str { finalize(BOLD, self) }
    fn blink(self) -> ~str { finalize(BLINK, self) }
    fn reverse(self) -> ~str { finalize(REVERSE, self) }
    fn concealed(self) -> ~str { finalize(CONCEALED, self) }
    fn faint(self) -> ~str { finalize(FAINT, self) }
    fn italic(self) -> ~str { finalize(ITALIC, self) }
    fn crossedout(self) -> ~str { finalize(CROSSEDOUT, self) }
}


impl AnsiColor for &'static str {
    // Foreground
    fn blackf(self) -> ~str { self.into_owned().blackf() }
    fn redf(self) -> ~str { self.into_owned().redf() }
    fn greenf(self) -> ~str { self.into_owned().greenf() }
    fn yellowf(self) -> ~str { self.into_owned().yellowf() }
    fn bluef(self) -> ~str { self.into_owned().bluef() }
    fn magentaf(self) -> ~str { self.into_owned().magentaf() }
    fn cyanf(self) -> ~str { self.into_owned().cyanf() }
    fn whitef(self) -> ~str { self.into_owned().whitef() }
    // Background
    fn blackb(self) -> ~str { self.into_owned().blackb() }
    fn redb(self) -> ~str { self.into_owned().redb() }
    fn greenb(self) -> ~str { self.into_owned().greenb() }
    fn yellowb(self) -> ~str { self.into_owned().yellowb() }
    fn blueb(self) -> ~str { self.into_owned().blueb() }
    fn magentab(self) -> ~str { self.into_owned().magentab() }
    fn cyanb(self) -> ~str { self.into_owned().cyanb() }
    fn whiteb(self) -> ~str { self.into_owned().whiteb() }

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
