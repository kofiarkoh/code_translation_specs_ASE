use std::io::{self, Read};

fn main() {
    // Read entire stdin; input is assumed well-formed per spec (undefined otherwise).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let q: usize = it.next().unwrap().parse().unwrap();

        // C code loops "for(; scanf(...), N;)" i.e., stops when N == 0.
        if n == 0 {
            break;
        }

        // Arrays are indexed by element value; C uses indices 0..N for P/M, but queries use 1..N.
        // We'll allocate N+1 and keep index 0 unused for the query domain.
        let mut p = vec![0usize; n + 1];
        let mut m = vec![0i32; n + 1];

        // Initialize: for(i=r=0;i<N;P[i]=i,M[i]=0,i++);
        // This sets P[0..N-1]=i and M[0..N-1]=0. We'll do 0..=N for safety.
        for i in 0..=n {
            p[i] = i;
            m[i] = 0;
        }

        // M[1]=1;
        if n >= 1 {
            m[1] = 1;
        }

        // for(i=2;i<=N;P[i]=t,i++) scanf("%d\n",&t);
        // Reads N-1 integers for i=2..N and sets P[i]=t.
        for i in 2..=n {
            let t: usize = it.next().unwrap().parse().unwrap();
            p[i] = t;
        }

        // Read queries, count 'M' occurrences.
        let mut qs = vec!['\0'; q];
        let mut vs = vec![0usize; q];
        for i in 0..q {
            let ch_str = it.next().unwrap();
            let ch = ch_str.as_bytes()[0] as char; // tokens are single-char per spec/C usage
            let v: usize = it.next().unwrap().parse().unwrap();
            qs[i] = ch;
            vs[i] = v;
            if ch == 'M' {
                m[v] += 1;
            }
        }

        // Process queries in reverse.
        let mut r: i64 = 0;
        for i in (0..q).rev() {
            if qs[i] == 'M' {
                m[vs[i]] -= 1;
            } else {
                let mut t = vs[i];
                // for(t=v[i];!M[t];) t=P[t];
                while m[t] == 0 {
                    t = p[t];
                }
                r += t as i64;
            }
        }

        // Output exactly one line per test case.
        println!("{}", r);
    }
}
