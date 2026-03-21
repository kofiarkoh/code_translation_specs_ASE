#include <stdio.h>

static int is_prime(int n) {
    int d;

    if (n < 2) return 0;
    if (n == 2) return 1;
    if ((n % 2) == 0) return 0;

    /* Precondition: d*d does not overflow int for all d tested. */
    for (d = 3; d * d <= n; d += 2) {
        if ((n % d) == 0) return 0;
    }
    return 1;
}

int main(void) {
    int x;

    while (scanf("%d", &x) == 1) {
        int lower = x;
        int upper = x;

        /* Preconditions say x >= 2; otherwise behavior is not guaranteed. */

        while (!is_prime(lower)) {
            lower--;
        }
        while (!is_prime(upper)) {
            upper++;
        }

        printf("%d\n", upper - lower);
    }

    return 0;
}
