#include <stdio.h>

int main(void) {
    int n;
    int p;

    while (scanf("%d %d", &n, &p) == 2) {
        if (n == 0) break;

        int held[50] = {0};
        int pool = p;
        int k = 0;

        for (;;) {
            if (pool > 0) {
                pool -= 1;
                held[k] += 1;

                /* Terminate immediately after a transfer that makes pool zero
                   and current participant holds all p tokens. */
                if (pool == 0 && held[k] == p) {
                    printf("%d\n", k);
                    break;
                }
            } else {
                /* pool == 0: move all tokens from participant k back to pool */
                pool += held[k];
                held[k] = 0;
            }

            k++;
            if (k == n) k = 0;
        }
    }

    return 0;
}
