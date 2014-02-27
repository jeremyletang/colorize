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

#[crate_id = "colorize_tests"];

extern crate colorize;
use colorize::AnsiColor;

//use colorize::{Red, Greenb};

fn main() {
    //colorize::global_fg(Red);
    //colorize::global_bg(Greenb);
    println!("{}", "\tTest foreground color for owned str".b_greenf());
    tests::foreground_color_owned_str();
    println!("{}", "\tTest background color for owned str".b_greenf());
    tests::background_color_owned_str();
    println!("{}", "\tTest foreground color for &'static str".b_greenf());
    tests::foreground_color_ref_str();
    println!("{}", "\tTest background color for &'static str".b_greenf());
    tests::background_color_ref_str();
    println!("{}", "\tTest custome styles for owned str".b_greenf());
    tests::custom_styles_owned_str();
    println!("{}", "\tTest custome styles for ref str".b_greenf());
    tests::custom_styles_ref_str();
    tests::final_test();
}

mod tests {
    use colorize::AnsiColor;

    pub fn foreground_color_owned_str() {
        println!("{}", (~"Black").blackf());
        println!("{}", (~"Bright black").b_blackf());
        println!("{}", (~"Red").redf());
        println!("{}", (~"Bright Red").b_redf());
        println!("{}", (~"Green").greenf());
        println!("{}", (~"Bright Green").b_greenf());
        println!("{}", (~"Yellow").yellowf());
        println!("{}", (~"Bright Yellow").b_yellowf());
        println!("{}", (~"Blue").bluef());
        println!("{}", (~"Bright Blue").b_bluef());
        println!("{}", (~"Magenta").magentaf());
        println!("{}", (~"Bright Magenta").b_magentaf());
        println!("{}", (~"Cyan").cyanf());
        println!("{}", (~"Bright Cyan").b_cyanf());
        println!("{}", (~"Grey").greyf());
        println!("{}", (~"Bright Grey").b_greyf());
        println!("{}", (~"Hello world").defaultf());
    }

    pub fn background_color_owned_str() {
        println!("{}", (~"Black").blackb());
        println!("{}", (~"Bright black").b_blackb());
        println!("{}", (~"Red").redb());
        println!("{}", (~"Bright Red").b_redb());
        println!("{}", (~"Green").greenb());
        println!("{}", (~"Bright Green").b_greenb());
        println!("{}", (~"Yellow").yellowb());
        println!("{}", (~"Bright Yellow").b_yellowb());
        println!("{}", (~"Blue").blueb());
        println!("{}", (~"Bright Blue").b_blueb());
        println!("{}", (~"Magenta").magentab());
        println!("{}", (~"Bright Magenta").b_magentab());
        println!("{}", (~"Cyan").cyanb());
        println!("{}", (~"Bright Cyan").b_cyanb());
        println!("{}", (~"Grey").greyb());
        println!("{}", (~"Bright Grey").b_greyb());
        println!("{}", (~"Hello world").defaultb());
    }

    pub fn foreground_color_ref_str() {
        println!("{}", "Black".blackf());
        println!("{}", "Bright black".b_blackf());
        println!("{}", "Red".redf());
        println!("{}", "Bright Red".b_redf());
        println!("{}", "Green".greenf());
        println!("{}", "Bright Green".b_greenf());
        println!("{}", "Yellow".yellowf());
        println!("{}", "Bright Yellow".b_yellowf());
        println!("{}", "Blue".bluef());
        println!("{}", "Bright Blue".b_bluef());
        println!("{}", "Magenta".magentaf());
        println!("{}", "Bright Magenta".b_magentaf());
        println!("{}", "Cyan".cyanf());
        println!("{}", "Bright Cyan".b_cyanf());
        println!("{}", "Grey".greyf());
        println!("{}", "Bright Grey".b_greyf());
        println!("{}", "Hello world".defaultf());
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
    
    pub fn custom_styles_owned_str() {
        println!("{}", (~"Hello world").underscore());
        println!("{}", (~"Hello world").bold());
        println!("{}", (~"Hello world").blink());
        println!("{}", (~"Hello world").reverse());
        println!("{}", (~"Hello world").concealed());
    }

    pub fn custom_styles_ref_str() {
        println!("{}", "Hello world".underscore());
        println!("{}", "Hello world".bold());
        println!("{}", "Hello world".blink());
        println!("{}", "Hello world".reverse());
        println!("{}", "Hello world".concealed());
    }

    pub fn final_test() {
        println!("{}", "Super final test combo !".magentaf().blink()
                 .b_yellowb().underscore());
    }
}

