use std::io::{self, BufRead};

struct FenwickTree {
    size: usize,
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        let mut tree = vec![0; size + 1];
        tree[0] = (size + 1) as i32; // First element holds the value 2*n + 1
        FenwickTree { size, tree }
    }

    fn add(&mut self, index: usize, value: i32) {
        let mut idx = index as usize;
        while idx < self.tree.len() {
            self.tree[idx] += value;
            idx += idx & (!idx + 1); // Move to the next index
        }
    }

    fn sum(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut idx = index as usize;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & (!idx + 1); // Move to the parent index
        }
        sum
    }
}

fn count_pairs(a: &[i32], l: i32) -> i32 {
    let mut count = 0;
    for &value in a {
        if value < l {
            count += 1;
        }
    }
    count
}

fn run() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let m = 1; // Placeholder for the threshold during counting

    let mut left = a.iter().cloned().min().unwrap_or(i32::MIN);
    let mut right = a.iter().cloned().max().unwrap_or(i32::MAX);
    let mut result = left;

    let mut bit = FenwickTree::new(2 * n + 2);

    while left <= right {
        let mid = left + (right - left) / 2;
        let valid_pairs = count_pairs(&a, mid);
        let total_pairs = n * (n - 1) / 2;

        if valid_pairs * 2 >= total_pairs {
            result = mid; // Update result if condition is met
            left = mid + 1; // Search for a larger threshold
        } else {
            right = mid - 1; // Search for a smaller threshold
        }
    }

    result
}

fn main() {
    let l = run();
    println!("{}", l);
}
