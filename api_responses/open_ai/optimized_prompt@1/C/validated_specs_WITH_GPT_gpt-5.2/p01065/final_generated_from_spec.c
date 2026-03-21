#include <stdio.h>

#define MAX_N 100
#define MAX_X 300
#define KMAX 600
#define BASE 300

static long long dp[MAX_N + 1][MAX_X + 1][KMAX + 1];

int main(void) {
    int n;
    int x, y;

    if (scanf("%d%d%d", &n, &x, &y) != 3) {
        return 0; /* unspecified behavior on malformed input; exit quietly */
    }

    /* Initialize all states as unreachable with a sentinel -1. */
    for (int i = 0; i <= n; i++) {
        for (int j = 0; j <= MAX_X; j++) {
            for (int k = 0; k <= KMAX; k++) {
                dp[i][j][k] = -1;
            }
        }
    }

    /* Initial state: 0 items processed, 0 resource used, balance index BASE, reward 0. */
    dp[0][0][BASE] = 0;

    for (int i = 0; i < n; i++) {
        int a, b, c, d;
        if (scanf("%d%d%d%d", &a, &b, &c, &d) != 4) {
            return 0; /* unspecified behavior on malformed input */
        }

        for (int j = 0; j <= x; j++) {
            for (int k = 0; k <= KMAX; k++) {
                long long cur = dp[i][j][k];
                if (cur < 0) continue;

                /* 1) No-op */
                if (dp[i + 1][j][k] < cur) dp[i + 1][j][k] = cur;

                /* 2) Trade: only resource bound is checked; k-b assumed in-range by precondition. */
                if (j + a <= x) {
                    int nk = k - b; /* may go out of [0..600] if inputs violate preconditions */
                    if (dp[i + 1][j + a][nk] < cur) dp[i + 1][j + a][nk] = cur;
                }

                /* 3) Reward: k+c assumed in-range by precondition. */
                {
                    int nk = k + c; /* may go out of [0..600] if inputs violate preconditions */
                    long long nv = cur + (long long)d;
                    if (dp[i + 1][j][nk] < nv) dp[i + 1][j][nk] = nv;
                }
            }
        }
    }

    long long ans = 0;
    int kup = BASE + y;
    if (kup > KMAX) kup = KMAX;
    if (kup < 0) kup = -1; /* makes loop empty if y is negative (though inputs are specified non-negative) */

    for (int j = 0; j <= x; j++) {
        for (int k = 0; k <= kup; k++) {
            long long v = dp[n][j][k];
            if (v > ans) ans = v;
        }
    }

    /* If no feasible plan yields a positive reward, output 0 (ans starts at 0). */
    printf("%lld\n", ans);
    return 0;
}
