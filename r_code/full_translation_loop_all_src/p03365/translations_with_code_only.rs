use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_000_010; // matches C arrays sized 1000010

fn mod_pow(mut a: i64, mut n: i64, m: i64) -> i64 {
    // Mirrors C: ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
    let mut x: i64 = 1;
    a %= m;
    while n != 0 {
        if (n & 1) != 0 {
            x = (x * a) % m;
        }
        a = (a * a) % m;
        n /= 2;
    }
    x
}

fn choose(n: i64, r: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < 0 || r < 0 || r > n {
        return 0;
    }
    let ni = n as usize;
    let ri = r as usize;
    let n_ri = (n - r) as usize;
    (((fact[ni] * invfact[ri]) % MOD) * invfact[n_ri]) % MOD
}

fn main() {
    // Read exactly one integer like scanf("%lld",&n)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    let mut fact = vec![0i64; MAX];
    let mut invfact = vec![0i64; MAX];
    let mut temp = vec![0i64; MAX];

    fact[0] = 1;
    // C: rep(i,1,1000005) fact[i]=fact[i-1]*i%MOD;
    for i in 1..1_000_005usize {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    // C: invfact[1000000]=pom(fact[1000000],MOD-2,MOD);
    invfact[1_000_000] = mod_pow(fact[1_000_000], MOD - 2, MOD);

    // C: for(ll i=999999;i>=0;i--) invfact[i]=invfact[i+1]*(i+1)%MOD;
    for i in (0..=999_999usize).rev() {
        invfact[i] = (invfact[i + 1] * ((i as i64) + 1)) % MOD;
    }

    // C: rep(k,0,n) temp[k]=choose(k-1,n-1-k)*fact[k]%MOD*fact[n-1-k]%MOD;
    // Note: C assumes n <= 1_000_000 (fits arrays). We preserve that assumption.
    for k in 0..(n as usize) {
        let kk = k as i64;
        let val = choose(kk - 1, n - 1 - kk, &fact, &invfact);
        let mut t = (val * fact[k]) % MOD;
        t = (t * fact[(n as usize) - 1 - k]) % MOD;
        temp[k] = t;
    }

    // C: ll ans=0; rep(k,1,n) ans=(ans+k*(temp[k]-temp[k-1]+MOD))%MOD;
    let mut ans: i64 = 0;
    for k in 1..(n as usize) {
        let kk = k as i64;
        let diff = (temp[k] - temp[k - 1] + MOD) % MOD;
        ans = (ans + (kk * diff) % MOD) % MOD;
    }

    // C: printf("%lld\n",ans);
    println!("{}", ans);
}
