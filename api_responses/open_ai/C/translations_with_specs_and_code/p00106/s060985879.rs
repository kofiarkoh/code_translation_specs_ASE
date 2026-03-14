// Aizu Vol-1 0106: Discounts of Buckwheat
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX_VALUE: usize = 50;
const TABLE_SIZE: usize = 60;

struct CostCalculator {
    tbl: [i32; TABLE_SIZE],
    a: [i32; 3],
    s: [i32; 3],
    c: [i32; 3],
    f: [i32; 3],
}

impl CostCalculator {
    fn new() -> Self {
        Self {
            tbl: [0; TABLE_SIZE],
            a: [2, 3, 5],
            s: [380, 550, 850],
            c: [10, 12, 15],
            f: [1520, 1870, 2244],
        }
    }

    fn combi(&mut self, i: usize, v: usize, cost: i32) {
        if i == 3 {
            if v <= MAX_VALUE && (self.tbl[v] == 0 || self.tbl[v] > cost) {
                self.tbl[v] = cost;
            }
            return;
        }
        for j in (0..=MAX_VALUE - v).step_by(self.a[i] as usize) {
            let cc = (j / self.c[i]) * self.f[i] + (j % self.c[i]) / self.a[i] * self.s[i];
            self.combi(i + 1, v + j as usize, cost + cc);
        }
    }

    fn calculate(&mut self) {
        self.combi(0, 0, 0);
    }

    fn get_cost(&self, value: i32) -> i32 {
        if value < 0 || value % 100 != 0 {
            // Invalid input handling is not specified, but we can return 0 for safety.
            return 0;
        }
        let index = (value / 100) as usize;
        if index < TABLE_SIZE {
            self.tbl[index]
        } else {
            0 // Out of bounds, return 0
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut calculator = CostCalculator::new();
    
    calculator.calculate();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        println!("{}", calculator.get_cost(n));
    }
}
