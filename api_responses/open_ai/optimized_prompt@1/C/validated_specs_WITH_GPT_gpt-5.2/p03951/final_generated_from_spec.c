#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) {
        /* Undefined behavior per spec if input missing/malformed; just exit. */
        return 0;
    }

    /* Per spec: N is non-negative; if negative, behavior is undefined. */
    size_t n = (size_t)N;

    /* Allocate storage proportional to N; stack allocation is allowed by spec,
       but heap avoids stack overflow for large N while preserving semantics. */
    char *s1 = (char *)malloc(n + 1);
    char *s2 = (char *)malloc(n + 1);
    if (!s1 || !s2) {
        /* Spec says behavior undefined if too large; just exit. */
        free(s1);
        free(s2);
        return 0;
    }

    if (scanf("%s %s", s1, s2) != 2) {
        /* Undefined behavior per spec; just exit. */
        free(s1);
        free(s2);
        return 0;
    }

    /* Compute k = max in [0, N] such that suffix of s1 of length k
       equals prefix of s2 of length k. */
    size_t k = 0;
    for (size_t cand = n; cand > 0; cand--) {
        /* Compare s1[n-cand .. n-1] with s2[0 .. cand-1] */
        if (memcmp(s1 + (n - cand), s2, cand) == 0) {
            k = cand;
            break;
        }
    }

    /* Output = 2*N - k, with k in [0, N]. */
    long long out = (long long)2 * (long long)N - (long long)k;
    printf("%lld\n", out);

    free(s1);
    free(s2);
    return 0;
}
