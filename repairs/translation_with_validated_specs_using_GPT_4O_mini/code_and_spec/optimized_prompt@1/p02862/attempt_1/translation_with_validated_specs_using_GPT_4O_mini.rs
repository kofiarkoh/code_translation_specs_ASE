const MAX: usize = 1_000_001;
const MOD: i64 = 1_000_000_007;

struct Combinatorics {
    fac: Vec<i64>,
    finv: Vec<i64>,
    inv: Vec<i64>,
}

impl Combinatorics {
    fn new() -> Self {
        let mut fac = vec![0; MAX];
        let mut finv = vec![0; MAX];
        let mut inv = vec![0; MAX];

        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;

        for i in 2..MAX {
            fac[i] = fac[i - 1] * i as i64 % MOD;
            inv[i] = MOD - inv[(MOD % i as i64) as usize] * (MOD / i as i64) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }

        Combinatorics { fac, finv, inv }
    }

    fn com(&self, n: i64, k: i64) -> i64 {
        if n < k || n < 0 || k < 0 {
            return 0;
        }
        self.fac[n as usize] * (self.finv[k as usize] * self.finv[(n - k) as usize] % MOD) % MOD
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: i64 = iter.next().unwrap().parse().unwrap();
    let y: i64 = iter.next().unwrap().parse().unwrap();

    if x < 0 || y < 0 {
        println!("0");
        return;
    }

    let sum = x + y;
    if sum % 3 != 0 {
        println!("0");
        return;
    }

    let n = sum / 3;
    let x_ = x - n;
    let y_ = y - n;

    if x_ < 0 || y_ < 0 {
        println!("0");
        return;
    }

    let comb = Combinatorics::new();
    println!("{}", comb.com(x_ + y_, x_));
}
