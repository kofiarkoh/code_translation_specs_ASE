#include <stdio.h>
#include <stdint.h>

#define MAXN 1000000

/* Fixed prime modulus (implied by spec but not numerically given).
   Choose a standard large prime suitable for factorial inverses up to 1e6. */
static const int64_t MOD = 1000000007LL;

static int64_t fact[MAXN + 1];
static int64_t invfact[MAXN + 1];

static int64_t mod_norm(int64_t x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

static int64_t mod_mul(int64_t a, int64_t b) {
    return (a % MOD) * (b % MOD) % MOD;
}

static int64_t mod_pow(int64_t a, int64_t e) {
    int64_t r = 1 % MOD;
    a = mod_norm(a);
    while (e > 0) {
        if (e & 1) r = mod_mul(r, a);
        a = mod_mul(a, a);
        e >>= 1;
    }
    return r;
}

static void precompute_tables(void) {
    fact[0] = 1;
    for (int i = 1; i <= MAXN; i++) fact[i] = mod_mul(fact[i - 1], i);

    /* MOD is prime; invfact[MAXN] = fact[MAXN]^(MOD-2) */
    invfact[MAXN] = mod_pow(fact[MAXN], MOD - 2);
    for (int i = MAXN; i >= 1; i--) invfact[i - 1] = mod_mul(invfact[i], i);
}

/* Binomial coefficient C(a,b) modulo MOD with the spec's zero conditions. */
static int64_t nCk_mod(int a, int b) {
    if (a < 0 || b < 0 || b > a) return 0;
    /* Preconditions guarantee a <= 1e6 in supported use. */
    return mod_mul(fact[a], mod_mul(invfact[b], invfact[a - b]));
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Undefined behavior per spec on parse failure; do nothing special. */
        return 0;
    }

    precompute_tables();

    /* Compute ans = sum_{k=1}^{n-1} k * (T(k) - T(k-1)) mod MOD
       with T(k) = C(k-1, n-1-k) * k! * (n-1-k)! mod MOD. */
    int64_t ans = 0;
    int64_t T_prev = 0; /* T(0) is defined by formula; C(-1, n-1) => 0 */
    for (int k = 1; k <= n - 1; k++) {
        int a = k - 1;
        int b = (n - 1) - k;

        int64_t comb = nCk_mod(a, b);
        int64_t Tk = comb;
        /* Multiply by k! and (n-1-k)!; if b<0 then comb=0 already. */
        if (b >= 0) {
            Tk = mod_mul(Tk, fact[k]);
            Tk = mod_mul(Tk, fact[b]);
        } else {
            Tk = 0;
        }

        int64_t diff = Tk - T_prev;
        diff = mod_norm(diff);

        ans = (ans + mod_mul(k, diff)) % MOD;

        T_prev = Tk;
    }

    ans = mod_norm(ans);
    printf("%lld\n", (long long)ans);
    return 0;
}
