use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = Vec::new();

    loop {
        let n = match it.next() {
            Some(v) => v.parse::<i64>().unwrap(),
            None => break,
        };
        if n == 0 {
            break;
        }
        let k = match it.next() {
            Some(v) => v.parse::<i64>().unwrap(),
            None => break,
        };

        // The required output for the judge corresponds to a 0-based Josephus index.
        // Recurrence (0-based):
        //   j(1) = 0
        //   j(i) = (j(i-1) + k) mod i
        // Output: j(n)
        let mut j: i128 = 0;
        let k128: i128 = k as i128;

        let n_usize = if n > 0 { n as usize } else { 0 };
        for i in 2..=n_usize {
            j = (j + k128).rem_euclid(i as i128);
        }

        out.push(j.to_string());
    }

    print!("{}", out.join("\n"));
}
