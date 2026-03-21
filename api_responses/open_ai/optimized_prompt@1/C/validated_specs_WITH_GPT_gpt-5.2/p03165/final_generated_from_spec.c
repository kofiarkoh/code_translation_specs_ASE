#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
  LCS with deterministic tie-breaking:
  - Standard DP for lengths.
  - Backtrack from (n,m):
      if chars equal: take it and move diagonally
      else move to the neighbor with larger dp value
      if equal, move "up" (i-1, j) deterministically
  This yields a deterministic LCS among multiple valid ones.
*/

int main(void) {
    char a[3000];
    char b[3000];

    /* Input is assumed present and well-formed per specification; otherwise unspecified. */
    if (scanf("%2999s %2999s", a, b) != 2) {
        return 0;
    }

    size_t n = strlen(a);
    size_t m = strlen(b);

    /* Allocate DP table (n+1) x (m+1) of int lengths. */
    size_t rows = n + 1;
    size_t cols = m + 1;
    int *dp = (int *)malloc(rows * cols * sizeof(int));
    /* Allocation failure behavior is unspecified; we do not handle it explicitly. */

    /* Initialize first row/col to 0. */
    for (size_t i = 0; i < rows; i++) dp[i * cols + 0] = 0;
    for (size_t j = 0; j < cols; j++) dp[0 * cols + j] = 0;

    /* Fill DP. */
    for (size_t i = 1; i <= n; i++) {
        for (size_t j = 1; j <= m; j++) {
            if (a[i - 1] == b[j - 1]) {
                dp[i * cols + j] = dp[(i - 1) * cols + (j - 1)] + 1;
            } else {
                int up = dp[(i - 1) * cols + j];
                int left = dp[i * cols + (j - 1)];
                dp[i * cols + j] = (up > left) ? up : left;
            }
        }
    }

    int lcs_len = dp[n * cols + m];
    if (lcs_len <= 0) {
        /* Output nothing if LCS length is zero. */
        free(dp);
        return 0;
    }

    /* Reconstruct one LCS by backtracking with deterministic tie-breaking. */
    char *out = (char *)malloc((size_t)lcs_len + 1);
    /* Allocation failure behavior is unspecified; we do not handle it explicitly. */
    out[lcs_len] = '\0';

    size_t i = n, j = m;
    int k = lcs_len;
    while (i > 0 && j > 0) {
        if (a[i - 1] == b[j - 1]) {
            out[--k] = a[i - 1];
            i--;
            j--;
        } else {
            int up = dp[(i - 1) * cols + j];
            int left = dp[i * cols + (j - 1)];
            if (up > left) {
                i--;
            } else if (left > up) {
                j--;
            } else {
                /* Tie: move up deterministically. */
                i--;
            }
        }
    }

    /* Write characters with no required separators or terminator. */
    fwrite(out, 1, (size_t)lcs_len, stdout);

    free(out);
    free(dp);
    return 0;
}
