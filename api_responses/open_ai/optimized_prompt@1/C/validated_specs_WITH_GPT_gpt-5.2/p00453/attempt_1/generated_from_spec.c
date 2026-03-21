#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_N 150
#define MAX_K 10
#define MAX_M 79

typedef struct {
    int a; /* first attribute */
    int b; /* second attribute */
} Opt;

static int iabs_int(int x) { return x < 0 ? -x : x; }

int main(void) {
    int n, m;

    while (scanf("%d %d", &n, &m) == 2) {
        if (n == 0 && m == 0) break;

        /* Read stages/options */
        Opt opts[MAX_N][MAX_K];
        int cnt[MAX_N];

        for (int s = 0; s < n; s++) {
            int k;
            if (scanf("%d", &k) != 1) return 0;
            cnt[s] = k;
            for (int j = 0; j < k; j++) {
                int a, b;
                if (scanf("%d %d", &a, &b) != 2) return 0;
                opts[s][j].a = a;
                opts[s][j].b = b;
            }
        }

        /* DP: dp[s][used_skips][opt_index] = min cost to reach that option */
        static int dp[MAX_N][MAX_M + 1][MAX_K];
        const int INF = INT_MAX / 4;

        for (int s = 0; s < n; s++)
            for (int t = 0; t <= m; t++)
                for (int j = 0; j < MAX_K; j++)
                    dp[s][t][j] = INF;

        /* Start at stage 0 with any option, cost 0 */
        for (int j = 0; j < cnt[0]; j++) dp[0][0][j] = 0;

        for (int s = 0; s < n; s++) {
            for (int used = 0; used <= m; used++) {
                for (int j = 0; j < cnt[s]; j++) {
                    int cur = dp[s][used][j];
                    if (cur >= INF) continue;

                    /* Move to s+1 without consuming skip */
                    if (s + 1 < n) {
                        for (int k = 0; k < cnt[s + 1]; k++) {
                            int cost = (opts[s][j].b + opts[s + 1][k].b) *
                                       iabs_int(opts[s][j].a - opts[s + 1][k].a);
                            int nxt = cur + cost; /* overflow not guarded per spec */
                            if (nxt < dp[s + 1][used][k]) dp[s + 1][used][k] = nxt;
                        }
                    }

                    /* Move to s+2 consuming one skip */
                    if (s + 2 < n && used + 1 <= m) {
                        for (int k = 0; k < cnt[s + 2]; k++) {
                            int cost = (opts[s][j].b + opts[s + 2][k].b) *
                                       iabs_int(opts[s][j].a - opts[s + 2][k].a);
                            int nxt = cur + cost; /* overflow not guarded per spec */
                            if (nxt < dp[s + 2][used + 1][k]) dp[s + 2][used + 1][k] = nxt;
                        }
                    }
                }
            }
        }

        /* Compute answer:
           - any path ending at stage n-1 with <= m skips
           - or ending at stage n-2 with < m skips
        */
        int ans = INF;

        if (n - 1 >= 0) {
            for (int used = 0; used <= m; used++)
                for (int j = 0; j < cnt[n - 1]; j++)
                    if (dp[n - 1][used][j] < ans) ans = dp[n - 1][used][j];
        }

        if (n - 2 >= 0) {
            for (int used = 0; used < m; used++)
                for (int j = 0; j < cnt[n - 2]; j++)
                    if (dp[n - 2][used][j] < ans) ans = dp[n - 2][used][j];
        }

        /* Per spec, behavior is unspecified if no valid path exists; we still print ans. */
        printf("%d\n", ans);
    }

    return 0;
}
