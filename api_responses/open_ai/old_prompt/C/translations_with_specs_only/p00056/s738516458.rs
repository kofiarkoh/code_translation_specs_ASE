// Define the maximum value for n as specified in the specification
const MAX: usize = 50000;

// Function to generate a table of prime numbers using the Sieve of Eratosthenes
fn make_prime_table(table: &mut [u8; MAX]) -> i32 {
    // Initialize the table: 0 for prime, 1 for non-prime
    for i in 0..MAX {
        table[i] = 1; // Assume all numbers are non-prime initially
    }
    table[0] = 0; // 0 is not prime
    table[1] = 0; // 1 is not prime

    // Sieve of Eratosthenes algorithm
    for i in 2..(MAX as f64).sqrt() as usize + 1 {
        if table[i] == 0 {
            continue; // Skip if i is not prime
        }
        for j in (i * i..MAX).step_by(i) {
            table[j] = 1; // Mark multiples of i as non-prime
        }
    }
    0 // Return 0 upon successful execution
}

// Function to count distinct pairs of primes that sum to n
fn count_prime_pairs(n: usize, table: &[u8; MAX]) -> usize {
    let mut count = 0;
    for i in 2..=n / 2 {
        if table[i as usize] == 0 && table[(n - i) as usize] == 0 {
            count += 1; // Found a valid pair of primes
        }
    }
    count
}

// Main function to read input and output the counts of prime pairs
fn main() {
    let mut ptable = [0u8; MAX]; // Allocate the prime table
    make_prime_table(&mut ptable); // Populate the prime table

    // Read integers from standard input
    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        input.clear(); // Clear the input buffer
        stdin.read_line(&mut input).expect("Failed to read line");
        let n: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };

        if n == 0 {
            break; // Terminate if input is 0
        }

        // Count and print the number of distinct pairs of primes that sum to n
        let count = count_prime_pairs(n, &ptable);
        println!("{}", count);
    }
}
