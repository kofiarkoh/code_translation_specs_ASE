#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <limits.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    if (n <= 1) {
        /* Per specification: if n = 1, output 0. (Also covers n <= 0 if input violates preconditions.) */
        printf("0\n");
        return 0;
    }

    /* 1-indexed arrays for convenience with prefix sums and DP ranges. */
    long long *w = (long long *)malloc((size_t)(n + 1) * sizeof(long long));
    long long *pref = (long long *)malloc((size_t)(n + 1) * sizeof(long long));
    if (!w || !pref) return 0; /* Unspecified behavior on resource failure; exit quietly. */

    pref[0] = 0;
    for (int i = 1; i <= n; i++) {
        if (scanf("%lld", &w[i]) != 1) return 0;
        pref[i] = pref[i - 1] + w[i];
    }

    /* dp[i][j] = minimum cost to merge items i..j into one group (1 <= i <= j <= n).
       opt[i][j] = argmin k for dp[i][j] split (Knuth optimization).
       Memory proportional to n^2 as required. */
    size_t N = (size_t)(n + 1);
    long long *dp = (long long *)malloc(N * N * sizeof(long long));
    int *opt = (int *)malloc(N * N * sizeof(int));
    if (!dp || !opt) return 0;

    /* Helper macro for 2D indexing in flat arrays. */
    #define IDX(i,j) ((size_t)(i) * N + (size_t)(j))

    for (int i = 1; i <= n; i++) {
        dp[IDX(i, i)] = 0;
        opt[IDX(i, i)] = i;
    }

    /* sum(i,j) = pref[j] - pref[i-1] */
    for (int len = 2; len <= n; len++) {
        for (int i = 1; i + len - 1 <= n; i++) {
            int j = i + len - 1;
            long long s = pref[j] - pref[i - 1];

            /* Knuth optimization:
               opt[i][j-1] <= opt[i][j] <= opt[i+1][j]
               This is intended by the specification's note about exploiting optimal partition structure. */
            int left = opt[IDX(i, j - 1)];
            int right = opt[IDX(i + 1, j)];
            if (left < i) left = i;
            if (right > j - 1) right = j - 1;
            if (left > right) { /* Fallback to full range if bounds are inconsistent. */
                left = i;
                right = j - 1;
            }

            long long best = LLONG_MAX;
            int bestk = left;
            for (int k = left; k <= right; k++) {
                long long cand = dp[IDX(i, k)] + dp[IDX(k + 1, j)] + s;
                if (cand < best) {
                    best = cand;
                    bestk = k;
                }
            }
            dp[IDX(i, j)] = best;
            opt[IDX(i, j)] = bestk;
        }
    }

    printf("%lld\n", dp[IDX(1, n)]);

    #undef IDX
    free(dp);
    free(opt);
    free(w);
    free(pref);
    return 0;
}
