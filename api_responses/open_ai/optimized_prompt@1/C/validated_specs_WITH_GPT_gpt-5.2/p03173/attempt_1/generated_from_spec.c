#include <stdio.h>
#include <stdint.h>

#define MAXN 400

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) {
        return 0; /* unspecified output if input cannot be read */
    }

    int64_t a[MAXN + 1];
    int64_t prefix[MAXN + 1];
    prefix[0] = 0;

    for (int i = 1; i <= N; i++) {
        if (scanf("%" SCNd64, &a[i]) != 1) {
            return 0; /* unspecified output if input cannot be read */
        }
        prefix[i] = prefix[i - 1] + a[i];
    }

    if (N <= 1) {
        printf("0");
        return 0;
    }

    /* dp[i][j] = minimum cost to merge contiguous subsequence i..j into one group */
    static int64_t dp[MAXN + 1][MAXN + 1];

    /* Initialize base cases: dp[i][i] = 0 (already one group) */
    for (int i = 1; i <= N; i++) dp[i][i] = 0;

    /* Length from 2 to N */
    for (int len = 2; len <= N; len++) {
        for (int i = 1; i + len - 1 <= N; i++) {
            int j = i + len - 1;

            /* sum of values in i..j */
            int64_t sum = prefix[j] - prefix[i - 1];

            /* Find min over split point k: (i..k) + (k+1..j) + sum */
            int64_t best = INT64_MAX;
            for (int k = i; k < j; k++) {
                int64_t cand = dp[i][k] + dp[k + 1][j] + sum;
                if (cand < best) best = cand;
            }
            dp[i][j] = best;
        }
    }

    printf("%" PRId64, dp[1][N]);
    return 0;
}
