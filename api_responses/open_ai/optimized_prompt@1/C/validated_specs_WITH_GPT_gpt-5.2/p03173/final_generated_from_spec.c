#include <stdio.h>
#include <limits.h>

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) {
        return 0; /* unspecified behavior per spec; exit quietly */
    }

    int a[401];
    long long prefix[401];

    prefix[0] = 0;
    for (int i = 1; i <= N; i++) {
        if (scanf("%d", &a[i]) != 1) {
            return 0; /* unspecified behavior per spec; exit quietly */
        }
        prefix[i] = prefix[i - 1] + (long long)a[i];
    }

    if (N <= 1) {
        printf("%lld\n", 0LL);
        return 0;
    }

    /* dp[l][r] = minimum cost to merge interval [l..r] into one group */
    static long long dp[401][401];

    for (int i = 1; i <= N; i++) {
        dp[i][i] = 0;
    }

    for (int len = 2; len <= N; len++) {
        for (int l = 1; l + len - 1 <= N; l++) {
            int r = l + len - 1;
            long long sum_lr = prefix[r] - prefix[l - 1];

            long long best = LLONG_MAX;
            for (int k = l + 1; k <= r; k++) {
                /* split into [l..k-1] and [k..r] */
                long long cand = dp[l][k - 1] + dp[k][r] + sum_lr;
                if (cand < best) best = cand;
            }
            dp[l][r] = best;
        }
    }

    printf("%lld\n", dp[1][N]);
    return 0;
}
