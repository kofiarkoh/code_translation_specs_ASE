use std::collections::HashMap;

fn is_perfect_square(x: u64) -> bool {
    let root = (x as f64).sqrt() as u64;
    root * root == x
}

fn is_perfect_cube(x: u64) -> bool {
    let root = (x as f64).cbrt() as u64;
    root * root * root == x
}

fn prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut factors = HashMap::new();
    let mut num = n;

    // Check for number of 2s that divide n
    while num % 2 == 0 {
        *factors.entry(2).or_insert(0) += 1;
        num /= 2;
    }

    // n must be odd at this point, so we can skip even numbers
    let mut factor = 3;
    while factor * factor <= num {
        while num % factor == 0 {
            *factors.entry(factor).or_insert(0) += 1;
            num /= factor;
        }
        factor += 2;
    }

    // This condition is to check if num is a prime number greater than 2
    if num > 2 {
        *factors.entry(num).or_insert(0) += 1;
    }

    factors
}

fn count_integers(n: usize, arr: &[u64]) -> u64 {
    if n == 0 {
        return 0; // Handle empty input case
    }

    let mut count = 0;
    let mut occurrences = HashMap::new();

    for &num in arr {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    for (&num, &occurrence) in &occurrences {
        if num == 1 {
            count += occurrence; // Count 1s directly
            continue;
        }

        let factors = prime_factors(num);
        let product: u64 = factors.iter().map(|(k, v)| k.pow(*v as u32)).product();

        if product > 10_000_000_000 {
            count += occurrence; // Increment based on original count
            continue;
        }

        if is_perfect_square(num) || is_perfect_cube(num) {
            count += occurrence; // Count perfect squares and cubes
        }
    }

    count
}

fn main() {
    let n = 5; // Example input size
    let arr = vec![60, 125, 1000000, 27, 7]; // Example input array
    let result = count_integers(n, &arr);
    println!("{}", result); // Output the result
}
