fn transform_input(input: i64) -> i64 {
    // Calculate the quotient and remainder when input is divided by 11
    let quotient = input / 11;
    let remainder = input % 11;

    // Determine the output based on the specified conditions
    if remainder == 0 {
        // If divisible by 11, return twice the quotient
        2 * quotient
    } else if remainder > 6 {
        // If remainder is greater than 6, return twice the quotient plus 2
        2 * quotient + 2
    } else {
        // If remainder is 6 or less, return twice the quotient plus 1
        2 * quotient + 1
    }
}

fn main() {
    let input: i64 = 123; // Replace with any valid long long integer
    let result = transform_input(input);
    println!("{}", result);
}
