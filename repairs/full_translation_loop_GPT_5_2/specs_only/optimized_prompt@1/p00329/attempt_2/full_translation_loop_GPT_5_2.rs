use std::io::{self, Read};

fn try_kuhn(u: usize, g: &[Vec<u8>], seen: &mut [bool], mt: &mut [Option<usize>]) -> bool {
    let n = g.len();
    for v in 0..n {
        if g[u][v] != 0 && !seen[v] {
            seen[v] = true;
            if mt[v].is_none() || try_kuhn(mt[v].unwrap(), g, seen, mt) {
                mt[v] = Some(u);
                return true;
            }
        }
    }
    false
}

fn main() {
    // Input format is assumed correct per specification; otherwise behavior is undefined.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_raw: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // The specification says the number is "decreased by one for internal processing".
    // This matches the observed sample: input 4 => internal N = 3 and a 3x3 grid follows.
    // For 0, behavior is unspecified; we keep Rust-defined behavior via saturating_sub.
    let n = n_raw.saturating_sub(1);

    let mut g = vec![vec![0u8; n]; n];
    for i in 0..n {
        for j in 0..n {
            let v: u8 = it.next().unwrap().parse().unwrap();
            g[i][j] = if v == 0 { 0 } else { 1 };
        }
    }

    // mt[v] = Some(u) means right node v is matched to left node u.
    let mut mt: Vec<Option<usize>> = vec![None; n];

    let mut matched = 0usize;
    for u in 0..n {
        let mut seen = vec![false; n];
        if try_kuhn(u, &g, &mut seen, &mut mt) {
            matched += 1;
        }
    }

    if matched == n {
        println!("yes");
        // The expected output for the sample prints, for each left node i (1..=N),
        // the matched right node index (1..=N) on its own line.
        // So we invert mt (right->left) into ans (left->right) and print ans in left order.
        let mut ans: Vec<usize> = vec![0; n];
        for (v, mu) in mt.iter().enumerate() {
            if let Some(u) = *mu {
                ans[u] = v + 1; // 1-based right index
            }
        }
        for u in 0..n {
            println!("{}", ans[u]);
        }
    } else {
        println!("no");
    }
}
