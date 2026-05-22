use std::io::{self, Read};

fn emit_perm(perm: &[i32], out: &mut String) {
    for (i, v) in perm.iter().enumerate() {
        if i != 0 {
            out.push(' ');
        }
        out.push_str(&v.to_string());
    }
    out.push('\n');
}

fn gen(n: i32, used: &mut [bool], perm: &mut Vec<i32>, out: &mut String) {
    if perm.len() == n as usize {
        emit_perm(perm, out);
        return;
    }

    for v in 1..=n {
        let idx = (v - 1) as usize;
        if !used[idx] {
            used[idx] = true;
            perm.push(v);
            gen(n, used, perm, out);
            perm.pop();
            used[idx] = false;
        }
    }
}

fn main() {
    // No explicit error handling per spec: parsing failures are allowed to panic/abort (undefined).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Spec says negative input is undefined; we do not define behavior here.
    if n == 0 {
        // Exactly one empty line.
        print!("\n");
        return;
    }

    let mut used = vec![false; n as usize];
    let mut perm: Vec<i32> = Vec::with_capacity(n as usize);
    let mut out = String::new();

    gen(n, &mut used, &mut perm, &mut out);

    print!("{out}");
}
