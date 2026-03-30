use std::io;

fn count_set_bits(z: u32) -> u32 {
    if z == 0 {
        0
    } else {
        count_set_bits(z >> 1) + (z & 1)
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Input must be a number");

    // Preconditions check
    assert!(n <= 20, "n must be less than or equal to 20");

    let mut a = vec![0; 20];
    let mut x = vec![vec![0; 20]; 20];
    let mut y = vec![vec![0; 20]; 20];

    for i in 0..n {
        let mut a_input = String::new();
        io::stdin().read_line(&mut a_input).expect("Failed to read line");
        a[i] = a_input.trim().parse().expect("Input must be a number");

        // Preconditions check
        assert!(a[i] <= 20, "a[i] must be less than or equal to 20");

        for j in 0..a[i] {
            let mut constraint_input = String::new();
            io::stdin().read_line(&mut constraint_input).expect("Failed to read line");
            let constraints: Vec<usize> = constraint_input
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Input must be a number"))
                .collect();

            // Preconditions check
            assert!(constraints[0] >= 1 && constraints[0] <= n, "x[i][j] must be in range 1 to n");
            assert!(constraints[1] == 0 || constraints[1] == 1, "y[i][j] must be either 0 or 1");

            x[i][j] = constraints[0];
            y[i][j] = constraints[1];
        }
    }

    let mut max = 0;

    for i in 0..(1 << n) {
        let mut flag = true;

        for j in 0..n {
            if (i & (1 << j)) == 0 {
                continue;
            }
            for k in 0..a[j] {
                if ((i >> (x[j][k] - 1)) & 1) ^ y[j][k] != 0 {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }

        // If no contradictions were found
        if flag {
            let count = count_set_bits(i);
            if max < count {
                max = count;
            }
        }
    }

    println!("{}", max);
}
