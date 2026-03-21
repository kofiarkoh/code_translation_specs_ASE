#include <stdio.h>

#define MOD 1000000007LL

static long long mod_pow(long long a, long long e) {
    long long r = 1 % MOD;
    a %= MOD;
    while (e > 0) {
        if (e & 1LL) r = (r * a) % MOD;
        a = (a * a) % MOD;
        e >>= 1LL;
    }
    return r;
}

int main(void) {
    long long s;
    if (scanf("%lld", &s) != 1) {
        /* Unspecified behavior per spec if parsing fails; exit quietly. */
        return 0;
    }

    /* Table: number of positive base-10 integers with exactly d digits, for d=1..8. */
    long long cnt[9] = {0};
    cnt[1] = 9;
    for (int d = 2; d <= 8; d++) cnt[d] = cnt[d - 1] * 10;

    long long ans = 0;

    /* Explicit contributions for digit-lengths up to 8:
       For each d in [1..8], count constructions where size = k*d = s, with k>0,
       and each of the k positions can be any d-digit positive integer (cnt[d] choices).
       Contribution: cnt[d]^k (mod MOD) if d divides s. */
    for (int d = 1; d <= 8; d++) {
        if (d != 0 && s % d == 0) {
            long long k = s / d;
            if (k > 0) {
                ans = (ans + mod_pow(cnt[d], k)) % MOD;
            }
        }
    }

    /* Additional contributions for larger digit-lengths handled via modular exponentiation.
       Narrowest reasonable interpretation consistent with constraints:
       For each divisor d of s with d >= 9, treat d as a digit-length and add (9*10^(d-1))^(s/d).
       This uses divisor enumeration up to sqrt(s) as specified. */
    if (s > 0) {
        for (long long i = 1; i * i <= s; i++) {
            if (s % i != 0) continue;

            long long d1 = i;
            long long d2 = s / i;

            if (d1 >= 9) {
                long long base = (9LL * mod_pow(10LL, d1 - 1)) % MOD;
                long long k = s / d1;
                if (k > 0) ans = (ans + mod_pow(base, k)) % MOD;
            }
            if (d2 != d1 && d2 >= 9) {
                long long base = (9LL * mod_pow(10LL, d2 - 1)) % MOD;
                long long k = s / d2;
                if (k > 0) ans = (ans + mod_pow(base, k)) % MOD;
            }
        }
    }

    /* Final normalization to [0, MOD-1]. */
    ans %= MOD;
    if (ans < 0) ans += MOD;

    printf("%lld\n", ans);
    return 0;
}
