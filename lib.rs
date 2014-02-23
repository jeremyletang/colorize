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

fn finalize(mut color: ~str, text: &str) -> ~str {
    color.push_str(text);
    color.push_str("\x1b[0;39;49m");
    color
}

impl AnsiColor for ~str {
    // Foreground
    fn blackf(self) -> ~str { finalize(~"\x1b[30m", self) }
    fn redf(self) -> ~str { finalize(~"\x1b[31m", self) }
    fn greenf(self) -> ~str { finalize(~"\x1b[32m", self) }
    fn yellowf(self) -> ~str { finalize(~"\x1b[33m", self) }
    fn bluef(self) -> ~str { finalize(~"\x1b[34m", self) }
    fn magentaf(self) -> ~str { finalize(~"\x1b[35m", self) }
    fn cyanf(self) -> ~str { finalize(~"\x1b[36m", self) }
    fn whitef(self) -> ~str { finalize(~"\x1b[37m", self) }
    // Background
    fn blackb(self) -> ~str { finalize(~"\x1b[40m", self) }
    fn redb(self) -> ~str { finalize(~"\x1b[41m", self) }
    fn greenb(self) -> ~str { finalize(~"\x1b[42m", self) }
    fn yellowb(self) -> ~str { finalize(~"\x1b[43m", self) }
    fn blueb(self) -> ~str { finalize(~"\x1b[44m", self) }
    fn magentab(self) -> ~str { finalize(~"\x1b[45m", self) }
    fn cyanb(self) -> ~str { finalize(~"\x1b[46m", self) }
    fn whiteb(self) -> ~str { finalize(~"\x1b[47m", self) }

    // styles
    fn underscore(self) -> ~str { finalize(~"\x1b[4m", self) }
    fn bold(self) -> ~str { finalize(~"\x1b[1m", self) }
    fn blink(self) -> ~str { finalize(~"\x1b[5m", self) }
    fn reverse(self) -> ~str { finalize(~"\x1b[7m", self) }
    fn concealed(self) -> ~str { finalize(~"\x1b[8m", self) }
    fn faint(self) -> ~str { finalize(~"\x1b[2m", self) }
    fn italic(self) -> ~str { finalize(~"\x1b[3m", self) }
    fn crossedout(self) -> ~str { finalize(~"\x1b[9m", self) }
}

impl AnsiColor for &'static str {
    // Foreground
    fn blackf(self) -> ~str { finalize(~"\x1b[30m", self) }
    fn redf(self) -> ~str { finalize(~"\x1b[31m", self) }
    fn greenf(self) -> ~str { finalize(~"\x1b[32m", self) }
    fn yellowf(self) -> ~str { finalize(~"\x1b[33m", self) }
    fn bluef(self) -> ~str { finalize(~"\x1b[34m", self) }
    fn magentaf(self) -> ~str { finalize(~"\x1b[35m", self) }
    fn cyanf(self) -> ~str { finalize(~"\x1b[36m", self) }
    fn whitef(self) -> ~str { finalize(~"\x1b[37m", self) }
    // Background
    fn blackb(self) -> ~str { finalize(~"\x1b[40m", self) }
    fn redb(self) -> ~str { finalize(~"\x1b[41m", self) }
    fn greenb(self) -> ~str { finalize(~"\x1b[42m", self) }
    fn yellowb(self) -> ~str { finalize(~"\x1b[43m", self) }
    fn blueb(self) -> ~str { finalize(~"\x1b[44m", self) }
    fn magentab(self) -> ~str { finalize(~"\x1b[45m", self) }
    fn cyanb(self) -> ~str { finalize(~"\x1b[46m", self) }
    fn whiteb(self) -> ~str { finalize(~"\x1b[47m", self) }

    // styles
    fn underscore(self) -> ~str { finalize(~"\x1b[4m", self) }
    fn bold(self) -> ~str { finalize(~"\x1b[1m", self) }
    fn blink(self) -> ~str { finalize(~"\x1b[5m", self) }
    fn reverse(self) -> ~str { finalize(~"\x1b[7m", self) }
    fn concealed(self) -> ~str { finalize(~"\x1b[8m", self) }
    fn faint(self) -> ~str { finalize(~"\x1b[2m", self) }
    fn italic(self) -> ~str { finalize(~"\x1b[3m", self) }
    fn crossedout(self) -> ~str { finalize(~"\x1b[9m", self) }
}
