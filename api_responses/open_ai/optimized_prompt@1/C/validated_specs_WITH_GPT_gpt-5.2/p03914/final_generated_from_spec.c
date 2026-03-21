#include <stdio.h>
#include <stdint.h>

#define MAXN 305
#define MOD 1000000007LL

int main(void) {
    long long size, steps;
    if (scanf("%lld %lld", &size, &steps) != 2) {
        return 0; /* No explicit error handling per specification */
    }

    /* Fixed-size DP tables; behavior is undefined if inputs exceed supported bounds. */
    static long long dp[MAXN][MAXN];
    static long long ndp[MAXN][MAXN];

    /* Initialization: state (1,1) = 1, others 0. */
    dp[1][1] = 1;

    for (long long i = 0; i < steps; i++) {
        /* Clear next table. */
        for (int a = 0; a < MAXN; a++) {
            for (int b = 0; b < MAXN; b++) {
                ndp[a][b] = 0;
            }
        }

        /* Only states with 1 <= b <= a and a up to i+1 can be non-zero (invariant). */
        long long amax = i + 1;
        if (amax >= MAXN) amax = MAXN - 1;

        for (long long a = 1; a <= amax; a++) {
            for (long long b = 1; b <= a; b++) {
                long long cur = dp[a][b] % MOD;
                if (cur == 0) continue;

                /* Transition 1: (a+1, b) with multiplicity (size - a). */
                {
                    long long mult = (size - a);
                    long long add = (cur * (mult % MOD)) % MOD;
                    /* No bounds checks per specification; relies on preconditions. */
                    ndp[a + 1][b] = (ndp[a + 1][b] + add) % MOD;
                }

                /* Transition 2: (a, a) with multiplicity b. */
                {
                    long long mult = b;
                    long long add = (cur * (mult % MOD)) % MOD;
                    ndp[a][a] = (ndp[a][a] + add) % MOD;
                }

                /* Transition 3: (a, b) with multiplicity (a - b). */
                {
                    long long mult = (a - b);
                    long long add = (cur * (mult % MOD)) % MOD;
                    ndp[a][b] = (ndp[a][b] + add) % MOD;
                }
            }
        }

        /* Swap dp and ndp. */
        for (int a = 0; a < MAXN; a++) {
            for (int b = 0; b < MAXN; b++) {
                dp[a][b] = ndp[a][b];
            }
        }
    }

    /* Output: state (size, size) after exactly 'steps' steps, modulo MOD. */
    /* If unreachable, it remains 0. */
    long long ans = dp[size][size] % MOD;
    if (ans < 0) ans += MOD;
    printf("%lld\n", ans);

    return 0;
}
