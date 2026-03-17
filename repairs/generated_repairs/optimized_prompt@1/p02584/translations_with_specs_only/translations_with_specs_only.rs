fn modified_absolute_value(target: i32, divisor: i32, count: u32) -> i32 {
    if divisor == 0 {
        panic!("Divisor cannot be zero.");
    }

    let abs_target = target.abs();
    let mut result = abs_target;

    for _ in 0..count {
        result /= divisor;
    }

    // Ensure the result is rounded down to the nearest integer
    result = result.max(0); // Ensure result is non-negative
    result
}

fn main() {
    let target = 10;
    let divisor = 3;
    let count = 4;

    let output = modified_absolute_value(target, divisor, count);
    println!("{}", output);
}
