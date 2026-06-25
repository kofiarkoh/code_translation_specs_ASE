use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let n_usize = n as usize;

        let mut a: Vec<i32> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            let v: i32 = it.next().unwrap().parse().unwrap();
            a.push(v);
        }

        // qsort with comparator: return (*(int*)b) - (*(int*)s)
        // That subtraction can overflow in C (UB), but typical intent is descending sort.
        // Use a safe descending sort by value.
        a.sort_unstable_by(|x, y| y.cmp(x));

        let mut m: i32 = 0;
        let mut k: i32 = 0; // In C, k is only used if n/2 < m; keep a default.

        // Emulate the C loop structure, including the j < n+1 bound and a[n] access.
        // In the C code, a[n] is read when the last run reaches the end; that's out-of-bounds (UB).
        // The intended behavior is to treat it as a sentinel different from a[i] to flush the last run.
        // We emulate that by using a virtual sentinel at index n that is guaranteed != a[i].
        let mut i: usize = 0;
        while i < n_usize {
            let mut j: usize = i + 1;
            while j < n_usize + 1 {
                let aj_differs = if j == n_usize {
                    true // sentinel differs from any real value
                } else {
                    a[i] != a[j]
                };

                if aj_differs {
                    let run_len = (j - i) as i32;
                    if m < run_len {
                        m = run_len;
                        k = a[i];
                    }
                    i = j - 1; // matches: i = j-1; then outer loop i++
                    break;
                }
                j += 1;
            }
            i += 1;
        }

        if (n / 2) < m {
            out.push_str(&format!("{k}\n"));
        } else {
            out.push_str("NO COLOR\n");
        }
    }

    print!("{out}");
}
