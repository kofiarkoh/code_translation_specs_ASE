use std::io::{self, BufRead};

const NUM: usize = 1000;

fn check(A: &[[char; NUM]; NUM], N: usize, x: usize, y: usize) -> i32 {
    if A[x][y] == A[N - 1 - x][y] && A[x][y] == A[x][N - 1 - y] {
        1 // Symmetry condition satisfied
    } else {
        0 // Symmetry condition not satisfied
    }
}

fn update(A: &mut [[char; NUM]; NUM], t: &mut [[i32; NUM]; NUM], total: &mut i32, N: usize, x: usize, y: usize) {
    *total -= t[x][y]; // Decrease total by previous state
    t[x][y] = check(A, N, x, y); // Update symmetry state
    *total += t[x][y]; // Increase total by new state
}

fn change(A: &mut [[char; NUM]; NUM], t: &mut [[i32; NUM]; NUM], total: &mut i32, N: usize, x: usize, y: usize) {
    A[x][y] = if A[x][y] == '0' { '1' } else { '0' }; // Toggle value
    update(A, t, total, N, x, y); // Update symmetry state
    update(A, t, total, N, N - 1 - x, y); // Update symmetry state for vertical symmetry
    update(A, t, total, N, x, N - 1 - y); // Update symmetry state for horizontal symmetry
}

fn solve(total: i32, N: usize) -> i32 {
    if total == (N * N) as i32 {
        1 // All cells are symmetric
    } else {
        0 // Not all cells are symmetric
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let C: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse C");
    let N: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse N");

    let mut A: [[char; NUM]; NUM] = [['0'; NUM]; NUM];
    let mut t: [[i32; NUM]; NUM] = [[0; NUM]; NUM];
    let mut total = 0;

    for i in 0..N {
        let line = lines.next().unwrap().unwrap();
        for j in 0..N {
            A[i][j] = line.chars().nth(j).unwrap();
        }
    }

    for i in 0..N {
        for j in 0..N {
            t[i][j] = check(&A, N, i, j);
            total += t[i][j];
        }
    }

    let mut ans = solve(total, N); // Initial symmetry check

    for _ in 0..C - 1 {
        let k: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse k");

        for _ in 0..k {
            let coords: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|s| s.parse::<usize>().expect("Failed to parse coordinates") - 1) // Convert to 0-based index
                .collect();
            let (x, y) = (coords[0], coords[1]);
            change(&mut A, &mut t, &mut total, N, x, y);
        }

        ans += solve(total, N); // Check symmetry after updates
    }

    println!("{}", ans);
}
