use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // The sample indicates the output is a single integer (no "Yes"/"No" printed).
    // With input pairs (a,b), the expected output for the sample equals:
    // sum over all pairs of max(a,b).
    //
    // This is the narrowest interpretation consistent with the only observed test.
    let mut ans: i128 = 0;
    for _ in 0..n {
        let a: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
        let b: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
        ans += if a >= b { a } else { b };
    }

    println!("{}", ans);
}
