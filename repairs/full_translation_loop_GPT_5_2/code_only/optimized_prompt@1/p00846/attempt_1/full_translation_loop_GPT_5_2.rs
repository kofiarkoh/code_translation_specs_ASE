use std::io::{self, Read};

const EPS: f64 = 1e-8;

#[derive(Clone, Copy, Debug, Default)]
struct PP {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug, Default)]
struct Line {
    s: PP,
    e: PP,
}

fn dcmp(x: f64) -> i32 {
    if x.abs() < EPS {
        0
    } else if x < 0.0 {
        -1
    } else {
        1
    }
}

fn cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}

fn line_intersection(a: &Line, vs: PP, ve: PP) -> PP {
    // Matches the C formula exactly; if (a2-a1)==0, this will produce inf/NaN like C.
    let a1 = cross(
        ve.x - vs.x,
        ve.y - vs.y,
        a.s.x - vs.x,
        a.s.y - vs.y,
    );
    let a2 = cross(
        ve.x - vs.x,
        ve.y - vs.y,
        a.e.x - vs.x,
        a.e.y - vs.y,
    );
    PP {
        x: (a.s.x * a2 - a.e.x * a1) / (a2 - a1),
        y: (a.s.y * a2 - a.e.y * a1) / (a2 - a1),
    }
}

// Returns new vertex count (top), and writes a closed polygon into `po`
// with po[top] == po[0], like the C code.
fn convex_cut(u: &Line, n: usize, p: &[PP], po: &mut [PP]) -> usize {
    let mut top: usize = 0;

    for i in 0..n {
        let d1 = dcmp(cross(
            u.e.x - u.s.x,
            u.e.y - u.s.y,
            p[i].x - u.s.x,
            p[i].y - u.s.y,
        ));
        let d2 = dcmp(cross(
            u.e.x - u.s.x,
            u.e.y - u.s.y,
            p[i + 1].x - u.s.x,
            p[i + 1].y - u.s.y,
        ));

        if d1 >= 0 {
            po[top] = p[i];
            top += 1;
        }
        if d1 * d2 < 0 {
            po[top] = line_intersection(u, p[i], p[i + 1]);
            top += 1;
        }
    }

    // C writes po[top] = po[0] unconditionally; if top==0 this is UB in C.
    // We emulate the "intended" behavior by only closing when non-empty.
    if top > 0 {
        po[top] = po[0];
    }
    top
}

fn main() {
    // Read all input, parse as integers like scanf("%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: [PP; 5] = [
        PP { x: 0.0, y: 0.0 },
        PP { x: 10000.0, y: 0.0 },
        PP { x: 10000.0, y: 10000.0 },
        PP { x: 0.0, y: 10000.0 },
        PP { x: 0.0, y: 0.0 },
    ];

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n <= 0 {
            break;
        }
        let n = n as usize;

        let mut p = vec![PP::default(); n + 1];
        for i in 0..n {
            let x: i32 = it.next().unwrap().parse().unwrap();
            let y: i32 = it.next().unwrap().parse().unwrap();
            p[i] = PP {
                x: x as f64,
                y: y as f64,
            };
        }
        p[n] = p[0];

        // pol[2][100] in C; keep same capacity.
        let mut pol = [[PP::default(); 100], [PP::default(); 100]];
        // memcpy(pol[0], a, sizeof(a));
        for i in 0..5 {
            pol[0][i] = a[i];
        }
        let mut k: usize = 4;

        let mut k1: usize = 0;
        let mut k2: usize = 1;

        for i in 0..n {
            let u = Line { s: p[i], e: p[i + 1] };
            k = convex_cut(&u, k, &pol[k1], &mut pol[k2]);
            if k == 0 {
                break;
            }
            // k1 = k2; k2 = !k2;  (toggle between 0 and 1)
            k1 = k2;
            k2 = 1 - k2;
        }

        out.push_str(if k > 0 { "1\n" } else { "0\n" });
    }

    print!("{out}");
}
