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

pub fn test_fibonnaci() {
    println!("Testing fibonacci");

    let fib_num_result: u32 = get_fib_num(6);
    assert_eq!(fib_num_result, 8);

    let fib_num_result: u32 = get_fib_num(0);
    assert_eq!(fib_num_result, 0);

    println!("Fibonacci test passed successfully.");
    println!("\n");
}
