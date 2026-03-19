use std::io::{self, Read};

fn main() {
    // Read entire stdin, token-scan like scanf("%d %s %s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let s_str = it.next().unwrap_or("");
    let t_str = it.next().unwrap_or("");

    // C uses VLA char S[N+1], T[N+1] and scanf("%s") (no bounds checking).
    // We emulate the intended behavior for well-formed inputs: treat S and T as byte arrays.
    // Indexing is by byte, matching C char indexing.
    let s = s_str.as_bytes();
    let t = t_str.as_bytes();

    let mut ans: i32 = 0;

    // Mirror C's int variables and control flow as closely as possible.
    let mut i: i32 = n - 1;
    let mut j: i32 = i;

    // Note: In the C code, the outer `while (i>=0)` never changes `i` (it is shadowed inside),
    // so termination relies on `break`. We preserve that structure.
    while i >= 0 {
        // This shadows the outer i in C; in Rust we just create a new variable.
        let mut i_inner: i32 = n - 1;

        while j >= 0 && s[i_inner as usize] != t[j as usize] {
            j -= 1;
        }

        if j >= 0 {
            while j >= 0 && s[i_inner as usize] == t[j as usize] {
                i_inner -= 1;
                j -= 1;
            }
            if j < 0 {
                ans = 2 * n - (n - i_inner - 1);
                break;
            }
        } else {
            ans = 2 * n - (n - i_inner - 1);
            break;
        }
    }

    // C prints without a trailing newline.
    print!("{}", ans);
}
