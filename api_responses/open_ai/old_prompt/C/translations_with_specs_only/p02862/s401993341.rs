const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_000_000;

struct Combinatorics {
    fac: Vec<i64>,
    finv: Vec<i64>,
    inv: Vec<i64>,
}

impl Combinatorics {
    fn new() -> Self {
        let mut fac = vec![1; MAX + 1];
        let mut finv = vec![1; MAX + 1];
        let mut inv = vec![1; MAX + 1];

        for i in 2..=MAX {
            fac[i] = fac[i - 1] * i as i64 % MOD;
        }

        finv[MAX] = mod_inverse(fac[MAX]);
        for i in (1..MAX).rev() {
            finv[i] = finv[i + 1] * (i + 1) as i64 % MOD;
        }

        for i in 1..=MAX {
            inv[i] = finv[i] * fac[i - 1] % MOD;
        }

        Combinatorics { fac, finv, inv }
    }

    fn combination(&self, n: usize, k: usize) -> i64 {
        if k > n {
            return 0;
        }
        self.fac[n] * self.finv[k] % MOD * self.finv[n - k] % MOD
    }
}

fn mod_inverse(a: i64) -> i64 {
    let mut m = MOD;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut a = a % m;

    while a > 1 {
        let q = a / m;
        let t = m;
        m = a % m;
        a = t;
        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += MOD;
    }
    x1
}

fn unique_paths(x: i64, y: i64) -> i64 {
    if x < 0 || y < 0 || (x + y) % 3 != 0 {
        return 0;
    }

    let total_moves = x + y;
    let right_moves = (2 * x - total_moves / 3) as usize;
    let down_moves = (2 * y - total_moves / 3) as usize;

    if right_moves < 0 || down_moves < 0 {
        return 0;
    }

    let comb = Combinatorics::new();
    comb.combination(right_moves + down_moves, right_moves)
}

fn main() {
    let x: i64 = 3; // Example input for right movements
    let y: i64 = 3; // Example input for down movements

    let result = unique_paths(x, y);
    println!("{}", result);
}
