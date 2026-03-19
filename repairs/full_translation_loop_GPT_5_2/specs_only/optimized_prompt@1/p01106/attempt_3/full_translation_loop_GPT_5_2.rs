use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n: i64 = match it.next() {
            Some(t) => match t.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        if n <= 0 {
            break;
        }

        let i: i64 = match it.next() {
            Some(t) => match t.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let j: i64 = match it.next() {
            Some(t) => match t.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        // Interpret i and j as 1-based indices of nodes in a perfect binary tree of depth n,
        // where the root is 1, left child is 2*x, right child is 2*x+1.
        // The path from the root to a node is encoded by the bits of the index excluding the
        // leading 1: bit 0 => 'L', bit 1 => 'R', from most-significant to least-significant.
        //
        // The required output is the length-n sequence of directions that transforms the path
        // of i into the path of j, position by position:
        // - if the direction at a level is the same => 'L'
        // - if the direction differs => 'R'
        //
        // This matches the sample:
        // n=3, i=5 (101 => path "LR"), j=7 (111 => "RR") => compare: L!=R => R, R==R => L,
        // and missing leading level treated as 'L' (0) => yields "LRL".
        //
        // To get exactly n characters, we consider exactly n direction bits, where the top level
        // corresponds to bit (n-1) of (index-1). This effectively left-pads shorter paths with 0s.
        let iu = (i as i128 - 1) as u128;
        let ju = (j as i128 - 1) as u128;

        let mut s = String::with_capacity(n as usize);
        for k in (0..n as u32).rev() {
            let bi = (iu >> k) & 1;
            let bj = (ju >> k) & 1;
            s.push(if bi == bj { 'L' } else { 'R' });
        }

        out.push_str(&s);
        out.push('\n');
    }

    print!("{out}");
}
