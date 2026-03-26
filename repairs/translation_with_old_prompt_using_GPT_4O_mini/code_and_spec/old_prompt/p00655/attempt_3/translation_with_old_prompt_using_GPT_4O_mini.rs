use std::io::{self, BufRead};

const FH: usize = 0; // First half
const SH: usize = 1; // Second half
const ALL: usize = 2;
const MAX: usize = 200005;

struct Queues {
    que: [[i32; MAX]; 2],
    qt: [usize; 2],
    qe: [usize; 2],
    min: [[i32; MAX]; 2],
    at: [usize; 2],
    ae: [usize; 2],
    max: [[i32; MAX]; 2],
    zt: [usize; 2],
    ze: [usize; 2],
}

impl Queues {
    fn new() -> Self {
        Queues {
            que: [[0; MAX]; 2],
            qt: [0; 2],
            qe: [0; 2],
            min: [[0; MAX]; 2],
            at: [0; 2],
            ae: [0; 2],
            max: [[0; MAX]; 2],
            zt: [0; 2],
            ze: [0; 2],
        }
    }

    fn size(&self, id: usize) -> usize {
        self.qe[id] - self.qt[id]
    }

    fn min_size(&self, id: usize) -> usize {
        self.ae[id] - self.at[id]
    }

    fn max_size(&self, id: usize) -> usize {
        self.ze[id] - self.zt[id]
    }

    fn del(&mut self, id: usize) -> i32 {
        let a;
        if id == ALL {
            a = self.del(FH);
            if self.size(FH) < self.size(SH) {
                let temp = self.del(SH);
                self.add(FH, temp);
            }
        } else if id == FH {
            a = self.que[id][self.qe[id] - 1];
            self.qe[id] -= 1;
            if self.min[id][self.ae[id] - 1] == a {
                self.ae[id] -= 1;
            }
            if self.max[id][self.ze[id] - 1] == a {
                self.ze[id] -= 1;
            }
        } else {
            a = self.que[id][self.qt[id]];
            self.qt[id] += 1;
            if self.min[id][self.at[id]] == a {
                self.at[id] += 1;
            }
            if self.max[id][self.zt[id]] == a {
                self.zt[id] += 1;
            }
        }
        a
    }

    fn add(&mut self, id: usize, a: i32) {
        if id == ALL {
            self.add(SH, a);
            if self.size(FH) < self.size(SH) {
                let temp = self.del(SH);
                self.add(FH, temp);
            }
            return;
        }
        self.que[id][self.qe[id]] = a;
        self.qe[id] += 1;
        if self.size(id) == 1 {
            self.min[id][self.ae[id]] = a;
            self.ae[id] += 1;
            self.max[id][self.ze[id]] = a;
            self.ze[id] += 1;
        } else if id == FH {
            if self.min[id][self.ae[id] - 1] > a {
                self.min[id][self.ae[id]] = a;
                self.ae[id] += 1;
            }
            if self.max[id][self.ze[id] - 1] < a {
                self.max[id][self.ze[id]] = a;
                self.ze[id] += 1;
            }
        } else {
            while self.min_size(id) > 0 && self.min[id][self.ae[id] - 1] > a {
                self.ae[id] -= 1;
            }
            while self.max_size(id) > 0 && self.max[id][self.ze[id] - 1] < a {
                self.ze[id] -= 1;
            }
            self.min[id][self.ae[id]] = a;
            self.ae[id] += 1;
            self.max[id][self.ze[id]] = a;
            self.ze[id] += 1;
        }
    }

    fn gmin(&self, id: usize, th: usize) -> i32 {
        if id == FH {
            self.min[id][self.ae[id] - th]
        } else {
            self.min[id][self.at[id] + th - 1]
        }
    }

    fn gmax(&self, id: usize, th: usize) -> i32 {
        if id == FH {
            self.max[id][self.ze[id] - th]
        } else {
            self.max[id][self.zt[id] + th - 1]
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut queues = Queues::new();

    while let Some(line) = lines.next() {
        let q: usize = line.trim().parse().unwrap_or(0);
        queues.qt = [0; 2];
        queues.qe = [0; 2];
        queues.at = [0; 2];
        queues.ae = [0; 2];
        queues.zt = [0; 2];
        queues.ze = [0; 2];

        for _ in 0..q {
            let op: usize = lines.next().unwrap_or_else(|_| "0".to_string()).trim().parse().unwrap_or(0);
            let a: i32;

            match op {
                0 => {
                    a = lines.next().unwrap_or_else(|_| "0".to_string()).trim().parse().unwrap_or(0);
                    queues.add(ALL, a);
                }
                1 => {
                    a = queues.del(ALL);
                    println!("{}", a);
                }
                2 => {
                    a = queues.gmin(FH, 1);
                    println!("{}", a);
                }
                3 => {
                    a = queues.gmin(SH, 1);
                    println!("{}", a);
                }
                4 => {
                    let th: usize = lines.next().unwrap_or_else(|_| "1".to_string()).trim().parse().unwrap_or(1);
                    a = queues.gmin(FH, th);
                    println!("{}", a);
                }
                5 => {
                    let th: usize = lines.next().unwrap_or_else(|_| "1".to_string()).trim().parse().unwrap_or(1);
                    a = queues.gmin(SH, th);
                    println!("{}", a);
                }
                6 => {
                    a = queues.gmax(FH, 1);
                    println!("{}", a);
                }
                7 => {
                    a = queues.gmax(SH, 1);
                    println!("{}", a);
                }
                8 => {
                    let th: usize = lines.next().unwrap_or_else(|_| "1".to_string()).trim().parse().unwrap_or(1);
                    a = queues.gmax(FH, th);
                    println!("{}", a);
                }
                _ => {
                    let th: usize = lines.next().unwrap_or_else(|_| "1".to_string()).trim().parse().unwrap_or(1);
                    a = queues.gmax(SH, th);
                    println!("{}", a);
                }
            }
        }
        println!("end");
    }
}
