fn main() {
    // Read input values for n and m
    let (n, m) = read_input();

    // Preconditions: m must be greater than 1 and n must be non-negative
    if m <= 1 || n < 0 {
        panic!("Invalid input: m must be greater than 1 and n must be non-negative.");
    }

    // Initialize arrays for factorials and inverse factorials
    let mut fact = vec![1; (n + 1) as usize];
    let mut invfact = vec![1; (n + 1) as usize];

    // Compute factorials and their modular inverses
    make_fact(n, m, &mut fact, &mut invfact);

    // Compute the result based on the combinatorial expression
    let result = compute_result(n, m, &fact, &invfact);

    // Output the result
    println!("{}", result);
}

// Function to read input values for n and m
fn read_input() -> (i32, i32) {
    // For simplicity, we assume valid input is provided
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

// Function to compute factorials and their inverses
fn make_fact(n: i32, m: i32, fact: &mut Vec<i32>, invfact: &mut Vec<i32>) {
    for i in 1..=n {
        fact[i as usize] = (fact[(i - 1) as usize] * i) % m;
    }
    // Compute inverses using Fermat's Little Theorem
    invfact[n as usize] = mod_inverse(fact[n as usize], m);
    for i in (1..=n).rev() {
        invfact[(i - 1) as usize] = (invfact[i as usize] * i) % m;
    }
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(a: i32, m: i32) -> i32 {
    // a^(m-2) mod m
    let mut result = 1;
    let mut exp = m - 2;
    let mut base = a;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m;
        }
        base = (base * base) % m;
        exp /= 2;
    }
    result
}

// Function to compute the final result based on the combinatorial expression
fn compute_result(n: i32, m: i32, fact: &Vec<i32>, invfact: &Vec<i32>) -> i32 {
    if n == 0 {
        return 0; // Edge case: if n is 0, output is 0
    }
    let mut result = 0;
    for k in 0..=n {
        let comb = (fact[n as usize] * invfact[k as usize] % m * invfact[(n - k) as usize] % m) % m;
        result = (result + comb * mod_pow(2, k, m)) % m;
    }
    result
}

// Function to compute modular exponentiation
fn mod_pow(base: i32, exp: i32, m: i32) -> i32 {
    let mut result = 1;
    let mut b = base % m;
    let mut e = exp;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        b = (b * b) % m;
        e /= 2;
    }
    result
}
