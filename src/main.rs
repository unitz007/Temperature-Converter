use std::env;

use temp_converter;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    temp_converter::convert(&mut args[1]);
}