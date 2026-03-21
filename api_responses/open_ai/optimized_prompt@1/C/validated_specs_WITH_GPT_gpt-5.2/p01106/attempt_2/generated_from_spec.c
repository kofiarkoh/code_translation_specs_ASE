#include <stdio.h>
#include <stdint.h>

int main(void) {
    int n;
    int64_t i, j;

    /* Fixed-size buffers as implied by the specification (size 62, writes up to n+1). */
    int low[62];
    char out[62];

    while (1) {
        if (scanf("%d %lld %lld", &n, (long long *)&i, (long long *)&j) != 3) {
            return 0; /* input exhausted / parsing fails */
        }
        if (n <= 0) {
            return 0; /* terminating case not processed */
        }

        int64_t I = i - 1; /* 1-based to 0-based working values */
        int64_t J = j - 1;

        /* Phase A: derive low[] from I, mutating I via full-width bitwise complement. */
        for (int t = 0; t < n; t++) {
            int p = (n - 1) - t;
            int bit = (int)((I >> p) & 1LL); /* signed right shift as in C */
            low[t] = bit;
            if (bit == 0) {
                I = ~I;
            }
        }

        /* Phase B: produce output from J driven by low[], mutating J similarly. */
        for (int t = 0; t < n; t++) {
            int p = (n - 1) - t;
            int b = (int)((J >> p) & 1LL);
            out[t] = (b == low[t]) ? 'L' : 'R';
            if (low[t] == 0) {
                J = ~J;
            }
        }
        out[n] = '\0';

        fputs(out, stdout);
        fputc('\n', stdout);
    }
}
