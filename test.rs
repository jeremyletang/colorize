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

fn main() {
    println!("Test foreground color for owned str");
    tests::foreground_color_owned_str();
    println!("Test background color for owned str");
    tests::background_color_owned_str();
    println!("Test foreground color for &'static str");
    tests::foreground_color_ref_str();
    println!("Test background color for &'static str");
    tests::background_color_ref_str();
    println!("Test custome styles for owned str");
    tests::custom_styles_owned_str();
    println!("Test custome styles for ref str");
    tests::custom_styles_ref_str();
}

mod tests {
    use colorize::AnsiColor;

    pub fn foreground_color_owned_str() {
        println!("{}", (~"Hello world").blackf());
        println!("{}", (~"Hello world").redf());
        println!("{}", (~"Hello world").greenf());
        println!("{}", (~"Hello world").yellowf());
        println!("{}", (~"Hello world").bluef());
        println!("{}", (~"Hello world").magentaf());
        println!("{}", (~"Hello world").cyanf());
        println!("{}", (~"Hello world").whitef());
    }

    pub fn background_color_owned_str() {
        println!("{}", (~"Hello world").blackb());
        println!("{}", (~"Hello world").redb());
        println!("{}", (~"Hello world").greenb());
        println!("{}", (~"Hello world").yellowb());
        println!("{}", (~"Hello world").blueb());
        println!("{}", (~"Hello world").magentab());
        println!("{}", (~"Hello world").cyanb());
        println!("{}", (~"Hello world").whiteb());
    }

    pub fn foreground_color_ref_str() {
        println!("{}", "Hello world".blackf());
        println!("{}", "Hello world".redf());
        println!("{}", "Hello world".greenf());
        println!("{}", "Hello world".yellowf());
        println!("{}", "Hello world".bluef());
        println!("{}", "Hello world".magentaf());
        println!("{}", "Hello world".cyanf());
        println!("{}", "Hello world".whitef());
    }

    pub fn background_color_ref_str() {
        println!("{}", "Hello world".blackb());
        println!("{}", "Hello world".redb());
        println!("{}", "Hello world".greenb());
        println!("{}", "Hello world".yellowb());
        println!("{}", "Hello world".blueb());
        println!("{}", "Hello world".magentab());
        println!("{}", "Hello world".cyanb());
        println!("{}", "Hello world".whiteb());
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
}

