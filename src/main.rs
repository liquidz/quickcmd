extern crate url;

use std::env;
// use url::percent_encoding::lossy_utf8_percent_decode;
use url::percent_encoding::percent_decode;

mod parse;
mod search;

fn decode(s: &str) -> Result<String, std::str::Utf8Error> {
    let res = try!(percent_decode(s.as_bytes()).decode_utf8());
    Ok(res.to_string().replace("+", " "))
}

fn main() {
    // let key = "QUERY_STRING";
    match env::var("HOME") {
        Ok(val) => println!("home = {}", val),
        Err(e) => println!("err: {}", e),
    }
    let sample = "g+http%3A%2F%2Fwww.nifty.com%3Ffoo%3Dbar";
    println!("sample: {}", sample);

    match decode(sample) {
        Ok(res) => {
            println!("res: {:?}", res.parse::<parse::Request>().unwrap());
        }
        Err(e) => println!("err: {}", e),
    }
}
