#include <stdio.h>

int main(void) {
    int a, b;
    if (scanf("%d %d", &a, &b) != 2) {
        /* Unspecified output if input cannot be read as two integers. */
        return 0;
    }

    /* Per specification precondition, arithmetic is representable; otherwise undefined. */
    int expr = a - 2 * b;

    int out = (expr > 0) ? expr : 0;
    printf("%d", out);
    return 0;
}
