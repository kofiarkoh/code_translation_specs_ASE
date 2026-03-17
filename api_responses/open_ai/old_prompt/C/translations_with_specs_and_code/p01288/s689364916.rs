use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    
    let mut input = String::new();
    while reader.read_line(&mut input).unwrap() > 0 {
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        
        if n == 0 {
            break;
        }

        let mut p = vec![0; n + 1];
        let mut m = vec![0; n + 1];
        let mut queries = Vec::with_capacity(q);
        let mut r: i64 = 0;

        m[1] = 1; // Mark the first element as marked
        for i in 2..=n {
            let t: usize = iter.next().unwrap().parse().unwrap();
            p[i] = t;
        }

        input.clear();
        reader.read_line(&mut input).unwrap();
        for i in 0..q {
            let parts: Vec<&str> = input.split_whitespace().collect();
            let query_type = parts[0].chars().next().unwrap();
            let v: usize = parts[1].parse().unwrap();
            queries.push((query_type, v));

            if query_type == 'M' {
                m[v] += 1;
            }
            input.clear();
            reader.read_line(&mut input).unwrap();
        }

        for i in (0..q).rev() {
            let (query_type, v) = &queries[i];
            if *query_type == 'M' {
                m[*v] -= 1;
            } else {
                let mut t = *v;
                while m[t] == 0 {
                    t = p[t];
                }
                r += t as i64;
            }
        }

        println!("{}", r);
        input.clear();
    }
}
