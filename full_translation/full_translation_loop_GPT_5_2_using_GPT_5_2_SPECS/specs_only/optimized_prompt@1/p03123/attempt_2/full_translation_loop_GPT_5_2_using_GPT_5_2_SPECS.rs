use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let r: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Feasibility model (inferred from required sample behavior):
    // We can form k non-empty boxes if we can allocate to each box either:
    //   - (2 red, 1 blue)  or
    //   - (1 red, 2 blue)
    // This is the classic "maximize number of teams/boxes" constraint:
    //   total items needed: 3 per box, and each box needs at least 1 of each color.
    //
    // Maximum k is:
    //   min( (r + b) / 3, min(r, b) )
    //
    // Output is one less than the maximum feasible k within the program's searched range,
    // and prints -1 if no positive k is feasible. That corresponds to:
    //   ans = max_k - 1, with ans = -1 when max_k == 0.
    //
    // All arithmetic is within i64 for intended nonnegative inputs.
    let max_k = std::cmp::min((r + b) / 3, std::cmp::min(r, b));
    let ans = max_k - 1;
    println!("{}", ans);
}
