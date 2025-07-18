const RUN_TEMP_COVERSION: bool = true;
const RUN_FIB_TEST: bool = true;
const RUN_XMAS_SONG: bool = true;
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
        println!("\n")
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

    if RUN_XMAS_SONG {
        // no test for this one, just print out the lyrics and look
        print_lyrics_xmas()
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

// XMAS SONG FUNCTIONS
const XMAS_LYRICS_BASE_1: &str = "On the "; // always starts
// now text after day name below
const XMAS_LYRICS_BASE_2: &str = " day of Christmas \nMy true love sent to me";
// the lines which get added with each day
const DAYS_XMAS_LYRICS: [&str; 11] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five gold rings",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
];
// first gift changes if its the only gift or at the end of the list
const PARTRIDGE_OPTIONS: [&str; 2] = [
    "A partridge in a pear tree.",
    "And a partridge in a pear tree!",
];
const NUM_DAYS: u32 = 12;
const DAY_NAMES: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];
fn print_lyrics_xmas() {
    let mut day_lyrics = String::new();
    for day_num in 1..NUM_DAYS + 1 {
        //  add base "on the Xth day of xmas..."
        day_lyrics = String::from(XMAS_LYRICS_BASE_1);
        day_lyrics += DAY_NAMES[(day_num - 1) as usize];
        day_lyrics += XMAS_LYRICS_BASE_2;
        day_lyrics += "\n";

        // now start adding the specifics
        if day_num == 1 {
            // if only partridge then special case
            day_lyrics += PARTRIDGE_OPTIONS[0];
        } else {
            // add non partridge gifts
            day_lyrics = add_middle_days(day_lyrics, day_num - 1);
            // and add patridge at the end
            day_lyrics += PARTRIDGE_OPTIONS[1];
        }
        day_lyrics += "\n";
        // print that day's gifts
        println!("{}", day_lyrics)
    }
}

fn add_middle_days(mut day_lyrics: String, day_num: u32) -> String {
    // loop over each gift to be added in this specific day
    for day_num_ii in (1..day_num + 1).rev() {
        // need to access the gifts in reverse order, index reverse here
        let array_el_idx: u32 = (DAYS_XMAS_LYRICS.len() as u32) - day_num_ii;
        // add the specific gift
        day_lyrics += DAYS_XMAS_LYRICS
            .iter()
            .nth(array_el_idx as usize)
            .copied()
            .unwrap();
        day_lyrics += "\n";
    }
    return day_lyrics;
}
