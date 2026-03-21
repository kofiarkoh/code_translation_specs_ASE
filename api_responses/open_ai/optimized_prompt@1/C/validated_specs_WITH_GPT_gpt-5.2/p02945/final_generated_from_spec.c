#include <stdio.h>

int main(void) {
    int a, b;
    if (scanf("%d %d", &a, &b) != 2) {
        /* Unspecified behavior per specification if input is missing/invalid. */
        return 0;
    }

    int sum = a + b;
    int diff = a - b;
    int prod = a * b;

    /* Invariant: max is always >= each result considered so far. */
    int max = sum;
    if (diff > max) max = diff;
    if (prod > max) max = prod;

    printf("%d", max);
    return 0;
}
