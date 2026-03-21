#include <stdio.h>

#define MAXN 100000

int main(void) {
    int n;
    static int B[MAXN];

    while (scanf("%d", &n) == 1) {
        if (n == 0) break;

        /* Preconditions say n <= 100000 for non-terminating cases. */
        for (int i = 0; i < n; i++) {
            (void)scanf("%d", &B[i]);
        }

        /* Transform in-place as specified. */
        for (int i = 1; i < n; i += 2) {
            if (B[i - 1] != B[i]) {
                int v = B[i];
                int j = i - 1;
                /* Overwrite consecutive elements immediately preceding i
                   that are not already equal to v, stopping at first earlier
                   position that already equals v or at start. */
                while (j >= 0 && B[j] != v) {
                    B[j] = v;
                    j--;
                }
            }
        }

        /* Count zeros in transformed sequence. */
        int zero_count = 0;
        for (int k = 0; k < n; k++) {
            if (B[k] == 0) zero_count++;
        }

        printf("%d\n", zero_count);
    }

    return 0;
}
