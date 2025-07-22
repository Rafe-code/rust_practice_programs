// Temperature functions
fn to_celsius(fahrenheit: f32) -> f32 {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    return celsius;
}

fn to_fahrenheit(celsius: f32) -> f32 {
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    return fahrenheit;
}

pub fn test_temp_conversion() {
    println!("Testing fahrenheit celsius converter");

    // test with some quick numbers
    let mut pass_temp_conv: bool = true;

    let test_result = to_celsius(32.0);
    if test_result != 0.0 {
        pass_temp_conv = false;
    }
    let test_result: f32 = to_fahrenheit(20.0);
    if test_result != 68.0 {
        pass_temp_conv = false;
    }
    // test over, print results
    if pass_temp_conv {
        println!("Temperature test passed successfully.")
    } else {
        println!("Temperature test failed.")
    }
    println!("\n")
}
