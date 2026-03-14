use std::io::{self, Write};

const SIZE: usize = 10;

struct Solver {
    val: [i32; SIZE],
    s: i32,
}

impl Solver {
    fn new() -> Self {
        Solver {
            val: [0; SIZE],
            s: 0,
        }
    }

    fn get_way(&mut self, x: usize, n: i32, sum: i32) -> i32 {
        if n == 0 {
            return if sum == self.s { 1 } else { 0 };
        }

        let mut ret = 0;
        for i in x..SIZE {
            if self.val[i] == 0 && sum + i as i32 <= self.s {
                self.val[i] = 1;
                ret += self.get_way(i, n - 1, i as i32 + sum);
                self.val[i] = 0;
            }
        }

        ret
    }
}

fn main() {
    let mut input = String::new();
    let mut solver = Solver::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().expect("Invalid input");
        solver.s = iter.next().unwrap().parse().expect("Invalid input");

        if n == 0 && solver.s == 0 {
            break;
        }

        solver.val = [0; SIZE]; // Reset val array
        println!("{}", solver.get_way(0, n, 0));
    }
}
