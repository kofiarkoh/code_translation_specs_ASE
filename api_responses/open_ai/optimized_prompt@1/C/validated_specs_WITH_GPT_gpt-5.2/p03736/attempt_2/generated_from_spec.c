#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef int64_t i64;

#define INF64 ((i64)0x3f3f3f3f3f3f3f3fLL)

static inline i64 i64_min(i64 a, i64 b) { return a < b ? a : b; }
static inline i64 i64_abs(i64 x) { return x < 0 ? -x : x; }

/* Fenwick tree for prefix minimum on indices [0..n-1].
   We implement 0-based external indexing, internally 1-based. */
typedef struct {
    int n;
    i64 *bit; /* size n+1 */
} FenwickMin;

static void fenwick_init(FenwickMin *fw, int n) {
    fw->n = n;
    fw->bit = (i64*)malloc((size_t)(n + 1) * sizeof(i64));
    if (!fw->bit) exit(1);
    for (int i = 1; i <= n; i++) fw->bit[i] = INF64;
}

static void fenwick_free(FenwickMin *fw) {
    free(fw->bit);
    fw->bit = NULL;
    fw->n = 0;
}

/* point update: bit[idx] = min(bit[idx], val) */
static void fenwick_update_min(FenwickMin *fw, int idx0, i64 val) {
    int i = idx0 + 1;
    while (i <= fw->n) {
        if (val < fw->bit[i]) fw->bit[i] = val;
        i += i & -i;
    }
}

/* query prefix min over [0..idx0] */
static i64 fenwick_query_min(const FenwickMin *fw, int idx0) {
    i64 res = INF64;
    int i = idx0 + 1;
    while (i > 0) {
        if (fw->bit[i] < res) res = fw->bit[i];
        i -= i & -i;
    }
    return res;
}

int main(void) {
    int n;
    int q;
    if (scanf("%d %d", &n, &q) != 2) return 0;

    int cur_in, t_in;
    if (scanf("%d %d", &cur_in, &t_in) != 2) return 0;

    int cur = cur_in - 1; /* 0-based */
    int t = t_in - 1;     /* 0-based */

    /* dp is specified as initially 0 for all indices due to static zero-init.
       Use calloc to match that behavior without relying on global storage. */
    i64 *dp = (i64*)calloc((size_t)n, sizeof(i64));
    if (!dp) exit(1);

    FenwickMin fwL, fwR;
    fenwick_init(&fwL, n);
    fenwick_init(&fwR, n);

    /* Seed using target position t:
       fwL at index t with value -t  (represents dp[t]-t with dp[t]=0)
       fwR at mirrored index n-1-t with value +t (represents dp[t]+t with dp[t]=0) */
    fenwick_update_min(&fwL, t, -(i64)t);
    fenwick_update_min(&fwR, (n - 1 - t), (i64)t);

    i64 D = 0;

    for (int k = 1; k <= q; k++) {
        int next_in;
        if (scanf("%d", &next_in) != 1) break;
        int next = next_in - 1;

        i64 step = i64_abs((i64)cur - (i64)next);
        D += step;

        /* best_candidate_at_next = min over inserted pos:
             dp[pos] + |pos - next|
           Using two prefix-min structures:
             min_{pos <= next} (dp[pos] - pos) + next
             min_{pos >= next} (dp[pos] + pos) - next
           The second is obtained by mirroring indices and using prefix min. */
        i64 cand1 = fenwick_query_min(&fwL, next);
        if (cand1 != INF64) cand1 += (i64)next;

        int mir = n - 1 - next;
        i64 cand2 = fenwick_query_min(&fwR, mir);
        if (cand2 != INF64) cand2 -= (i64)next;

        i64 best = i64_min(cand1, cand2);

        /* x := best - |prev-next| */
        i64 x = best - step;

        /* Relax dp at previous position */
        if (x < dp[cur]) dp[cur] = x;

        /* Insert relaxed value into both structures at previous position */
        fenwick_update_min(&fwL, cur, dp[cur] - (i64)cur);
        fenwick_update_min(&fwR, (n - 1 - cur), dp[cur] + (i64)cur);

        cur = next;
    }

    i64 M = dp[0];
    for (int i = 1; i < n; i++) M = i64_min(M, dp[i]);

    printf("%lld\n", (long long)(M + D));

    fenwick_free(&fwL);
    fenwick_free(&fwR);
    free(dp);
    return 0;
}
