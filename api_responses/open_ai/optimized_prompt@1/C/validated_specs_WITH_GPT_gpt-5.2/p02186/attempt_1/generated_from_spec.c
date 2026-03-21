#include <stdio.h>
#include <stdint.h>

#define CAP 131072 /* capacity for prefix-sum positions: n+1 <= 131072 */

typedef struct {
    int64_t mn;   /* minimum prefix sum in segment */
    int64_t mx;   /* maximum prefix sum in segment */
    int64_t best; /* maximum difference P[b]-P[a] with a<b within segment */
} Node;

static Node seg[4 * (CAP + 5)];
static int64_t P[CAP + 1]; /* prefix sums P[0..n] */
static int n, q;

static inline int64_t max64(int64_t a, int64_t b) { return a > b ? a : b; }
static inline int64_t min64(int64_t a, int64_t b) { return a < b ? a : b; }

static Node merge(Node L, Node R) {
    Node res;
    res.mn = min64(L.mn, R.mn);
    res.mx = max64(L.mx, R.mx);
    /* best either entirely in left, entirely in right, or crossing: max(R.mx - L.mn) */
    res.best = max64(max64(L.best, R.best), R.mx - L.mn);
    return res;
}

static void build(int v, int tl, int tr) {
    if (tl == tr) {
        int64_t x = P[tl];
        seg[v].mn = x;
        seg[v].mx = x;
        seg[v].best = 0; /* no pair a<b inside a single point */
        return;
    }
    int tm = (tl + tr) >> 1;
    build(v << 1, tl, tm);
    build(v << 1 | 1, tm + 1, tr);
    seg[v] = merge(seg[v << 1], seg[v << 1 | 1]);
}

static void point_set(int v, int tl, int tr, int pos, int64_t val) {
    if (tl == tr) {
        seg[v].mn = val;
        seg[v].mx = val;
        seg[v].best = 0;
        return;
    }
    int tm = (tl + tr) >> 1;
    if (pos <= tm) point_set(v << 1, tl, tm, pos, val);
    else point_set(v << 1 | 1, tm + 1, tr, pos, val);
    seg[v] = merge(seg[v << 1], seg[v << 1 | 1]);
}

/* range add with lazy propagation: add delta to all P[k] for k in [l..r] */
static int64_t lazy[4 * (CAP + 5)];

static void apply_add(int v, int64_t delta) {
    seg[v].mn += delta;
    seg[v].mx += delta;
    /* seg[v].best unchanged by uniform shift */
    lazy[v] += delta;
}

static void push(int v) {
    if (lazy[v] != 0) {
        apply_add(v << 1, lazy[v]);
        apply_add(v << 1 | 1, lazy[v]);
        lazy[v] = 0;
    }
}

static void range_add(int v, int tl, int tr, int l, int r, int64_t delta) {
    if (l > r) return;
    if (l == tl && r == tr) {
        apply_add(v, delta);
        return;
    }
    push(v);
    int tm = (tl + tr) >> 1;
    range_add(v << 1, tl, tm, l, (r < tm ? r : tm), delta);
    range_add(v << 1 | 1, tm + 1, tr, (l > tm + 1 ? l : tm + 1), r, delta);
    seg[v] = merge(seg[v << 1], seg[v << 1 | 1]);
}

int main(void) {
    if (scanf("%d %d", &n, &q) != 2) return 0;

    /* Build initial prefix sums P[0..n] */
    P[0] = 0;
    int64_t cur = 0;
    static int64_t A[CAP + 1]; /* 1-based underlying array values */
    for (int i = 1; i <= n; i++) {
        int64_t x;
        scanf("%lld", (long long*)&x);
        A[i] = x;
        cur += x;
        P[i] = cur;
    }

    /* Build segment tree over indices [0..n] */
    for (int i = 0; i < (int)(4 * (CAP + 5)); i++) lazy[i] = 0;
    build(1, 0, n);

    /* Output initial maximum subarray sum (empty allowed => at least 0) */
    printf("%lld\n", (long long)max64(0, seg[1].best));

    for (int qi = 0; qi < q; qi++) {
        int idx;
        int64_t t;
        scanf("%d %lld", &idx, (long long*)&t);

        /* Update underlying array at position idx to t:
           delta affects all prefix sums P[k] for k >= idx by +delta. */
        int64_t delta = t - A[idx];
        A[idx] = t;

        if (delta != 0) {
            range_add(1, 0, n, idx, n, delta);
        }

        printf("%lld\n", (long long)max64(0, seg[1].best));
    }

    return 0;
}
