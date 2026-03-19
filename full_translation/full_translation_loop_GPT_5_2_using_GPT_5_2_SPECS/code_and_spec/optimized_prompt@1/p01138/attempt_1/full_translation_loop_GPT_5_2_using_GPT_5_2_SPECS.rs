use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse tokens. Times are given as "H:M:S" with no spaces.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();
    const DAY: usize = 24 * 3600;

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // unspecified behavior in spec; just stop if input ends
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut time = vec![0u32; DAY];

        for _ in 0..n {
            let start_tok = it.next().unwrap();
            let end_tok = it.next().unwrap();

            let dep = parse_hms_to_seconds(start_tok);
            let arr = parse_hms_to_seconds(end_tok);

            // Spec: interval includes seconds >= start and < end; if equal, contributes nothing.
            for t in dep..arr {
                // Preconditions guarantee t is within the day.
                time[t] += 1;
            }
        }

        let mut maxv: u32 = 0;
        for &v in &time {
            if v > maxv {
                maxv = v;
            }
        }

        out.push_str(&format!("{maxv}\n"));
    }

    print!("{out}");
}

fn parse_hms_to_seconds(s: &str) -> usize {
    let mut parts = s.split(':');
    let h: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let sec: usize = parts.next().unwrap().parse().unwrap();
    h * 3600 + m * 60 + sec
}
