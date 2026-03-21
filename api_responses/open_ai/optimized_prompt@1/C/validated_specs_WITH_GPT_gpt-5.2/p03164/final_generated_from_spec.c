#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_ITEMS 128
#define MAX_VALUE 112344
#define TABLE_SIZE (MAX_VALUE + 1)

/*
 * DP over total value:
 * dp[v] = minimum achievable total weight to get total value exactly v,
 *         or INF if not achievable within tracked bounds.
 *
 * Invariant: dp[0] == 0 always.
 */
int main(void) {
    int n;
    int W;

    if (scanf("%d", &n) != 1) return 1;
    if (scanf("%d", &W) != 1) return 1;

    /* Preconditions say n <= 128; we do not add extra behavior beyond spec. */
    int weights[MAX_ITEMS];
    int values[MAX_ITEMS];

    for (int i = 0; i < n; i++) {
        if (scanf("%d", &weights[i]) != 1) return 1;
        if (scanf("%d", &values[i]) != 1) return 1;
    }

    /* Sentinel for "not achievable". Use a large int that won't overflow on +weight under preconditions. */
    const int INF = INT_MAX / 4;

    static int dp[TABLE_SIZE];
    for (int v = 0; v <= MAX_VALUE; v++) dp[v] = INF;
    dp[0] = 0;

    for (int i = 0; i < n; i++) {
        int wi = weights[i];
        int vi = values[i];

        /* 0/1 choice: iterate values descending to avoid reusing the same item multiple times. */
        if (vi < 0) continue; /* values are nonnegative by precondition; keep narrow behavior. */
        if (vi > MAX_VALUE) continue; /* out of tracked bounds; precondition says it won't happen. */

        for (int v = MAX_VALUE - vi; v >= 0; v--) {
            int curw = dp[v];
            if (curw == INF) continue;

            int newv = v + vi;
            int neww = curw + wi; /* precondition: fits in int */
            if (neww < dp[newv]) dp[newv] = neww;
        }
    }

    int best = -1;
    for (int v = MAX_VALUE; v >= 0; v--) {
        if (dp[v] <= W) {
            best = v;
            break;
        }
    }

    if (best < 0) {
        /* Specific internal failure case per spec. */
        printf("error\n");
        return 0;
    }

    printf("%d\n", best);
    return 0;
}
