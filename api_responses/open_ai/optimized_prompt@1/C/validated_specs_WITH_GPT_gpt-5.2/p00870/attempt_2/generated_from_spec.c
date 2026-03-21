#include <stdio.h>
#include <string.h>
#include <stdint.h>

#define MAX_N 20
#define MAX_TOKEN 4096
#define MAX_TEXT 200000

static int token_read(char *buf, size_t cap) {
    /* Reads a whitespace-delimited token. Returns 1 on success, 0 on EOF. */
    return (scanf("%4095s", buf) == 1);
}

int main(void) {
    int n, m;

    while (scanf("%d %d", &n, &m) == 2) {
        if (n == 0 && m == 0) break;

        /* Fixed limits per specification; exceeding them is undefined. */
        char patterns[MAX_N][MAX_TOKEN];
        int plen[MAX_N];

        for (int i = 0; i < n; i++) {
            if (!token_read(patterns[i], sizeof(patterns[i]))) return 0;
            plen[i] = (int)strlen(patterns[i]);
        }

        static char text[MAX_TEXT + 1];
        int L = 0;
        text[0] = '\0';

        for (int i = 0; i < m; i++) {
            char frag[MAX_TOKEN];
            if (!token_read(frag, sizeof(frag))) return 0;

            size_t flen = strlen(frag);
            if (flen > 0) {
                unsigned char last = (unsigned char)frag[flen - 1];
                if (last < (unsigned char)' ') {
                    frag[flen - 1] = '\0';
                    flen--;
                }
            }

            /* Concatenate; if exceeds MAX_TEXT, behavior is undefined by spec. */
            if (L + (int)flen > MAX_TEXT) {
                /* Undefined; we still avoid overflow in this implementation. */
                flen = (size_t)(MAX_TEXT - L);
                frag[flen] = '\0';
            }
            memcpy(text + L, frag, flen);
            L += (int)flen;
            text[L] = '\0';
        }

        /* Subset DP: reachable[subset] is a bitset over positions 0..L. */
        if (n > MAX_N) {
            /* Undefined by spec; exit quietly. */
            return 0;
        }
        uint32_t FULL = (n == 32) ? 0xFFFFFFFFu : ((n == 0) ? 0u : ((1u << n) - 1u));
        uint32_t subsets = 1u << n;

        int words = (L + 32) / 32; /* enough for bits 0..L inclusive */
        static uint32_t dp[(1u << MAX_N)][(MAX_TEXT + 32) / 32];
        /* Clear only used portion. */
        for (uint32_t s = 0; s < subsets; s++) {
            for (int w = 0; w < words; w++) dp[s][w] = 0;
        }

        /* Set bit for position 0 in subset 0. */
        dp[0][0] |= 1u;

        /* Helper lambdas (as macros) for bit operations. */
#define BIT_TEST(arr, pos) (((arr)[(pos) >> 5] >> ((pos) & 31)) & 1u)
#define BIT_SET(arr, pos)  ((arr)[(pos) >> 5] |= (1u << ((pos) & 31)))

        for (uint32_t s = 0; s < subsets; s++) {
            /* For each reachable position P in subset s, try to place any unused pattern. */
            for (int P = 0; P <= L; P++) {
                if (!BIT_TEST(dp[s], P)) continue;

                for (int i = 0; i < n; i++) {
                    uint32_t bit = 1u << i;
                    if (s & bit) continue;

                    int len = plen[i];
                    if (P + len > L) continue;

                    /* Exact match at position P. */
                    if (len == 0) {
                        /* Empty pattern: matches trivially; still advances by 0. */
                        BIT_SET(dp[s | bit], P);
                    } else if (memcmp(text + P, patterns[i], (size_t)len) == 0) {
                        BIT_SET(dp[s | bit], P + len);
                    }
                }
            }
        }

        int count = 0;
        for (int P = 0; P <= L; P++) {
            if (BIT_TEST(dp[FULL], P)) count++;
        }

        printf("%d\n", count);

#undef BIT_TEST
#undef BIT_SET
    }

    return 0;
}
