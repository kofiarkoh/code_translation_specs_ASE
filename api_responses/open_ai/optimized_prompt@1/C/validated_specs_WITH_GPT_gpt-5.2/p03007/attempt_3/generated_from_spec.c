#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

static int cmp_i64(const void *p1, const void *p2) {
    int64_t a = *(const int64_t *)p1;
    int64_t b = *(const int64_t *)p2;
    if (a < b) return -1;
    if (a > b) return 1;
    return 0;
}

static int64_t i64_abs(int64_t x) {
    /* Specification assumes all intermediate values fit in signed 64-bit.
       abs(INT64_MIN) would overflow; behavior is not defined by the spec in that case. */
    return (x < 0) ? -x : x;
}

static int64_t *g_a = NULL;
static int g_l = -1;

/* Mixed-sign recursive procedure exactly as specified. */
static int64_t dfs(int f, int b, int sign) {
    if (f == g_l) {
        int64_t sum = g_a[g_l];
        if (sign == 1) {
            for (int i = g_l + 1; i <= b - 1; i++) {
                printf("%lld %lld\n", (long long)sum, (long long)g_a[i]);
                sum = sum - g_a[i];
            }
            printf("%lld %lld\n", (long long)g_a[b], (long long)sum);
            sum = g_a[b] - sum;
            return sum;
        } else { /* sign == -1 */
            for (int i = g_l + 1; i <= b; i++) {
                printf("%lld %lld\n", (long long)sum, (long long)g_a[i]);
                sum = sum - g_a[i];
            }
            return sum;
        }
    }

    if (b == g_l + 1) {
        int64_t sum = g_a[g_l + 1];
        if (sign == 1) {
            for (int i = f; i <= g_l; i++) {
                printf("%lld %lld\n", (long long)sum, (long long)g_a[i]);
                sum = sum - g_a[i];
            }
            return sum;
        } else { /* sign == -1 */
            for (int i = f; i <= g_l - 1; i++) {
                printf("%lld %lld\n", (long long)sum, (long long)g_a[i]);
                sum = sum - g_a[i];
            }
            printf("%lld %lld\n", (long long)g_a[g_l], (long long)sum);
            sum = g_a[g_l] - sum;
            return sum;
        }
    }

    /* Recursive case */
    if (sign == 1) {
        printf("%lld %lld\n", (long long)g_a[b], (long long)g_a[f]);
        int64_t x = dfs(f + 1, b - 1, -sign);
        printf("%lld %lld\n", (long long)(g_a[b] - g_a[f]), (long long)x);
        return (g_a[b] - g_a[f]) - x;
    } else { /* sign == -1 */
        printf("%lld %lld\n", (long long)g_a[f], (long long)g_a[b]);
        int64_t x = dfs(f + 1, b - 1, -sign);
        printf("%lld %lld\n", (long long)(g_a[f] - g_a[b]), (long long)x);
        return (g_a[f] - g_a[b]) - x;
    }
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    int64_t *a = (int64_t *)malloc((size_t)n * sizeof(int64_t));
    if (!a) return 0;

    for (int i = 0; i < n; i++) {
        if (scanf("%lld", (long long *)&a[i]) != 1) {
            free(a);
            return 0;
        }
    }

    qsort(a, (size_t)n, sizeof(int64_t), cmp_i64);

    int64_t S = 0;
    for (int i = 0; i < n; i++) S += i64_abs(a[i]);

    if (a[0] >= 0) {
        /* All non-negative */
        int64_t ans = S - 2 * a[0];
        printf("%lld\n", (long long)ans);

        int64_t x = a[0];
        for (int i = 1; i <= n - 2; i++) {
            printf("%lld %lld\n", (long long)x, (long long)a[i]);
            x = x - a[i];
        }
        printf("%lld %lld\n", (long long)a[n - 1], (long long)x);
    } else if (a[n - 1] <= 0) {
        /* All non-positive */
        int64_t ans = S + 2 * a[n - 1];
        printf("%lld\n", (long long)ans);

        int64_t x = a[n - 1];
        for (int i = 0; i <= n - 2; i++) {
            printf("%lld %lld\n", (long long)x, (long long)a[i]);
            x = x - a[i];
        }
    } else {
        /* Mixed signs */
        printf("%lld\n", (long long)S);

        /* Find l: last index with a[l] < 0 and a[l+1] >= 0 via binary-search-like loop. */
        int l = 0, r = n - 1;
        while (r - l > 1) {
            int m = (l + r) / 2;
            if (a[m] < 0) l = m;
            else r = m;
        }
        g_a = a;
        g_l = l;

        (void)dfs(0, n - 1, 1);
    }

    free(a);
    return 0;
}
