// Task:
// Given a list of integers use a vector to return:
// median
// mode

use std::collections::HashMap;

fn calc_median_even(numbers: &Vec<i32>, numbers_len: usize) -> f32 {
    let index_1: usize = ((numbers_len as i32) / 2) as usize;
    let index_2: usize = ((numbers_len as i32) / 2 - 1) as usize;

    let median_val: f32 = (numbers[index_1] as f32 + numbers[index_2] as f32) / 2.0;
    return median_val;
}
fn get_median(numbers: &Vec<i32>) -> f32 {
    let mut numbers_clone = numbers.clone();
    numbers_clone.sort();

    match (numbers_clone.len() as i32) % 2 {
        0 => calc_median_even(&numbers_clone, numbers_clone.len()),
        _ => numbers_clone[((numbers_clone.len() as i32) / 2) as usize] as f32,
    }
}

// not going to bother thinking about ties
fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut numbers_counts: HashMap<i32, i32> = HashMap::new();

    for i in 0..numbers.len() {
        let count = numbers_counts.entry(numbers[i]).or_insert(0);
        *count += 1;
    }

    let mut mode = 0; // initialising as 0 isn't good practice, but eh
    let mut largest_count: i32 = 0;

    for (number, count) in numbers_counts {
        if count > largest_count {
            mode = number;
            largest_count = count;
        }
    }
    mode
}

pub fn test_median() {
    // check odd and even numbers
    let numbers: Vec<i32> = vec![0, 1, 2, 3, 4];
    let median_val = get_median(&numbers);
    assert_eq!(median_val, 2.0);

    let numbers: Vec<i32> = vec![0, 1, 2, 3];
    let median_val = get_median(&numbers);
    assert_eq!(median_val, 1.5);

    println!("Median tests have passed.")
}

pub fn test_mode() {
    let numbers: Vec<i32> = vec![0, 1, 2, 0, 2, 2, 2];
    let mode_val = get_mode(&numbers);
    assert_eq!(mode_val, 2);

    let numbers: Vec<i32> = vec![20; 500];
    let mode_val = get_mode(&numbers);
    assert_eq!(mode_val, 20);

    println!("Mode tests have passed.")
}
