// this function makes the conversion.
// if the argument ends with C, it assumes its a temperature in celsius.
// if argument ends with F, it assumes the its a temperature in fahrenheit.
pub fn convert(value: &mut String) {
    if value.trim().ends_with("C") {
        println!("calculating for fahrenheit...");
        let fahrenheit = (trunc(value) / 9.0) * 32.0;
        println!("{}F", fahrenheit);
    } else if value.trim().ends_with("F") {
        println!("calculating for celsius...");
        let celsius = (trunc(value) * 9.0) / 32.0;
        println!("{}C", celsius);
    } else {
        println!("Invalid value, value must end with either 'C' or 'F'.");
    }
}

// This function takes an argument, makes a new string by removing the last character.
// function also converts string value to number and returns the number.
fn trunc(arg: &mut String) -> f64 {
    let new_value = match arg.truncate(arg.len() - 2) {
        () => arg,
    };

    let calc_value: f64 = match new_value.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            0.0
        }
    };

    return calc_value;
}
