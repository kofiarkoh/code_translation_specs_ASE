use std::io::{self, Read};

fn main() {
    // Input format (inferred from failing sample):
    // H W
    // H lines of text grid
    // R C
    // R lines of pattern grid
    //
    // The specification doesn't pin down the exact textual format, but this is the narrowest
    // interpretation consistent with the sample and typical grid-search problems.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();

    let mut text: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = it.next().unwrap_or("");
        text.push(row.as_bytes().to_vec());
    }

    let r: usize = it.next().unwrap().parse().unwrap();
    let c: usize = it.next().unwrap().parse().unwrap();

    let mut pat: Vec<Vec<u8>> = Vec::with_capacity(r);
    for _ in 0..r {
        let row = it.next().unwrap_or("");
        pat.push(row.as_bytes().to_vec());
    }

    if r == 0 || c == 0 || h < r || w < c {
        return;
    }

    let mut out = String::new();
    for i in 0..=h - r {
        for j in 0..=w - c {
            if exact_match(&text, &pat, i, j, r, c) {
                out.push_str(&format!("{} {}\n", i, j));
            }
        }
    }
    print!("{out}");
}

fn exact_match(text: &[Vec<u8>], pat: &[Vec<u8>], si: usize, sj: usize, r: usize, c: usize) -> bool {
    for i in 0..r {
        // If row lengths are shorter than expected, behavior is undefined by spec;
        // we treat it as non-match to avoid panics.
        if text[si + i].len() < sj + c || pat[i].len() < c {
            return false;
        }
        for j in 0..c {
            if text[si + i][sj + j] != pat[i][j] {
                return false;
            }
        }
    }
    true
}
