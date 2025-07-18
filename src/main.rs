const RUN_TEMP_COVERSION: bool = true;
const RUN_FIB_TEST: bool = true;
fn main() {
    // TEMPERATURE CONVERSION
    if RUN_TEMP_COVERSION {
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
        // println!("\n")
    }
    // FIBONACCI TEST
    if RUN_FIB_TEST {
        println!("Testing fibonacci");

        let mut pass_fib_test: bool = true;

        let fib_num_result: u32 = get_fib_num(6);
        if fib_num_result != 8 {
            pass_fib_test = false;
        }
        let fib_num_result: u32 = get_fib_num(0);
        if fib_num_result != 0 {
            pass_fib_test = false;
        }
        // test over, print results
        if pass_fib_test {
            println!("Fibonacci test passed successfully.")
        } else {
            println!("Fibonacci test failed.")
        }
        println!("\n")
    }
}

// Temperature functions
fn to_celsius(fahrenheit: f32) -> f32 {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    return celsius;
}

fn to_fahrenheit(celsius: f32) -> f32 {
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    return fahrenheit;
}

// Fibonacci functions
fn get_fib_num(n_want: u32) -> u32 {
    let mut fib_num: u32 = 0;
    let mut fib_num_prev: u32 = 1;

    for n in 1..n_want {
        if n == 1 {
            fib_num = 1;
            fib_num_prev = 1;
        } else {
            fib_num += fib_num_prev;
            fib_num_prev = fib_num - fib_num_prev;
        }
    }
    return fib_num;
}
