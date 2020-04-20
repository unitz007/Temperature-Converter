use std::env;

use temp_converter;

// main function reads command line argument and passes it to convert function. 
fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        println!("Usage: temp_converter \"<Temperature>\"");
    } else {
        temp_converter::convert(&mut args[0]);
    }
}
