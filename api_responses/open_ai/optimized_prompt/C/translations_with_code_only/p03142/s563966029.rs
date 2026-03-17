use std::io::{self, BufRead};
use std::ptr;

const MAX_SIZE: usize = 114514;

struct Graph {
    ec: Vec<usize>,
    es: Vec<Vec<usize>>,
    sansyo: Vec<usize>,
    next_count: [usize; 2],
    next_buffer: [[usize; MAX_SIZE]; 2],
    answer: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            ec: vec![0; n + 1],
            es: vec![Vec::new(); n + 1],
            sansyo: vec![0; n + 1],
            next_count: [0, 0],
            next_buffer: [[0; MAX_SIZE]; 2],
            answer: vec![0; n + 1],
        }
    }

    fn ae(&mut self, f: usize, t: usize) {
        self.es[f].push(t);
        self.ec[f] += 1;
    }

    fn process(&mut self, n: usize) {
        for i in 1..=n {
            if self.sansyo[i] == 0 {
                self.next_buffer[0][self.next_count[0]] = i;
                self.next_count[0] += 1;
            }
        }

        let mut phase = 0;
        while self.next_count[phase] > 0 {
            self.next_count[1 - phase] = 0;
            for i in 0..self.next_count[phase] {
                let target = self.next_buffer[phase][i];
                for &neighbor in &self.es[target] {
                    if self.sansyo[neighbor] > 0 {
                        self.sansyo[neighbor] -= 1;
                        if self.sansyo[neighbor] == 0 {
                            self.answer[neighbor] = target;
                            self.next_buffer[1 - phase][self.next_count[1 - phase]] = neighbor;
                            self.next_count[1 - phase] += 1;
                        }
                    }
                }
            }
            phase = 1 - phase;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    let mut graph = Graph::new(n);

    for _ in 0..(n - 1 + m) {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        graph.ae(a, b);
        graph.sansyo[b] += 1;
    }

    graph.process(n);

    for i in 1..=n {
        println!("{}", graph.answer[i]);
    }
}
