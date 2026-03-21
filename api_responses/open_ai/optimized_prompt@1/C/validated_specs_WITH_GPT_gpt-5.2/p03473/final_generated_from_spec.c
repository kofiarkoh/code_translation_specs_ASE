#include <stdio.h>

int main(void) {
    int x;
    /* No explicit error handling per specification; behavior unspecified if input is missing/malformed. */
    if (scanf("%d", &x) != 1) {
        return 0;
    }

    /* Compute and output (48 - input_value). */
    printf("%d\n", 48 - x);
    return 0;
}
