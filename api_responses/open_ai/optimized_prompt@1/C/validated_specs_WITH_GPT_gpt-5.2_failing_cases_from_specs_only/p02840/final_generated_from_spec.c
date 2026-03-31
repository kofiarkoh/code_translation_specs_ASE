#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

typedef struct {
    int64_t v; /* residue in [0, D-1] */
    int64_t l; /* inclusive */
    int64_t r; /* exclusive */
} Interval;

static int64_t i64_abs(int64_t x) { return x < 0 ? -x : x; }

static int64_t gcd_i64(int64_t a, int64_t b) {
    a = i64_abs(a);
    b = i64_abs(b);
    while (b != 0) {
        int64_t t = a % b;
        a = b;
        b = t;
    }
    return a;
}

/* nonnegative remainder in [0, m-1], assuming m > 0 */
static int64_t mod_nonneg(int64_t a, int64_t m) {
    int64_t r = a % m;
    if (r < 0) r += m;
    return r;
}

int main(void) {
    int64_t N, X, D;
    if (scanf("%" SCNd64 " %" SCNd64 " %" SCNd64, &N, &X, &D) != 3) {
        return 0; /* unspecified behavior per spec; keep minimal */
    }

    /* 5.1 Special cases */
    if (X == 0 && D == 0) {
        printf("1\n");
        fflush(stdout);
        return 0;
    }
    if (D == 0) {
        /* distinct sums are {0, X, 2X, ..., NX} => N+1 */
        printf("%" PRId64 "\n", N + 1);
        fflush(stdout);
        return 0;
    }
    if (X == 0 && D != 0) {
        /* sums are D * (sum of chosen indices), indices 0..N-1 */
        /* distinct index-sums range 0..N(N-1)/2 */
        int64_t ans = (N * (N - 1)) / 2 + 1;
        printf("%" PRId64 "\n", ans);
        fflush(stdout);
        return 0;
    }

    /* 5.2 General case */
    {
        int64_t G = gcd_i64(X, D);
        /* G > 0 because X != 0 and D != 0 */
        X /= G;
        D /= G;

        int64_t Y;
        if (D < 0) {
            Y = X;
            X = X + D * (N - 1);
            D = -D;
        } else {
            Y = X + D * (N - 1);
        }
        /* now D > 0 */

        /* storage: at most N+1 intervals inserted (one per i) */
        Interval *arr = NULL;
        /* Use VLA-like allocation via static upper bound per precondition */
        /* N <= 200000, so N+1 <= 200001 */
        static Interval buf[200001];
        arr = buf;
        int64_t m = 0;

        /* i = 0 interval: v=0, [0, D] */
        arr[m].v = 0;
        arr[m].l = 0;
        arr[m].r = D;
        m++;

        for (int64_t i = 1; i <= N; i++) {
            /* L(i) = ((2X + D(i-1)) * i) / 2 */
            /* R(i) = ((2Y - D(i-1)) * i) / 2 + D */
            int64_t tL = (2 * X + D * (i - 1));
            int64_t L = (tL * i) / 2;

            int64_t tR = (2 * Y - D * (i - 1));
            int64_t R = (tR * i) / 2 + D;

            int64_t V = mod_nonneg(L, D);

            /* merge search: first interval satisfying same residue and touch/overlap */
            int found = 0;
            for (int64_t k = 0; k < m; k++) {
                if (arr[k].v != V) continue;
                /* predicate: p.l <= R and p.r >= L (note: uses R, not R-1) */
                if (arr[k].l <= R && arr[k].r >= L) {
                    if (L < arr[k].l) arr[k].l = L;
                    if (R > arr[k].r) arr[k].r = R;
                    found = 1;
                    break;
                }
            }
            if (!found) {
                arr[m].v = V;
                arr[m].l = L;
                arr[m].r = R;
                m++;
            }
        }

        int64_t ans = 0;
        for (int64_t k = 0; k < m; k++) {
            /* per spec: integer division */
            ans += (arr[k].r - arr[k].l) / D;
        }

        printf("%" PRId64 "\n", ans);
        fflush(stdout);
    }

    return 0;
}
