#include <stdio.h>
#include <stdlib.h>

static void merge_sort(long long *a, int n) {
    if (n <= 1) return;

    int mid = n / 2;
    merge_sort(a, mid);
    merge_sort(a + mid, n - mid);

    /* Sentinel-based merge; requires all input values < 1e18. */
    const long long SENTINEL = 1000000000000000000LL;

    long long *L = (long long *)malloc((size_t)(mid + 1) * sizeof(long long));
    long long *R = (long long *)malloc((size_t)(n - mid + 1) * sizeof(long long));
    /* Per spec: no allocation failure handling; if malloc fails, behavior is undefined. */

    for (int i = 0; i < mid; i++) L[i] = a[i];
    for (int j = 0; j < n - mid; j++) R[j] = a[mid + j];
    L[mid] = SENTINEL;
    R[n - mid] = SENTINEL;

    int i = 0, j = 0;
    for (int k = 0; k < n; k++) {
        if (L[i] <= R[j]) {
            a[k] = L[i++];
        } else {
            a[k] = R[j++];
        }
    }

    free(L);
    free(R);
}

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) {
        /* Input completeness violations are undefined by spec; keep minimal behavior. */
        return 0;
    }

    long long a[N]; /* VLA as required by spec; N must be valid for stack allocation. */
    for (int i = 0; i < N; i++) {
        scanf("%lld", &a[i]);
    }

    merge_sort(a, N);

    long long SumA = 0;
    long long SumB = 0;

    for (int idx = N - 1, t = 0; idx >= 0; idx--, t++) {
        if ((t & 1) == 0) SumA += a[idx];
        else SumB += a[idx];
    }

    printf("%lld\n", (long long)(SumA - SumB));
    return 0;
}
