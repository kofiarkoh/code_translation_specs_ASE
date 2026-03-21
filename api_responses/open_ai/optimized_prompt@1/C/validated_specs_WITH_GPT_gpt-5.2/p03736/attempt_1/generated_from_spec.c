#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef long long ll;

static inline ll llmin(ll a, ll b) { return a < b ? a : b; }
static inline ll llabsll(ll x) { return x < 0 ? -x : x; }

/*
  Segment tree supporting:
    - point update: a[pos] = min(a[pos], val)
    - range query: min over [l, r]
  Used twice to represent prefix-min summaries in two transformed spaces:
    A[i] stores min(dp[i] - i)
    B[i] stores min(dp[i] + i)
  Then dp[x] = min( min_{i<=x}(dp[i]-i) + x, min_{i>=x}(dp[i]+i) - x )
*/
typedef struct {
    int n;
    ll *t;
} SegTree;

static void st_init(SegTree *st, int n) {
    st->n = 1;
    while (st->n < n) st->n <<= 1;
    st->t = (ll*)malloc((size_t)(2 * st->n) * sizeof(ll));
    for (int i = 0; i < 2 * st->n; i++) st->t[i] = (ll)4e18;
}

static void st_free(SegTree *st) {
    free(st->t);
    st->t = NULL;
    st->n = 0;
}

static void st_point_min(SegTree *st, int pos1, ll val) {
    /* pos1 is 1-based */
    int p = (pos1 - 1) + st->n;
    if (val < st->t[p]) st->t[p] = val;
    p >>= 1;
    while (p) {
        ll nv = llmin(st->t[p << 1], st->t[(p << 1) | 1]);
        if (nv == st->t[p]) break;
        st->t[p] = nv;
        p >>= 1;
    }
}

static ll st_range_min(SegTree *st, int l1, int r1) {
    if (l1 > r1) return (ll)4e18;
    int l = (l1 - 1) + st->n;
    int r = (r1 - 1) + st->n;
    ll res = (ll)4e18;
    while (l <= r) {
        if (l & 1) res = llmin(res, st->t[l++]);
        if (!(r & 1)) res = llmin(res, st->t[r--]);
        l >>= 1;
        r >>= 1;
    }
    return res;
}

int main(void) {
    int n;
    int q;
    int cur, tpos;

    if (scanf("%d %d %d %d", &n, &q, &cur, &tpos) != 4) return 0;

    SegTree A, B;
    st_init(&A, n);
    st_init(&B, n);

    /*
      Internal accumulated value dp[r] is the best "adjustment term" achievable
      for index r under the recurrence induced by processing moves.
      Initialize with dp[t] = 0, others = +inf.
    */
    st_point_min(&A, tpos, 0 - (ll)tpos);
    st_point_min(&B, tpos, 0 + (ll)tpos);

    ll D = 0;

    for (int k = 1; k <= q; k++) {
        int nxt;
        if (scanf("%d", &nxt) != 1) break;

        D += llabsll((ll)nxt - (ll)cur);

        /*
          Compute best dp at current position (before incorporating this move),
          then relax dp at the new position with movement cost |nxt-cur|.
        */
        ll bestA = st_range_min(&A, 1, cur);
        ll bestB = st_range_min(&B, cur, n);
        ll dp_cur = llmin(bestA + (ll)cur, bestB - (ll)cur);

        ll cand = dp_cur + llabsll((ll)nxt - (ll)cur);

        st_point_min(&A, nxt, cand - (ll)nxt);
        st_point_min(&B, nxt, cand + (ll)nxt);

        cur = nxt;
    }

    /* M is the minimum accumulated value over all indices after processing. */
    ll minA = st_range_min(&A, 1, n);
    ll minB = st_range_min(&B, 1, n);
    ll M = llmin(minA + 1LL, minB - 1LL); /* compute dp[1] from both sides */
    for (int r = 2; r <= n; r++) {
        ll a = st_range_min(&A, 1, r);
        ll b = st_range_min(&B, r, n);
        ll dp_r = llmin(a + (ll)r, b - (ll)r);
        if (dp_r < M) M = dp_r;
    }

    ll ans = D + M;
    printf("%lld\n", ans);

    st_free(&A);
    st_free(&B);
    return 0;
}
