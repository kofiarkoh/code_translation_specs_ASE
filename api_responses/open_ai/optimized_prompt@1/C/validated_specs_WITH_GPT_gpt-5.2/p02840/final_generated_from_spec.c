#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef long long ll;

typedef struct {
    ll l, r; /* inclusive interval */
} Interval;

static int cmp_ll(const void *a, const void *b) {
    ll x = *(const ll *)a, y = *(const ll *)b;
    return (x < y) ? -1 : (x > y);
}

static int cmp_interval_l(const void *a, const void *b) {
    const Interval *x = (const Interval *)a;
    const Interval *y = (const Interval *)b;
    if (x->l < y->l) return -1;
    if (x->l > y->l) return 1;
    if (x->r < y->r) return -1;
    if (x->r > y->r) return 1;
    return 0;
}

int main(void) {
    ll N, X, D;
    if (scanf("%lld %lld %lld", &N, &X, &D) != 3) return 0;

    /* Special cases per specification */
    if (X == 0 && D == 0) {
        printf("1\n");
        return 0;
    }
    if (D == 0) {
        /* sums are k*X for k=0..N; distinct count is N+1 (as specified) */
        printf("%lld\n", N + 1);
        return 0;
    }
    if (X == 0 && D != 0) {
        /* distinct sums of {0, D, 2D, ..., (N-1)D} are 0..N(N-1)/2 scaled by D */
        ll ans = (N * (N - 1)) / 2 + 1;
        printf("%lld\n", ans);
        return 0;
    }

    /* Normalize D to be positive; preserve set of subset sums by reversing indices. */
    if (D < 0) {
        D = -D;
        X = X + (N - 1) * D;
    }

    /*
      General case:
      Any subset sum can be written as:
        sum = m*X + D * t
      where m = |S| in [0..N], and t is sum of chosen indices.
      For fixed m, t ranges over a contiguous interval:
        t in [m(m-1)/2, m(2N-m-1)/2]
      Thus sums for fixed m form an arithmetic progression with step D:
        m*X + D*t
      Distinctness across different m depends on residues modulo D.
      Group by r = (m*X) mod D; within each residue, count union of intervals
      in the quotient space after dividing by D.
    */

    ll absD = D;
    ll xmod = X % absD;
    if (xmod < 0) xmod += absD;

    /* Collect all possible residues r_m = (m*X) mod D for m=0..N */
    ll *res = (ll *)malloc((size_t)(N + 1) * sizeof(ll));
    if (!res) return 0;
    for (ll m = 0; m <= N; m++) {
        __int128 v = (__int128)m * (__int128)xmod;
        ll r = (ll)(v % absD);
        res[m] = r;
    }

    qsort(res, (size_t)(N + 1), sizeof(ll), cmp_ll);

    /* Unique residues */
    ll *uniq = (ll *)malloc((size_t)(N + 1) * sizeof(ll));
    if (!uniq) return 0;
    ll U = 0;
    for (ll i = 0; i <= N; i++) {
        if (i == 0 || res[i] != res[i - 1]) uniq[U++] = res[i];
    }
    free(res);

    /* For each residue, gather intervals for all m that map to it, then union them. */
    Interval *intervals = (Interval *)malloc((size_t)(N + 1) * sizeof(Interval));
    if (!intervals) return 0;

    ll ans = 0;

    for (ll ui = 0; ui < U; ui++) {
        ll r = uniq[ui];
        ll cnt = 0;

        for (ll m = 0; m <= N; m++) {
            __int128 v = (__int128)m * (__int128)xmod;
            ll rm = (ll)(v % absD);
            if (rm != r) continue;

            /* t interval for this m */
            __int128 L = (__int128)m * (m - 1) / 2;
            __int128 R = (__int128)m * (2 * N - m - 1) / 2;

            /*
              Convert sums to quotient space:
                sum = m*X + D*t
              For fixed residue r = (m*X) mod D, define base = (m*X - r)/D (integer).
              Then distinct sums correspond to integers in:
                [base + L, base + R]
            */
            __int128 mx = (__int128)m * (__int128)X;
            __int128 base = (mx - r) / absD;

            intervals[cnt].l = (ll)(base + L);
            intervals[cnt].r = (ll)(base + R);
            cnt++;
        }

        if (cnt == 0) continue;

        qsort(intervals, (size_t)cnt, sizeof(Interval), cmp_interval_l);

        /* Union intervals and add total length */
        ll curL = intervals[0].l;
        ll curR = intervals[0].r;
        for (ll i = 1; i < cnt; i++) {
            ll L = intervals[i].l;
            ll R = intervals[i].r;
            if (L > curR + 1) {
                ans += (curR - curL + 1);
                curL = L;
                curR = R;
            } else {
                if (R > curR) curR = R;
            }
        }
        ans += (curR - curL + 1);
    }

    free(intervals);
    free(uniq);

    printf("%lld\n", ans);
    return 0;
}
