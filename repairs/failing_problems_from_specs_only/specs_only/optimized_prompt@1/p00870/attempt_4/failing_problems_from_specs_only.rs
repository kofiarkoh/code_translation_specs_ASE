use std::io::{self, Read};

fn main() {
    // Use a byte-based scanner to avoid any UTF-8 assumptions and to more closely match
    // C-style token reading. Tokens are delimited by ASCII whitespace (<= ' ').
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data).unwrap();
    let mut sc = Scanner::new(&data);

    let mut out = String::new();
    let mut first = true;

    loop {
        let n = match sc.next_usize() {
            None => break,
            Some(v) => v,
        };
        let m = match sc.next_usize() {
            None => break,
            Some(v) => v,
        };

        if n == 0 && m == 0 {
            break;
        }

        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            patterns.push(sc.next_token().unwrap_or_default());
        }

        let mut text: Vec<u8> = Vec::new();
        for _ in 0..m {
            let mut frag = sc.next_token().unwrap_or_default();
            // Spec rule: if last character has ASCII code < ' ', remove it.
            if let Some(&last) = frag.last() {
                if last < b' ' {
                    frag.pop();
                }
            }
            text.extend_from_slice(&frag);
        }

        let l = text.len();
        let mask_count = 1usize << n;
        let full_mask = if n == 0 { 0 } else { mask_count - 1 };

        // dp[pos][mask] reachability, flattened.
        let mut dp = vec![false; (l + 1) * mask_count];
        dp[0] = true; // (0, empty) reachable

        // Forward propagation scanning positions increasing.
        for p in 0..l {
            let base = p * mask_count;
            for s in 0..mask_count {
                if !dp[base + s] {
                    continue;
                }
                for k in 0..n {
                    let bit = 1usize << k;
                    if (s & bit) != 0 {
                        continue;
                    }
                    let pat = &patterns[k];
                    let lenk = pat.len();
                    if p + lenk > l {
                        continue;
                    }
                    if &text[p..p + lenk] == &pat[..] {
                        let np = p + lenk;
                        let ns = s | bit;
                        dp[np * mask_count + ns] = true;
                    }
                }
            }
        }

        let mut count = 0usize;
        for p in 0..=l {
            if dp[p * mask_count + full_mask] {
                count += 1;
            }
        }

        if !first {
            out.push('\n');
        }
        first = false;
        out.push_str(&count.to_string());
    }

    print!("{out}");
}

struct Scanner<'a> {
    data: &'a [u8],
    i: usize,
}

impl<'a> Scanner<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, i: 0 }
    }

    fn is_ws(b: u8) -> bool {
        b <= b' '
    }

    fn skip_ws(&mut self) {
        while self.i < self.data.len() && Self::is_ws(self.data[self.i]) {
            self.i += 1;
        }
    }

    fn next_token(&mut self) -> Option<Vec<u8>> {
        self.skip_ws();
        if self.i >= self.data.len() {
            return None;
        }
        let start = self.i;
        while self.i < self.data.len() && !Self::is_ws(self.data[self.i]) {
            self.i += 1;
        }
        Some(self.data[start..self.i].to_vec())
    }

    fn next_usize(&mut self) -> Option<usize> {
        let tok = self.next_token()?;
        // Dataset headers are integers in ASCII.
        let s = std::str::from_utf8(&tok).ok()?;
        s.parse::<usize>().ok()
    }
}
