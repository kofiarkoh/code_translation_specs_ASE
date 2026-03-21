#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

static int cmp_ll(const void *p, const void *q) {
    long long a = *(const long long *)p;
    long long b = *(const long long *)q;
    if (a < b) return -1;
    if (a > b) return 1;
    return 0;
}

static long long llabs64(long long x) {
    /* Specification assumes no overflow in intermediates; abs(LLONG_MIN) not addressed. */
    return x < 0 ? -x : x;
}

/*
Mixed-sign recursive construction.

Returns the single value produced by reducing the subarray a[f..b] (inclusive)
under the deterministic strategy, while printing exactly (b-f) operations.

Parameter s is the alternating state:
- s == 0: use (a[f], a[b]) as printed order for the "extremes" operation
- s == 1: use (a[b], a[f]) as printed order for the "extremes" operation

l is the last index with a[l] < 0 and a[l+1] >= 0.
*/
static long long solve_mixed(long long *a, int f, int b, int l, int s) {
    if (f == l) {
        /* Base case: left end reaches last negative. */
        long long x = a[l];
        for (int i = l + 1; i <= b - 1; i++) {
            printf("%lld %lld\n", x, a[i]);
            x = x - a[i];
        }
        printf("%lld %lld\n", a[b], x);
        return a[b] - x;
    }
    if (b == l + 1) {
        /* Base case: right end is first non-negative. */
        long long x = a[l + 1];
        for (int i = l; i >= f + 1; i--) {
            printf("%lld %lld\n", x, a[i]);
            x = x - a[i];
        }
        printf("%lld %lld\n", a[f], x);
        return a[f] - x;
    }

    long long d;
    if (s == 0) {
        printf("%lld %lld\n", a[f], a[b]);
        d = a[f] - a[b];
    } else {
        printf("%lld %lld\n", a[b], a[f]);
        d = a[b] - a[f];
    }

    long long r = solve_mixed(a, f + 1, b - 1, l, 1 - s);

    printf("%lld %lld\n", d, r);
    return d - r;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    long long *a = (long long *)malloc((size_t)n * sizeof(long long));
    if (!a) return 0;

    for (int i = 0; i < n; i++) {
        (void)scanf("%lld", &a[i]);
    }

    qsort(a, (size_t)n, sizeof(long long), cmp_ll);

    long long S = 0;
    for (int i = 0; i < n; i++) S += llabs64(a[i]);

    if (a[0] >= 0) {
        /* All non-negative */
        long long ans = S - 2LL * a[0];
        printf("%lld\n", ans);

        long long x = a[0];
        for (int i = 1; i <= n - 2; i++) {
            printf("%lld %lld\n", x, a[i]);
            x = x - a[i];
        }
        printf("%lld %lld\n", a[n - 1], x);
    } else if (a[n - 1] <= 0) {
        /* All non-positive */
        long long ans = S + 2LL * a[n - 1];
        printf("%lld\n", ans);

        long long x = a[n - 1];
        for (int i = 0; i <= n - 2; i++) {
            printf("%lld %lld\n", x, a[i]);
            x = x - a[i];
        }
    } else {
        /* Mixed signs */
        long long ans = S;
        printf("%lld\n", ans);

        /* Find l: last index with a[l] < 0 and a[l+1] >= 0 via binary-search-like loop. */
        int lo = 0, hi = n - 1;
        while (hi - lo > 1) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] < 0) lo = mid;
            else hi = mid;
        }
        int l = lo; /* under mixed-sign condition, a[lo] < 0 and a[lo+1] >= 0 */

        (void)solve_mixed(a, 0, n - 1, l, 0);
    }

    free(a);
    return 0;
}
