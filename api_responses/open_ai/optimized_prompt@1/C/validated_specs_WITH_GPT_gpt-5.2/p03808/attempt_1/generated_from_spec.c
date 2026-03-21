#include <stdio.h>
#include <stdint.h>

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) return 0;

    static int64_t a[100001];
    for (int i = 1; i <= N; i++) {
        if (scanf("%" SCNd64, &a[i]) != 1) return 0;
    }

    if (N == 0) {
        /* Specification does not define behavior for N=0 due to division by N(N+1)/2.
           Choose the narrowest safe rejection. */
        puts("NO");
        return 0;
    }

    int64_t sum = 0;
    for (int i = 1; i <= N; i++) sum += a[i];

    int64_t denom = (int64_t)N * (N + 1) / 2;
    if (denom == 0 || (sum % denom) != 0) {
        puts("NO");
        return 0;
    }

    int64_t S = sum / denom;

    /* Cyclic previous: a[0] is a[N] */
    a[0] = a[N];

    int64_t sumx = 0;
    int64_t weighted = 0;

    for (int i = 1; i <= N; i++) {
        int64_t diff = a[i] - a[i - 1];
        int64_t t = S - diff;

        if ((t % N) != 0) {
            puts("NO");
            return 0;
        }

        int64_t x = t / N;

        if (x < 0 || x > S) {
            /* Case varies by failing condition per specification */
            puts("No");
            return 0;
        }

        sumx += x;
        weighted += x * (int64_t)(N - i + 1);
    }

    if (sumx != S) {
        puts("NO");
        return 0;
    }

    if (a[N] != weighted) {
        puts("NO");
        return 0;
    }

    puts("YES");
    return 0;
}
