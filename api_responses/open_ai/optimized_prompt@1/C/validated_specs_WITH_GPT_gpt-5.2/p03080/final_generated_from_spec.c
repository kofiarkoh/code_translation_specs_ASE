#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int count;
    /* Fixed upper bound on stored input sequence length (non-functional constraint). */
    char seq[1000005];

    if (scanf("%d", &count) != 1) {
        /* Input failure: terminate with non-zero status and no specified output. */
        return 1;
    }
    if (scanf("%1000004s", seq) != 1) {
        /* Input failure: terminate with non-zero status and no specified output. */
        return 1;
    }

    int total = 0;
    for (int i = 0; i < count; i++) {
        /* Only 'R' contributes +1; any other character contributes -1. */
        total += (seq[i] == 'R') ? 1 : -1;
    }

    /* Affirmative iff total is strictly greater than zero. */
    if (total > 0) {
        puts("Yes");
    } else {
        puts("No");
    }

    return 0;
}
