use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Info {
    no: i32,
    score: i32,
}

/// Token iterator over all whitespace-separated integers from stdin.
/// We parse them all up front to preserve the exact sequential consumption
/// semantics of the original C program (which uses scanf).
struct Tokens {
    data: Vec<i32>,
    pos: usize,
}

impl Tokens {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).expect("failed to read stdin");
        let data = s
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().expect("failed to parse integer"))
            .collect::<Vec<_>>();
        Tokens { data, pos: 0 }
    }

    /// Get next integer, analogous to a scanf("%d", &x).
    /// We assume input is well-formed and contains enough integers,
    /// matching the assumption in the C code (which does not check scanf failures).
    fn next(&mut self) -> Option<i32> {
        if self.pos < self.data.len() {
            let v = self.data[self.pos];
            self.pos += 1;
            Some(v)
        } else {
            None
        }
    }
}

/// Translate of the C scoring() function preserving its control flow and integer semantics.
fn scoring(tokens: &mut Tokens) -> i32 {
    // All variables are i32 to mirror C's `int`.
    let mut total: i32 = 0;
    let mut sum: i32 = 0;
    let mut frame: i32 = 1;
    let mut time: i32 = 1;
    let mut flg_spare: i32 = 0;
    let mut flg_strike: i32 = 0;

    // Frames 1..9
    while frame < 10 {
        let x = tokens.next().expect("unexpected EOF while reading roll");
        sum += x;

        if flg_spare != 0 {
            total += x;
            flg_spare = 0;
        } else if flg_strike != 0 {
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }

        if time == 2 {
            total += sum;
            if sum == 10 {
                flg_spare = 1;
            }
            sum = 0;
            time = 1;
            frame += 1;
        } else if time == 1 {
            time += 1;
            if x == 10 {
                total += 10;
                flg_strike += 2;
                sum = 0;
                time = 1;
                frame += 1;
            }
        }
    }

    // 10th frame and possible bonus rolls
    for i in 0..3 {
        // In C: if(i == 2 && sum < 10) break;
        if i == 2 && sum < 10 {
            break;
        }

        let x = tokens.next().expect("unexpected EOF while reading roll");
        total += x;
        sum += x;

        if flg_spare != 0 {
            total += x;
            flg_spare = 0;
        } else if flg_strike != 0 {
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }
    }

    total
}

fn main() {
    let mut tokens = Tokens::new();

    // Emulate: while (scanf("%d",&n) && n) { ... }
    loop {
        let n = match tokens.next() {
            Some(v) => v,
            None => break, // EOF -> stop
        };
        if n == 0 {
            break;
        }

        let mut infos: Vec<Info> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let no = tokens.next().expect("unexpected EOF while reading player no");
            let score = scoring(&mut tokens);
            infos.push(Info { no, score });
        }

        // sortInfo: primary by score descending, secondary by no ascending
        infos.sort_by(|a, b| {
            if a.score != b.score {
                b.score.cmp(&a.score) // descending by score
            } else {
                a.no.cmp(&b.no) // ascending by no
            }
        });

        for info in infos {
            // Preserve exact output formatting: "%d %d\n"
            println!("{} {}", info.no, info.score);
        }
    }
}
