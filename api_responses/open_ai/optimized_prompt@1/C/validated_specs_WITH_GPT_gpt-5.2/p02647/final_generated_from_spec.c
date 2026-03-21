#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main(void) {
    int N;
    long long K;
    if (scanf("%d %lld", &N, &K) != 2) return 0;

    int *a = (int *)malloc((size_t)N * sizeof(int));
    int *b = (int *)malloc((size_t)N * sizeof(int));
    int *diff = (int *)malloc((size_t)(N + 1) * sizeof(int)); /* N+1 for r+1 updates */
    if (!a || !b || !diff) {
        /* No explicit error handling required by spec; exit quietly if allocation fails. */
        return 0;
    }

    for (int i = 0; i < N; i++) {
        if (scanf("%d", &a[i]) != 1) a[i] = 0; /* unspecified on malformed input */
    }

    for (long long iter = 0; iter < K; iter++) {
        /* Early termination if already pointwise maximal (all N). */
        int allN = 1;
        for (int i = 0; i < N; i++) {
            if (a[i] != N) { allN = 0; break; }
        }
        if (allN) break;

        /* Build range-add difference array for contributions. */
        for (int i = 0; i <= N; i++) diff[i] = 0;

        for (int j = 0; j < N; j++) {
            int v = a[j]; /* precondition: non-negative */
            int l = j - v;
            int r = j + v;
            if (l < 0) l = 0;
            if (r >= N) r = N - 1;

            diff[l] += 1;
            if (r + 1 <= N - 1) {
                diff[r + 1] -= 1;
            } else {
                /* r == N-1 => r+1 == N, within diff size (N+1) */
                diff[N] -= 1;
            }
        }

        int cur = 0;
        for (int i = 0; i < N; i++) {
            cur += diff[i];
            /* By construction, 0 <= cur <= N; keep as int. */
            b[i] = cur;
        }

        /* Swap a and b for next iteration. */
        int *tmp = a; a = b; b = tmp;
    }

    for (int i = 0; i < N; i++) {
        if (i) putchar(' ');
        printf("%d", a[i]);
    }
    putchar('\n');

    free(a);
    free(b);
    free(diff);
    return 0;
}
