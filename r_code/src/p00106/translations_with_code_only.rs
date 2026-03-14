// Aizu Vol-1 0106: Discounts of Buckwheat 
// 2017.8.12 bal4u@uu

use std::io::{self, BufRead};

const A: [i32; 3] = [2, 3, 5];
const S: [i32; 3] = [380, 550, 850];
const C: [i32; 3] = [10, 12, 15];
const F: [i32; 3] = [1520, 1870, 2244];

fn combi(i: usize, v: usize, cost: i32, tbl: &mut [i32; 60]) {
    if i == 3 {
        if (v <= 50 && tbl[v] == 0) || (tbl[v] > cost) {
            tbl[v] = cost;
        }
        return;
    }
    for j in (0..=50).step_by(A[i] as usize).take_while(|&j| v + j <= 50) {
        let cc = (j / C[i]) * F[i] + (j % C[i]) / A[i] * S[i];
        combi(i + 1, v + j, cost + cc, tbl);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tbl = [0; 60];
    let mut n;

    combi(0, 0, 0, &mut tbl);
    
    for line in stdin.lock().lines() {
        n = line.unwrap().trim().parse::<i32>().unwrap();
        if n == 0 {
            break;
        }
        println!("{}", tbl[(n / 100) as usize]);
    }
}
