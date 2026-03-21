#include <stdio.h>

int main(void) {
    int a, b, c;

    /* Per specification: no explicit error handling; if input fails, behavior is undefined. */
    (void)scanf("%d %d %d", &a, &b, &c);

    /* Sort three integers into non-decreasing order using swaps (constant time/memory). */
    if (a > b) { int t = a; a = b; b = t; }
    if (b > c) { int t = b; b = c; c = t; }
    if (a > b) { int t = a; a = b; b = t; }

    printf("%d %d %d\n", a, b, c);
    return 0;
}
