// Rust implementation based on the provided specification

pub fn fibonacci(n: usize) -> u64 {
    // Calculate the nth Fibonacci number using an iterative approach
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

pub fn gcd(a: i32, b: i32) -> i32 {
    // Calculate the GCD using the Euclidean algorithm
    let mut a = a.abs(); // Ensure non-negative
    let mut b = b.abs(); // Ensure non-negative
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn ngcd(arr: &[i32]) -> i32 {
    // Calculate the GCD of an array of integers
    if arr.is_empty() {
        return 0; // Undefined behavior for empty array, returning 0
    }
    let mut result = arr[0];
    for &num in &arr[1..] {
        result = gcd(result, num);
    }
    result
}

pub fn lcm(a: i32, b: i32) -> i32 {
    // Calculate the LCM using GCD
    if a == 0 || b == 0 {
        return 0; // LCM is undefined for zero
    }
    (a.abs() / gcd(a, b)) * b.abs()
}

pub fn is_prime(n: i32) -> i32 {
    // Check if a number is prime
    if n <= 1 {
        return 0; // 0 and 1 are not prime
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return 0; // Not prime
        }
    }
    1 // Is prime
}

pub fn is_substr(s: &str, t: &str, i: usize) -> i32 {
    // Check if t is a substring of s starting at index i
    if i + t.len() > s.len() {
        return 0; // Out of bounds
    }
    if &s[i..i + t.len()] == t {
        return 1; // Is substring
    }
    0 // Not a substring
}

pub fn factorize(n: u64, ftr: &mut Vec<u64>, num: &mut usize) {
    // Factorize a number and store factors in ftr
    *num = 0; // Reset factor count
    let mut n = n;
    for i in 2..=((n as f64).sqrt() as u64) {
        while n % i == 0 {
            ftr.push(i);
            n /= i;
            *num += 1; // Increment factor count
        }
    }
    if n > 1 {
        ftr.push(n);
        *num += 1; // Include the last factor if greater than 1
    }
}

pub fn next_perm(p: &mut Vec<i32>) -> i32 {
    // Generate the next permutation in lexicographical order
    let len = p.len();
    if len <= 1 {
        return 0; // No next permutation
    }
    
    let mut i = len - 2;
    while i != usize::MAX && p[i] >= p[i + 1] {
        i -= 1;
    }
    
    if i == usize::MAX {
        p.reverse(); // Last permutation, reverse to first
        return 0; // No next permutation
    }
    
    let mut j = len - 1;
    while p[j] <= p[i] {
        j -= 1;
    }
    
    p.swap(i, j);
    p[i + 1..].reverse(); // Reverse the suffix
    1 // Next permutation exists
}

pub fn comb(size1: usize, size2: usize, result: &mut Vec<Vec<u64>>) {
    // Populate a 2D array with combination values
    result.clear(); // Clear previous results
    for i in 0..=size1 {
        let mut row = vec![0; size2 + 1];
        for j in 0..=i.min(size2) {
            if j == 0 || j == i {
                row[j] = 1; // C(i, 0) = C(i, i) = 1
            } else {
                row[j] = result[i - 1][j - 1] + result[i - 1][j]; // C(i, j) = C(i-1, j-1) + C(i-1, j)
            }
        }
        result.push(row);
    }
}

pub fn abs_val(a: i32, b: i32) -> i32 {
    // Return the absolute difference between two integers
    (a - b).abs()
}
