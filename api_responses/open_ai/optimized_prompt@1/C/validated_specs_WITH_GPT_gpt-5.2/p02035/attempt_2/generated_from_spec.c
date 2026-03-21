#include <stdio.h>
#include <stdint.h>

#define MAXN 2010

static int64_t mod_pow(int64_t a, int64_t e, int64_t mod) {
    if (mod == 1) return 0;
    a %= mod;
    if (a < 0) a += mod;
    int64_t r = 1 % mod;
    while (e > 0) {
        if (e & 1) r = (r * a) % mod;
        a = (a * a) % mod;
        e >>= 1;
    }
    return r;
}

int main(void) {
    int n, m_in;
    if (scanf("%d %d", &n, &m_in) != 2) return 0;

    int64_t m = (int64_t)m_in;

    /* All arithmetic is "modulo m" as per spec; if m==0, modulo is undefined.
       The spec provides no error handling; we avoid crashing and print 0. */
    if (m == 0) {
        printf("0\n");
        return 0;
    }

    /* Precompute factorials and inverse factorials up to MAXN inclusive. */
    static int64_t fact[MAXN + 1];
    static int64_t invfact[MAXN + 1];

    fact[0] = 1 % m;
    for (int i = 1; i <= MAXN; i++) fact[i] = (fact[i - 1] * i) % m;

    /* Modular inverse via exponentiation to m-2, as specified. */
    invfact[MAXN] = mod_pow(fact[MAXN], m - 2, m);
    for (int i = MAXN; i >= 1; i--) invfact[i - 1] = (invfact[i] * i) % m;

    /* Binomial coefficient modulo m with out-of-range returning 0. */
    auto int64_t C(int a, int b) {
        if (a < 0 || b < 0 || b > a) return 0;
        if (a > MAXN) {
            /* Spec: no validation; behavior not guaranteed. Narrow choice: return 0. */
            return 0;
        }
        return (((fact[a] * invfact[b]) % m) * invfact[a - b]) % m;
    }

    int64_t ans = 0;

    for (int i = 1; i <= n - 1; i++) {
        int ni = n - i;

        int64_t term = C(n, i);

        /* Multiply by 2^{i(i-1)/2} and 2^{(n-i)(n-i-1)/2}. */
        int64_t e1 = (int64_t)i * (i - 1) / 2;
        int64_t e2 = (int64_t)ni * (ni - 1) / 2;
        term = (term * mod_pow(2, e1, m)) % m;
        term = (term * mod_pow(2, e2, m)) % m;

        /* Inclusion-exclusion sum over j = 0..(n-i-1). */
        int64_t inner = 0;
        for (int j = 0; j < ni; j++) {
            int64_t comb = C(ni, j);
            int64_t base = (mod_pow(2, (int64_t)(ni - j), m) - 1) % m;
            if (base < 0) base += m;
            int64_t pw = mod_pow(base, (int64_t)i, m);
            int64_t add = (comb * pw) % m;
            if (j & 1) add = (m - add) % m; /* (-1)^j */
            inner += add;
            inner %= m;
        }

        term = (term * inner) % m;
        ans += term;
        ans %= m;
    }

    ans %= m;
    if (ans < 0) ans += m;
    printf("%lld\n", (long long)ans);
    return 0;
}
