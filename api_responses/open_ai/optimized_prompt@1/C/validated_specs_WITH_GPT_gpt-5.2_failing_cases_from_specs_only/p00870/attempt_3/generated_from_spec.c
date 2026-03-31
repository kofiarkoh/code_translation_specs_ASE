#include <stdio.h>
#include <string.h>
#include <stdint.h>

#define MAX_N 12
#define MAX_PATLEN 20
#define MAX_L 5000
#define MAX_MASK (1 << MAX_N)

int main(void) {
    int n, m;

    while (scanf("%d %d", &n, &m) == 2) {
        if (n == 0 && m == 0) break;

        char pat[MAX_N][MAX_PATLEN + 1];
        int patlen[MAX_N];

        for (int i = 0; i < n; i++) {
            if (scanf("%20s", pat[i]) != 1) return 0;
            patlen[i] = (int)strlen(pat[i]);
        }

        char target[MAX_L + 1];
        int L = 0;
        target[0] = '\0';

        for (int j = 0; j < m; j++) {
            /* Read fragment token into a buffer large enough for the stated bounds.
               (Behavior is undefined if bounds are violated per specification.) */
            char frag[MAX_L + 1];
            if (scanf("%5000s", frag) != 1) return 0;

            size_t flen = strlen(frag);
            if (flen > 0) {
                /* Normalize: if last character has ASCII code strictly less than ' ',
                   remove it by replacing with terminator. */
                unsigned char last = (unsigned char)frag[flen - 1];
                if (last < (unsigned char)' ') {
                    frag[flen - 1] = '\0';
                    flen--;
                }
            }

            /* Concatenate in order; undefined if total exceeds MAX_L. */
            if (flen > 0) {
                memcpy(target + L, frag, flen);
                L += (int)flen;
                target[L] = '\0';
            }
        }

        int allMask = (n == 0) ? 0 : ((1 << n) - 1);

        /* Reachability table: reachable[mask][pos] is 1 if (pos, mask) reachable.
           Use uint8_t to keep memory small: 4096 * 5001 ~ 20MB. */
        static uint8_t reachable[MAX_MASK][MAX_L + 1];
        for (int mask = 0; mask <= allMask; mask++) {
            memset(reachable[mask], 0, (size_t)(L + 1));
        }

        reachable[0][0] = 1;

        /* DP over subsets; transitions add one new pattern each time. */
        for (int mask = 0; mask <= allMask; mask++) {
            for (int i = 0; i <= L; i++) {
                if (!reachable[mask][i]) continue;

                for (int k = 0; k < n; k++) {
                    int bit = 1 << k;
                    if (mask & bit) continue;

                    int len = patlen[k];
                    if (i + len > L) continue;

                    if (len == 0) {
                        /* Not expected given token input; keep semantics narrow:
                           empty pattern matches trivially and advances by 0. */
                        reachable[mask | bit][i] = 1;
                    } else {
                        if (memcmp(target + i, pat[k], (size_t)len) == 0) {
                            reachable[mask | bit][i + len] = 1;
                        }
                    }
                }
            }
        }

        int count = 0;
        for (int p = 0; p <= L; p++) {
            if (reachable[allMask][p]) count++;
        }

        printf("%d\n", count);
    }

    return 0;
}
