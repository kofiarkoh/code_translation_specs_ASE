use std::io::{self, BufRead};

const MAX: usize = 100003;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of individuals (chairs)
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    let mut p = vec![0; n + 1];
    let mut hi = vec![0; n + 1];
    let mut q: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut ans = vec![0; n + 1];
    let mut sp = Vec::new();
    
    // Read chair preferences
    for i in 1..=n {
        let a: usize = lines.next().unwrap().unwrap().parse().unwrap();
        p[i] = a;
        hi[a] += 1;
    }

    // Allocate space for preferences
    for i in 1..=n {
        q[i] = vec![0; hi[i]];
        hi[i] = 0; // Reset hi for usage as a counter
    }

    // Fill the preference lists
    for i in 1..=n {
        let a = p[i];
        q[a][hi[a]] = i;
        hi[a] += 1;
    }

    let mut top = 0;

    // Assign chairs based on preferences
    for _ in 0..2 {
        for i in 1..=n {
            while hi[i] > 0 {
                top += 1;
                hi[i] -= 1;
                sp.push(q[i][hi[i]]);
            }
            if ans[i] == 0 && top > 0 {
                top -= 1;
                ans[i] = sp.pop().unwrap();
            }
        }
    }

    // Output the final seating arrangement
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
