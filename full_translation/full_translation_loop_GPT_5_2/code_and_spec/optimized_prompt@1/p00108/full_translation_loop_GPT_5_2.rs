use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    const MAX_N: usize = 13;
    const NUM_RANGE: usize = 101; // values 0..=100

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // no more input
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        // C code uses fixed arrays of size MAX=13; spec doesn't state max,
        // but we preserve the C program's constraint.
        // If n > 13, behavior is undefined in the original; we likewise do not handle it.
        let mut numline = vec![0usize; n.min(MAX_N)];
        for i in 0..n {
            let v: usize = it.next().unwrap().parse().unwrap();
            if i < numline.len() {
                numline[i] = v;
            }
        }

        let mut tmpline = vec![0usize; numline.len()];
        let mut num = vec![0usize; NUM_RANGE];

        let mut cnt: i32 = 0;
        let mut flag = true;

        while flag {
            // reset counts
            for x in num.iter_mut() {
                *x = 0;
            }

            // count occurrences of each value in current sequence
            for &v in &numline {
                // Preconditions say 0..=100; no extra checks to match C behavior.
                num[v] += 1;
            }

            // build transformed sequence: replace each element by its frequency
            for i in 0..numline.len() {
                tmpline[i] = num[numline[i]];
            }

            cnt += 1;

            // check stability: all positions unchanged
            let mut same = 0usize;
            for i in 0..numline.len() {
                if tmpline[i] == numline[i] {
                    same += 1;
                }
            }

            if same == numline.len() {
                flag = false;
            } else {
                numline.copy_from_slice(&tmpline);
                flag = true;
            }
        }

        // Output: number of transformations performed to reach stability is (cnt - 1),
        // matching the C program's behavior.
        out.push_str(&(cnt - 1).to_string());
        out.push('\n');

        for i in 0..tmpline.len() {
            if i + 1 == tmpline.len() {
                out.push_str(&format!("{}\n", tmpline[i]));
            } else {
                out.push_str(&format!("{} ", tmpline[i]));
            }
        }
    }

    print!("{out}");
}
