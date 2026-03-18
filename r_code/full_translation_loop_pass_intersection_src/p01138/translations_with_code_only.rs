use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse tokens similarly to scanf.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut time = vec![0i32; 24 * 3600];

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // EOF
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        // memset(time, 0, sizeof(time));
        time.fill(0);

        for _ in 0..n {
            let dep_tok = it.next().unwrap();
            let arr_tok = it.next().unwrap();

            let dep = parse_hms(dep_tok);
            let arr = parse_hms(arr_tok);

            // C code assumes indices are within [0, 24*3600) and dep < arr.
            // We preserve the same behavior for valid inputs.
            for j in dep..arr {
                time[j] += 1;
            }
        }

        let mut maxv = 0i32;
        for &v in &time {
            if maxv < v {
                maxv = v;
            }
        }

        print!("{maxv}\n");
    }
}

fn parse_hms(s: &str) -> usize {
    // Parses "HH:MM:SS" as in scanf("%d:%d:%d", ...).
    let mut parts = s.split(':');
    let h: i32 = parts.next().unwrap().parse().unwrap();
    let m: i32 = parts.next().unwrap().parse().unwrap();
    let sec: i32 = parts.next().unwrap().parse().unwrap();
    (h * 3600 + m * 60 + sec) as usize
}
