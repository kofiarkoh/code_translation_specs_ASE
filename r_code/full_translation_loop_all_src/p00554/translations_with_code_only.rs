use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // no input; C would have undefined behavior, but we just exit
    };
    let m: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C arrays are size 1000, indexed from 1..=m (index 0 unused).
    // We allocate 1001 to allow index 1000 safely.
    let mut a = vec![0i32; 1001];
    let mut b = vec![0i32; 1001];

    let mut count1: i32 = 0;
    let mut count2: i32 = 0;

    // for(i=1;i<=m;i++)
    for i in 1..=m {
        let ai: i32 = it.next().unwrap().parse().unwrap();
        let bi: i32 = it.next().unwrap().parse().unwrap();
        let idx = i as usize;
        a[idx] = ai;
        b[idx] = bi;

        if a[idx] >= n {
            a[idx] = 0;
            b[idx] = 0;
            count1 += 1;
        }
    }

    // if(m-1>count1)
    if m - 1 > count1 {
        // for(i=1;i<=m;i++) b[i]=n-a[i];
        for i in 1..=m {
            let idx = i as usize;
            // Use wrapping to emulate C signed overflow behavior as closely as possible.
            b[idx] = n.wrapping_sub(a[idx]);
        }

        // Selection-sort-like nested loops:
        // for(i=1;i<m;i++) for(j=i+1;j<=m;j++) if(b[i]>b[j]) swap
        for i in 1..m {
            for j in (i + 1)..=m {
                let ii = i as usize;
                let jj = j as usize;
                if b[ii] > b[jj] {
                    let temp = b[ii];
                    b[ii] = b[jj];
                    b[jj] = temp;
                }
            }
        }

        // for(i=1;i<=m-count1-1;i++) count2 += b[i];
        let limit = m - count1 - 1;
        for i in 1..=limit {
            let idx = i as usize;
            count2 = count2.wrapping_add(b[idx]); // wrapping to mimic C overflow
        }
    }

    print!("{}\n", count2);
}
