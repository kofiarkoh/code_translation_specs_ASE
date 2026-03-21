#include <stdio.h>

int main(void) {
    int N, A, B;
    /* N is read but intentionally unused per specification */
    if (scanf("%d %d %d", &N, &A, &B) != 3) {
        /* Unspecified behavior on invalid/missing input; exit quietly */
        return 0;
    }

    /* If overflow occurs in (B - A), behavior is unspecified per specification. */
    int diff = B - A;

    if ((diff % 2) == 0) {
        puts("Alice");
    } else {
        puts("Borys");
    }

    return 0;
}
