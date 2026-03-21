#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    int64_t t;
    int64_t k;
} Pair;

/* Comparator: sort by decreasing ratio t/k using cross-multiplication:
   p1 before p2 if t1*k2 > t2*k1.
   Ties are left unspecified (return 0). */
static int cmp_pairs(const void *a, const void *b) {
    const Pair *p1 = (const Pair *)a;
    const Pair *p2 = (const Pair *)b;

    /* Use 128-bit for the product to better preserve intended ordering
       without overflow in the comparator itself. The spec only states
       64-bit for ans/turn/derived values; comparator overflow behavior
       is not specified, so this is a narrow, safe choice. */
#if defined(__SIZEOF_INT128__)
    __int128 left  = (__int128)p1->t * (__int128)p2->k;
    __int128 right = (__int128)p2->t * (__int128)p1->k;
    if (left > right) return -1;
    if (left < right) return 1;
    return 0;
#else
    /* Fallback: may overflow; kept simple if __int128 is unavailable. */
    int64_t left  = p1->t * p2->k;
    int64_t right = p2->t * p1->k;
    if (left > right) return -1;
    if (left < right) return 1;
    return 0;
#endif
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* Fixed internal arrays: choose a conservative bound; spec says n must fit.
       We allocate dynamically to avoid guessing a bound while still matching
       the "indices 0..n are valid" requirement. */
    int64_t *h = (int64_t *)calloc((size_t)n + 1, sizeof(int64_t));
    int64_t *a = (int64_t *)calloc((size_t)n + 1, sizeof(int64_t));
    int64_t *d = (int64_t *)calloc((size_t)n + 1, sizeof(int64_t));
    int64_t *s = (int64_t *)calloc((size_t)n + 1, sizeof(int64_t));
    if (!h || !a || !d || !s) {
        free(h); free(a); free(d); free(s);
        return 0;
    }

    for (int i = 0; i <= n; i++) {
        if (scanf("%lld %lld %lld %lld",
                  (long long *)&h[i],
                  (long long *)&a[i],
                  (long long *)&d[i],
                  (long long *)&s[i]) != 4) {
            free(h); free(a); free(d); free(s);
            return 0;
        }
    }

    Pair *pairs = (Pair *)malloc((size_t)n * sizeof(Pair));
    if (!pairs) {
        free(h); free(a); free(d); free(s);
        return 0;
    }
    int m = 0;

    int64_t turn = 0;
    int64_t ans = 0;

    /* Phase 1 */
    for (int i = 1; i <= n; i++) {
        int64_t k = a[i] - d[0];
        if (k <= 0) continue; /* ignored entirely */

        if (a[0] <= d[i]) {
            /* Unsatisfiable condition for any relevant entity */
            printf("-1\n");
            free(pairs);
            free(h); free(a); free(d); free(s);
            return 0;
        }

        /* t = ceil((h[i] - d[i]) / (a[0] - d[i])) computed as:
           (h[i] - d[i] + (a[0] - 1)) / (a[0] - d[i]) */
        int64_t denom = a[0] - d[i];
        int64_t t = (h[i] - d[i] + (a[0] - 1)) / denom;

        turn += k;
        if (s[i] > s[0]) ans += k;

        pairs[m].t = t;
        pairs[m].k = k;
        m++;
    }

    /* Phase 2 */
    qsort(pairs, (size_t)m, sizeof(Pair), cmp_pairs);

    for (int i = 0; i < m; i++) {
        int64_t t = pairs[i].t;
        int64_t k = pairs[i].k;

        /* Intermediate arithmetic uses 64-bit signed integers; overflow unchecked. */
        ans = ans + turn * t - k;

        if (h[0] <= ans) {
            printf("-1\n");
            free(pairs);
            free(h); free(a); free(d); free(s);
            return 0;
        }

        turn -= k;
    }

    printf("%lld\n", (long long)ans);

    free(pairs);
    free(h); free(a); free(d); free(s);
    return 0;
}
