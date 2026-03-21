#include <stdio.h>

static int F(int level, int pos) {
    if (level == 0) {
        return (pos <= 0) ? 0 : 1;
    } else {
        /* Precondition states (2^level - 1) is representable in signed int. */
        int p = (1 << level) - 1;

        if (pos <= 2 * p) {
            return F(level - 1, pos - 1);
        } else {
            return (p + 1) + F(level - 1, pos - 1 - 2 * p);
        }
    }
}

int main(void) {
    int n, k;
    if (scanf("%d %d", &n, &k) != 2) {
        /* Behavior unspecified by the specification for missing/invalid input. */
        return 0;
    }

    printf("%d", F(n, k));
    return 0;
}
