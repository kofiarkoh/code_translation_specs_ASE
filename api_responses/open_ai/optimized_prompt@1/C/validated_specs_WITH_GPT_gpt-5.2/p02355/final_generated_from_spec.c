#include <stdio.h>

#ifndef MAX_CAP
#define MAX_CAP 200000
#endif

int main(void) {
    int N, K;
    if (scanf("%d %d", &N, &K) != 2) {
        /* Unspecified behavior for malformed input; exit quietly. */
        return 0;
    }

    static int a[MAX_CAP];
    for (int i = 0; i < N; i++) {
        if (scanf("%d", &a[i]) != 1) {
            /* Unspecified behavior for malformed input; exit quietly. */
            return 0;
        }
    }

    if (N < K) {
        printf("0\n");
        return 0;
    }

    /* Sliding window with counts for values 1..K. */
    static int cnt[MAX_CAP]; /* Precondition: K <= MAX_CAP - 1 */
    int have = 0;            /* number of distinct values in 1..K currently present (count > 0) */
    int best = 0;

    int l = 0;
    for (int r = 0; r < N; r++) {
        int x = a[r];
        if (x >= 1 && x <= K) {
            if (cnt[x] == 0) have++;
            cnt[x]++;
        }

        if (have == K) {
            /* Make window minimal w.r.t. left boundary. */
            while (l <= r) {
                int y = a[l];
                if (y >= 1 && y <= K) {
                    if (cnt[y] > 1) {
                        cnt[y]--;
                        l++;
                        continue;
                    }
                    /* cnt[y] == 1: removing would break coverage */
                    break;
                } else {
                    /* Values outside 1..K do not affect coverage; can always drop. */
                    l++;
                }
            }

            int len = r - l + 1;
            if (best == 0 || len < best) best = len;
        }
    }

    printf("%d\n", best);
    return 0;
}
