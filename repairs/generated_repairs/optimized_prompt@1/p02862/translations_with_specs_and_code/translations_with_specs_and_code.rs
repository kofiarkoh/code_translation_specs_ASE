const MAX: usize = 1_000_001;
const MOD: i64 = 1_000_000_007;

struct Combinatorics {
    fac: Vec<i64>,
    finv: Vec<i64>,
    inv: Vec<i64>,
}

impl Combinatorics {
    fn new() -> Self {
        let mut fac = vec![1; MAX];
        let mut finv = vec![1; MAX];
        let mut inv = vec![0; MAX];
        
        for i in 2..MAX {
            fac[i] = fac[i - 1] * i as i64 % MOD;
            inv[i] = MOD - inv[(MOD as usize % i)] * (MOD / i as i64) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }

        Combinatorics { fac, finv, inv }
    }

    fn comb(&self, n: i64, k: i64) -> i64 {
        if n < k || n < 0 || k < 0 {
            return 0;
        }
        self.fac[n as usize] * (self.finv[k as usize] * self.finv[(n - k) as usize] % MOD) % MOD
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut coords = input.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let x = coords[0];
    let y = coords[1];

    if (x + y) % 3 != 0 {
        println!("0");
        return;
    }

    let n = (x + y) / 3;
    let x_ = x - n;
    let y_ = y - n;

    if x_ < 0 || y_ < 0 {
        println!("0");
        return;
    }

    let comb = Combinatorics::new();
    println!("{}", comb.comb(x_ + y_, x_));
}
