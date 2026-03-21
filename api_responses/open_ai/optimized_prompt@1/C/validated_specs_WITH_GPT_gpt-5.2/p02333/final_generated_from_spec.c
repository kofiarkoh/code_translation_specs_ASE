#include <stdio.h>
#include <stdint.h>

#define MAXM 2000
#define MOD 1000000007LL  /* fixed prime modulus, > 2000 */

static int64_t mod_norm(int64_t x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

static int64_t mod_mul(int64_t a, int64_t b) {
    /* MOD fits in 32 bits; product fits in signed 64 bits safely */
    return (a * b) % MOD;
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

int main(void) {
    long long n_in, m_in;
    if (scanf("%lld %lld", &n_in, &m_in) != 2) {
        /* Unspecified behavior per spec; exit quietly */
        return 0;
    }

    int m = (int)m_in;
    int64_t n = (int64_t)n_in;

    /* Precompute factorials and inverse factorials up to MAXM under MOD */
    static int64_t fact[MAXM + 1];
    static int64_t ifact[MAXM + 1];

    fact[0] = 1;
    for (int i = 1; i <= MAXM; i++) fact[i] = mod_mul(fact[i - 1], i);

    /* Fermat inverse since MOD is prime */
    ifact[MAXM] = mod_pow(fact[MAXM], MOD - 2);
    for (int i = MAXM; i >= 1; i--) ifact[i - 1] = mod_mul(ifact[i], i);

    /* Compute sum_{i=0..m} (-1)^{m-i} C(m,i) i^n (mod MOD) */
    int64_t ans = 0;
    for (int i = 0; i <= m; i++) {
        int64_t comb = mod_mul(fact[m], mod_mul(ifact[i], ifact[m - i]));
        int64_t term = mod_mul(comb, mod_pow(i, n));
        if (((m - i) & 1) != 0) term = mod_norm(-term);
        ans += term;
        ans %= MOD;
    }

    ans = mod_norm(ans);
    printf("%lld\n", (long long)ans);
    return 0;
}
