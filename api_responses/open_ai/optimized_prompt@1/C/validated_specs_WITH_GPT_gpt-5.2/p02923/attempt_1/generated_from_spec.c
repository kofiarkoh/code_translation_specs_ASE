#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Unspecified behavior per spec if input is invalid; exit quietly. */
        return 0;
    }

    /* Preconditions say n is non-negative and enough memory exists. */
    int *h = NULL;
    if (n > 0) {
        h = (int *)malloc((size_t)n * sizeof(int));
        /* No explicit handling for allocation failure per spec. */
    }

    for (int i = 0; i < n; i++) {
        (void)scanf("%d", &h[i]);
    }

    if (n <= 1) {
        printf("0");
        free(h);
        return 0;
    }

    int current = 0; /* length in steps of current suffix run */
    int best = 0;    /* maximum length in steps seen so far */

    for (int i = 0; i < n - 1; i++) {
        if (h[i + 1] <= h[i]) {
            current += 1;
        } else {
            current = 0;
        }
        if (current > best) best = current;
    }

    printf("%d", best);

    free(h);
    return 0;
}
