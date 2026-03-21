#include <stdio.h>
#include <stdlib.h>

int main(void) {
    long long n_ll, k_ll;
    if (scanf("%lld %lld", &n_ll, &k_ll) != 2) {
        /* Unspecified behavior per spec if inputs are missing/invalid. */
        return 0;
    }

    /* Per spec: n is non-negative; k > 1 is required, otherwise undefined. */
    size_t n = (size_t)n_ll;
    long long k = k_ll;

    /* Memory usage proportional to n due to storing the input sequence. */
    int *a = (int *)malloc(n * sizeof(int));
    /* If allocation fails, behavior is undefined per spec; we do not handle it. */

    for (size_t i = 0; i < n; i++) {
        /* Values are read and stored but do not affect the result. */
        (void)scanf("%d", &a[i]);
    }

    long long t;
    if ((long long)n <= k) {
        t = 1;
    } else {
        /* Smallest positive integer t such that k + (t-1)*(k-1) >= n.
           Let m = n - k > 0. Need (t-1) >= ceil(m / (k-1)).
           So t = 1 + ceil((n-k)/(k-1)).
        */
        long long m = (long long)n - k;
        long long d = k - 1; /* k > 1 per precondition */
        long long q = (m + d - 1) / d; /* ceil division for positive m,d */
        t = 1 + q;
    }

    printf("%lld", t);

    free(a);
    return 0;
}
