#include <stdio.h>
#include <stdint.h>

#define MAXN 131072

static int64_t xs[MAXN];
static int64_t ys[MAXN];

static int parity_F(int64_t A, int64_t B, int n) {
    int res = 0;
    for (int i = 0; i < n; i++) {
        int64_t xi = xs[i], yi = ys[i];

        if (A < xi || B < yi) {
            continue; /* contributes 0 */
        }

        int64_t dx = A - xi;
        int64_t dy = B - yi;

        /* p = (A - xi) + (B - yi), q = (B - yi) */
        int64_t p = dx + dy;
        int64_t q = dy;

        /* q is a bitwise subset of p: (q & ~p) == 0 */
        if ( (q & ~p) == 0 ) {
            res ^= 1;
        }
    }
    return res;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* No explicit error handling per specification; capacity overflow is undefined. */
    for (int i = 0; i < n; i++) {
        long long xi, yi;
        if (scanf("%lld %lld", &xi, &yi) != 2) return 0;
        xs[i] = (int64_t)xi;
        ys[i] = (int64_t)yi;
    }

    const int64_t C = (int64_t)200000000000000000LL;

    /* Greedy bitwise maximization of R in [0, 2^61 - 1]. */
    int64_t R = 0;
    for (int k = 60; k >= 0; k--) {
        int64_t bit = (int64_t)1LL << k;
        int64_t R2 = R + bit;
        int64_t A2 = C - R2;
        int64_t B2 = R2;

        if (parity_F(A2, B2, n) == 1) {
            R = R2;
        }
    }

    int64_t Astar = C - R;
    printf("%lld\n", (long long)Astar);
    return 0;
}
