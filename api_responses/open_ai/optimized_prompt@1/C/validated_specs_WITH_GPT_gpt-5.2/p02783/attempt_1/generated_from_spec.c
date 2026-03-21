#include <stdio.h>

int main(void) {
    int dividend, divisor;

    /* Behavior is unspecified if input does not contain two valid integers. */
    if (scanf("%d %d", &dividend, &divisor) != 2) {
        return 0;
    }

    /* Preconditions: divisor is non-zero; inputs assumed non-negative for ceiling meaning. */
    int q = dividend / divisor; /* Undefined behavior if divisor == 0. */
    int r = dividend % divisor; /* Remainder; for non-negative inputs, r is in [0, divisor-1]. */

    int result = q + (r != 0);  /* If not exactly divisible, round up by 1. */

    printf("%d\n", result);
    return 0;
}
