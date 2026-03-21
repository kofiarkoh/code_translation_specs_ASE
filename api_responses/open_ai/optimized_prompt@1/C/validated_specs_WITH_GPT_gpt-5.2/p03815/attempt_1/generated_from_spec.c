#include <stdio.h>

int main(void) {
    int input;
    if (scanf("%d", &input) != 1) {
        /* Unspecified behavior per spec if input is missing/invalid.
           We choose to produce no output in this case. */
        return 0;
    }

    /* C integer division and remainder for signed integers use truncation toward zero,
       matching the specification's definition of q and r. */
    int q = input / 11;
    int r = input % 11;

    int out;
    if (r == 0) {
        out = 2 * q;
    } else if (r > 6) {
        out = 2 * q + 2;
    } else {
        out = 2 * q + 1;
    }

    printf("%d\n", out);
    return 0;
}
