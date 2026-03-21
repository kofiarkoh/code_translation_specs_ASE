#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    int64_t x; /* derived "turn" contribution: a_i - d0 */
    int64_t y; /* derived "score" contribution: d_i */
} Pair;

static int cmp_pair(const void *pa, const void *pb) {
    const Pair *a = (const Pair *)pa;
    const Pair *b = (const Pair *)pb;
    /* Order by decreasing ratio y/x without floating point:
       a before b if a.y/a.x > b.y/b.x  <=>  a.y*b.x > b.y*a.x
       x are positive for all stored pairs by construction. */
    __int128 lhs = (__int128)a->y * (__int128)b->x;
    __int128 rhs = (__int128)b->y * (__int128)a->x;
    if (lhs > rhs) return -1;
    if (lhs < rhs) return 1;
    return 0;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* Fixed internal capacity implied by spec: n <= 40001, so n+1 <= 40002. */
    int64_t t0, a0, d0, s0;
    if (scanf("%lld %lld %lld %lld", (long long *)&t0, (long long *)&a0, (long long *)&d0, (long long *)&s0) != 4)
        return 0;

    Pair *pairs = (Pair *)malloc((size_t)(n > 0 ? n : 1) * sizeof(Pair));
    if (!pairs) return 0;

    int m = 0;
    int64_t sum_turn = 0; /* sum of x over relevant entities */

    for (int i = 1; i <= n; i++) {
        int64_t ti, ai, di, si;
        if (scanf("%lld %lld %lld %lld", (long long *)&ti, (long long *)&ai, (long long *)&di, (long long *)&si) != 4) {
            free(pairs);
            return 0;
        }

        /* Relevant iff ai > d0; only those affect result. */
        if (ai > d0) {
            /* Unsatisfiable if a0 <= di for any relevant entity. */
            if (a0 <= di) {
                printf("-1\n");
                free(pairs);
                return 0;
            }
            int64_t x = ai - d0; /* positive */
            int64_t y = di;
            pairs[m].x = x;
            pairs[m].y = y;
            m++;
            sum_turn += x;
        }
    }

    if (m > 1) qsort(pairs, (size_t)m, sizeof(Pair), cmp_pair);

    int64_t ans = 0;
    int64_t rem = sum_turn;

    for (int i = 0; i < m; i++) {
        int64_t x = pairs[i].x;
        int64_t y = pairs[i].y;

        /* Update accumulated amount:
           ans += y * rem / (a0 - y) + 1
           with integer division truncating toward zero (all terms nonnegative). */
        int64_t denom = a0 - y; /* positive due to earlier unsat check */
        __int128 prod = (__int128)y * (__int128)rem;
        int64_t add = (int64_t)(prod / denom) + 1;
        ans += add;

        /* Failure if accumulated reaches/exceeds distinguished limit t0. */
        if (ans >= t0) {
            printf("-1\n");
            free(pairs);
            return 0;
        }

        rem -= x;
        if (rem < 0) rem = 0; /* defensive; spec implies nonnegative invariant */
    }

    /* Postcondition requires ans < t0 already ensured. */
    printf("%lld\n", (long long)ans);

    free(pairs);
    return 0;
}
