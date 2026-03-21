#include <stdio.h>

int main(void) {
    int a, b;
    if (scanf("%d %d", &a, &b) != 2) {
        /* Unspecified behavior per specification if input is missing/invalid.
           Returning without output is a narrow, reasonable choice. */
        return 0;
    }

    /* Preconditions guarantee (3*a + b) fits in int; overflow otherwise is undefined. */
    int total_half_units = 3 * a + b;

    /* Floor division by 2 for integers; for negative values, C truncates toward zero.
       The specification does not define behavior for negative inputs; we follow C semantics. */
    int whole_units = total_half_units / 2;

    printf("%d", whole_units);
    return 0;
}
