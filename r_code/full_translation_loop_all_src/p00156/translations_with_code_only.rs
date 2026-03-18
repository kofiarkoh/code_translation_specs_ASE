use std::io::{self, Read};

fn main() {
    // We must mimic scanf("%d %d") and then scanf(" %c") for each cell.
    // split_whitespace() is NOT equivalent for " %c" because grid rows like "#####"
    // contain no whitespace between characters. So we implement a small byte-scanner.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut sc = Scanner { buf: input, idx: 0 };

    let mut x = vec![0i32; 200_000];
    let mut y = vec![0i32; 200_000];

    let dx = [0i32, 1, 0, -1];
    let dy = [1i32, 0, -1, 0];

    let mut out = String::new();

    loop {
        let w = match sc.next_i32() {
            Some(v) => v,
            None => break,
        };
        let h = match sc.next_i32() {
            Some(v) => v,
            None => break,
        };

        if w == 0 && h == 0 {
            break;
        }

        let mut m = vec![vec!['\0'; 101]; 101];
        let mut c = vec![vec![0i32; 101]; 101];

        // Read grid with scanf(" %c"): skip whitespace, then take one byte/char.
        // If multiple '&' exist, the last one wins (as in the C code).
        for i in 0..(h as usize) {
            for j in 0..(w as usize) {
                let ch = sc.next_non_ws_char().unwrap_or('\0');
                m[i][j] = ch;
                if m[i][j] == '&' {
                    m[i][j] = '.';
                    y[0] = i as i32;
                    x[0] = j as i32;
                    c[i][j] = 1;
                }
            }
        }

        // C: for(r=1; r-t; t++) { ... }
        let mut t: usize = 0;
        let mut r: usize = 1;

        while r != t {
            let cx = x[t];
            let cy = y[t];

            let (cxu, cyu) = (cx as usize, cy as usize);
            let cur_cost = c[cyu][cxu];

            for k in 0..4 {
                let nx = cx + dx[k];
                let ny = cy + dy[k];

                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }

                let (nxu, nyu) = (nx as usize, ny as usize);

                if m[cyu][cxu] == m[nyu][nxu] {
                    if cur_cost < c[nyu][nxu] || c[nyu][nxu] == 0 {
                        c[nyu][nxu] = cur_cost;
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                } else {
                    // C signed overflow is UB; wrapping is the closest practical emulation.
                    let next_cost = cur_cost.wrapping_add(1);
                    if next_cost < c[nyu][nxu] || c[nyu][nxu] == 0 {
                        c[nyu][nxu] = next_cost;
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                }
            }

            t += 1;
        }

        let mut minv: i32 = w;

        for i in 0..(w as usize) {
            let v1 = c[0][i] / 2;
            if v1 < minv {
                minv = v1;
            }
            let v2 = c[(h as usize) - 1][i] / 2;
            if v2 < minv {
                minv = v2;
            }
        }
        for i in 0..(h as usize) {
            let v1 = c[i][0] / 2;
            if v1 < minv {
                minv = v1;
            }
            let v2 = c[i][(w as usize) - 1] / 2;
            if v2 < minv {
                minv = v2;
            }
        }

        out.push_str(&format!("{minv}\n"));
    }

    print!("{out}");
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn skip_ws(&mut self) {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }

    fn next_i32(&mut self) -> Option<i32> {
        self.skip_ws();
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut sign = 1i32;
        if self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut val: i32 = 0;
        let mut any = false;
        while self.idx < self.buf.len() {
            let b = self.buf[self.idx];
            if !b.is_ascii_digit() {
                break;
            }
            any = true;
            val = val.wrapping_mul(10).wrapping_add((b - b'0') as i32);
            self.idx += 1;
        }
        if any { Some(val.wrapping_mul(sign)) } else { None }
    }

    fn next_non_ws_char(&mut self) -> Option<char> {
        self.skip_ws();
        if self.idx >= self.buf.len() {
            return None;
        }
        let b = self.buf[self.idx];
        self.idx += 1;
        Some(b as char) // input is ASCII in typical problems; matches C char behavior.
    }
}
