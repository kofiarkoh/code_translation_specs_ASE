use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior is undefined if not exactly nine integers (per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut c = [[0i32; 3]; 3];
    let mut row_min = [0i32; 3];

    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = it.next().unwrap().parse::<i32>().unwrap();
        }
        // Compute minimum of the row (matches C logic, but expressed idiomatically).
        row_min[i] = c[i][0].min(c[i][1]).min(c[i][2]);
    }

    // Normalize by subtracting row minimums.
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] -= row_min[i];
        }
    }

    // Check if all columns are equal across rows in the normalized matrix.
    for j in 0..3 {
        if c[0][j] != c[1][j] || c[0][j] != c[2][j] {
            print!("No\n");
            return;
        }
    }

    print!("Yes\n");
}
