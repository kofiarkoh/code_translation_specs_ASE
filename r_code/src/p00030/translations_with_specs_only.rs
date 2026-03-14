use std::io;

fn main() {
    loop {
        let (n, s) = read_input();
        if n == 0 && s == 0 {
            break; // Termination condition
        }
        if n > 10 || s < 0 {
            continue; // Undefined behavior, ignore input
        }
        let count = count_combinations(n, s);
        println!("{}", count);
    }
}

fn read_input() -> (usize, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let n = parts.get(0).and_then(|&s| s.parse::<usize>().ok()).unwrap_or(0);
    let s = parts.get(1).and_then(|&s| s.parse::<i32>().ok()).unwrap_or(0);
    (n, s)
}

fn count_combinations(n: usize, s: i32) -> i32 {
    let mut result = 0;
    let numbers = (0..10).collect::<Vec<_>>();
    let mut combination = Vec::new();
    backtrack(&numbers, n, s, 0, &mut combination, &mut result);
    result
}

fn backtrack(numbers: &[usize], n: usize, target: i32, start: usize, combination: &mut Vec<usize>, result: &mut i32) {
    if combination.len() == n {
        if target == 0 {
            *result += 1; // Found a valid combination
        }
        return;
    }
    for i in start..numbers.len() {
        let num = numbers[i];
        if num as i32 > target {
            continue; // Skip if the number exceeds the target
        }
        combination.push(num);
        backtrack(numbers, n, target - num as i32, i + 1, combination, result);
        combination.pop(); // Backtrack
    }
}
