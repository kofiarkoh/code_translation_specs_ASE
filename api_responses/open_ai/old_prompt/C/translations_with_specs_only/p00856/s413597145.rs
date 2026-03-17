// Minimal Backgammon Simulation

const MAX_SQUARES: usize = 100;
const DBL_MAX: f64 = std::f64::MAX;

fn main() {
    let (n, t, l, b, sq) = read_input();
    let mut memo = vec![vec![DBL_MAX; MAX_SQUARES + 1]; MAX_SQUARES + 1];

    let probability = solve(n, t, &sq, &mut memo);
    println!("{}", probability);
}

// Reads input values for N, T, L, B and the board configuration
fn read_input() -> (usize, usize, usize, usize, Vec<char>) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let values: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = values[0];
    let t = values[1];
    let l = values[2];
    let b = values[3];
    let mut sq = vec![' '; MAX_SQUARES];

    // Populate the sq array based on L and B counts
    for i in 0..l {
        sq[i] = 'L';
    }
    for i in l..(l + b) {
        sq[i] = 'B';
    }

    (n, t, l, b, sq)
}

// Recursive function to calculate the probability of reaching the end
fn solve(n: usize, t: usize, sq: &[char], memo: &mut Vec<Vec<f64>>) -> f64 {
    if t == 0 {
        return if n == 0 { 1.0 } else { 0.0 };
    }
    if memo[n][t] != DBL_MAX {
        return memo[n][t];
    }

    let mut probability = 0.0;

    // Check possible moves
    for move_distance in 1..=6 {
        if n >= move_distance {
            match sq[n - move_distance] {
                'B' => continue, // Blocked square
                _ => probability += solve(n - move_distance, t - 1, sq, memo),
            }
        }
    }

    // Store the computed probability in memo
    memo[n][t] = probability;
    probability
}
