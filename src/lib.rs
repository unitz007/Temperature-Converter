// this function makes the conversion.
// if the argument ends with C, it assumes its a temperature in celsius.
// if argument ends with F, it assumes the its a temperature in fahrenheit.
pub fn convert(value: &mut String) -> f32 {
    let mut result = 0.0;
    if value.trim().ends_with("C") {
        println!("converting to fahrenheit...");
        result = trunc(value) * 1.8 + 32.0;
        println!("{:.2}F", result);
    } else if value.trim().ends_with("F") {
        println!("converting to celsius...");
        result = trunc(value) - 32.0 / 1.8;
        println!("{:.2}C", result);
    } else {
        println!("Invalid value, value must end with either 'C' or 'F'.");
    }

    result
}

// This function takes an argument, makes a new string by removing the last character.
// function also converts string value to number and returns the number.
fn trunc(arg: &mut String) -> f32 {
    let new_value = match arg.truncate(arg.len() - 2) {
        () => arg,
    };

    let calc_value: f32 = match new_value.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            0.0
        }
    };

    return calc_value;
}

#[test]
fn celsius_fahrenheit() {
    assert_eq!(convert(&mut "44C".to_string()), 39.2);
}

#[test]
fn fahrenheit_celsius() {
    assert_eq!(convert(&mut "44F".to_string()), -13.777779);
}
