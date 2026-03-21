#include <stdio.h>

int main(void) {
    int stack[10];
    int top = 0; /* number of stored values; next push index */

    int x;
    while (scanf("%d", &x) == 1) {
        if (x != 0) {
            /* Preconditions guarantee top is within 0..9 here. */
            stack[top++] = x;
        } else {
            /* Preconditions guarantee top > 0 here. */
            int v = stack[--top];
            printf("%d\n", v);
        }
    }

    return 0;
}
