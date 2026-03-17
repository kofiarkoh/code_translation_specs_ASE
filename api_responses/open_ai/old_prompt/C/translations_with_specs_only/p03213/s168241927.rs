fn main() {
    // Define the upper limit for factorization
    let n: usize = 100; // Assuming n is read from input and is valid (>= 2)

    // Initialize the yaku array to hold counts of prime factors
    let mut yaku = [0; 101]; // Fixed size array as per specification

    // Populate the yaku array with counts of prime factors for each integer from 2 to n
    for i in 2..=n {
        let mut num = i;
        let mut factor = 2;

        // Factorization process
        while factor * factor <= num {
            while num % factor == 0 {
                yaku[factor] += 1; // Increment the count for this prime factor
                num /= factor; // Divide num by the factor
            }
            factor += 1; // Move to the next potential factor
        }
        if num > 1 {
            yaku[num] += 1; // If num is still greater than 1, it is prime
        }
    }

    // Compute the final result based on the populated yaku array
    let result: i32 = yaku.iter().sum(); // Sum of all prime factor counts

    // Print the result followed by a newline
    println!("{}", result);
}
