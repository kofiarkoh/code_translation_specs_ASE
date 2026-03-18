use std::io::{self, Read};

fn main() {
    // The sample input contains multiple integers; process all tokens.
    // Undefined behavior on invalid integer per spec, so unwrap parsing.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let mut outs: Vec<String> = Vec::new();
    for tok in s.split_whitespace() {
        let n: i64 = tok.parse().unwrap();

        // Semantics inferred from spec + observed sample:
        // output depends on divisibility by 11 and remainder when divided by 11.
        // For n = 25, expected 5 matches: (n / 11) + (n % 11) = 2 + 3 = 5.
        // Use Euclidean division/remainder to keep remainder non-negative.
        let q = n.div_euclid(11);
        let r = n.rem_euclid(11);
        let out = q + r;

        outs.push(out.to_string());
    }

    print!("{}", outs.join("\n"));
}
