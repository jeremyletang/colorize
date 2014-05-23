// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang
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

#![crate_id = "colorize_tests"]

extern crate colorize;
use colorize::AnsiColor;

//use colorize::{Red, Greenb};

fn main() {
    //colorize::global_fg(Red);
    //colorize::global_bg(Greenb);
    println!("{}", "\tTest foreground color for strbuf".b_green());
    tests::foreground_color_strbuf();
    println!("{}", "\tTest background color for strbuf".greenb());
    tests::background_color_strbuf();
    println!("{}", "\tTest foreground color for &'static str".b_green());
    tests::foreground_color_ref_str();
    println!("{}", "\tTest background color for &'static str".greenb());
    tests::background_color_ref_str();
    println!("{}", "\tTest custome styles for owned str".b_green());
    tests::custom_styles_strbuf();
    println!("{}", "\tTest custome styles for ref str".b_green());
    tests::custom_styles_ref_str();
    tests::final_test();
}

mod tests {
    use colorize::AnsiColor;

    pub fn foreground_color_strbuf() {
        println!("{}", "Black".to_strbuf().black());
        println!("{}", "Bright black".to_strbuf().b_black());
        println!("{}", "Red".to_strbuf().red());
        println!("{}", "Bright Red".to_strbuf().b_red());
        println!("{}", "Green".to_strbuf().green());
        println!("{}", "Bright Green".to_strbuf().b_green());
        println!("{}", "Yellow".to_strbuf().yellow());
        println!("{}", "Bright Yellow".to_strbuf().b_yellow());
        println!("{}", "Blue".to_strbuf().blue());
        println!("{}", "Bright Blue".to_strbuf().b_blue());
        println!("{}", "Magenta".to_strbuf().magenta());
        println!("{}", "Bright Magenta".to_strbuf().b_magenta());
        println!("{}", "Cyan".to_strbuf().cyan());
        println!("{}", "Bright Cyan".to_strbuf().b_cyan());
        println!("{}", "Grey".to_strbuf().grey());
        println!("{}", "Bright Grey".to_strbuf().b_grey());
        println!("{}", "Hello world".to_strbuf().default());
    }

    pub fn background_color_strbuf() {
        println!("{}", "Black".to_strbuf().blackb());
        println!("{}", "Bright black".to_strbuf().b_blackb());
        println!("{}", "Red".to_strbuf().redb());
        println!("{}", "Bright Red".to_strbuf().b_redb());
        println!("{}", "Green".to_strbuf().greenb());
        println!("{}", "Bright Green".to_strbuf().b_greenb());
        println!("{}", "Yellow".to_strbuf().yellowb());
        println!("{}", "Bright Yellow".to_strbuf().b_yellowb());
        println!("{}", "Blue".to_strbuf().blueb());
        println!("{}", "Bright Blue".to_strbuf().b_blueb());
        println!("{}", "Magenta".to_strbuf().magentab());
        println!("{}", "Bright Magenta".to_strbuf().b_magentab());
        println!("{}", "Cyan".to_strbuf().cyanb());
        println!("{}", "Bright Cyan".to_strbuf().b_cyanb());
        println!("{}", "Grey".to_strbuf().greyb());
        println!("{}", "Bright Grey".to_strbuf().b_greyb());
        println!("{}", "Hello world".to_strbuf().defaultb());
    }

    pub fn foreground_color_ref_str() {
        println!("{}", "Black".black());
        println!("{}", "Bright black".b_black());
        println!("{}", "Red".red());
        println!("{}", "Bright Red".b_red());
        println!("{}", "Green".green());
        println!("{}", "Bright Green".b_green());
        println!("{}", "Yellow".yellow());
        println!("{}", "Bright Yellow".b_yellow());
        println!("{}", "Blue".blue());
        println!("{}", "Bright Blue".b_blue());
        println!("{}", "Magenta".magenta());
        println!("{}", "Bright Magenta".b_magenta());
        println!("{}", "Cyan".cyan());
        println!("{}", "Bright Cyan".b_cyan());
        println!("{}", "Grey".grey());
        println!("{}", "Bright Grey".b_grey());
        println!("{}", "Hello world".default());
    }

    pub fn background_color_ref_str() {
        println!("{}", "Black".blackb());
        println!("{}", "Bright black".b_blackb());
        println!("{}", "Red".redb());
        println!("{}", "Bright Red".b_redb());
        println!("{}", "Green".greenb());
        println!("{}", "Bright Green".b_greenb());
        println!("{}", "Yellow".yellowb());
        println!("{}", "Bright Yellow".b_yellowb());
        println!("{}", "Blue".blueb());
        println!("{}", "Bright Blue".b_blueb());
        println!("{}", "Magenta".magentab());
        println!("{}", "Bright Magenta".b_magentab());
        println!("{}", "Cyan".cyanb());
        println!("{}", "Bright Cyan".b_cyanb());
        println!("{}", "Grey".greyb());
        println!("{}", "Bright Grey".b_greyb());
        println!("{}", "Hello world".defaultb());
    }

    pub fn custom_styles_strbuf() {
        println!("{}", "Hello world".to_strbuf().underscore());
        println!("{}", "hello world".to_strbuf().bold());
        println!("{}", "Hello world".to_strbuf().blink());
        println!("{}", "Hello world".to_strbuf().reverse());
        println!("{}", "Hello world".to_strbuf().concealed());
    }

    pub fn custom_styles_ref_str() {
        println!("{}", "Hello world".underscore());
        println!("{}", "Hello world".bold());
        println!("{}", "Hello world".blink());
        println!("{}", "Hello world".reverse());
        println!("{}", "Hello world".concealed());
    }

    pub fn final_test() {
        println!("{}", "Super final test combo !".magenta().blink()
                 .b_yellowb().underscore());
    }
}
