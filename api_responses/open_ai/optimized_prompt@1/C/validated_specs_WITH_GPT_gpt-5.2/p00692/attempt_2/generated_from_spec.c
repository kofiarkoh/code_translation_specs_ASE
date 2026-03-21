#include <stdio.h>

static int best_depth;

static void dfs(int *a, int L, int depth) {
    if (depth > best_depth) best_depth = depth;

    for (int i = 0; i < L; i++) {
        int v = a[i];

        /* Enumerate only the program-defined forward neighbors. */
        int candidates[4];
        int cnt = 0;

        /* Right: i+1, same row (i%4 < 3) */
        if ((i % 4) < 3) candidates[cnt++] = i + 1;

        /* Down: i+4, within original 5x4 bounds (i < 16) */
        if (i < 16) candidates[cnt++] = i + 4;

        /* Down-right: i+5, within bounds and not last column */
        if (i < 16 && (i % 4) < 3) candidates[cnt++] = i + 5;

        /* Down-left: i+3, within bounds and not first column */
        if (i < 16 && (i % 4) > 0) candidates[cnt++] = i + 3;

        for (int k = 0; k < cnt; k++) {
            int j = candidates[k];
            if (j >= L) continue;          /* must be within current prefix */
            if (a[j] != v) continue;       /* values must match */

            /* Remove indices i and j, compacting while preserving order. */
            int b[20];
            int t = 0;
            for (int p = 0; p < L; p++) {
                if (p == i || p == j) continue;
                b[t++] = a[p];
            }
            dfs(b, L - 2, depth + 1);
        }
    }
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    for (int tc = 0; tc < n; tc++) {
        int a[20];
        for (int i = 0; i < 20; i++) {
            char c;
            /* Leading whitespace is skipped; each card is a single digit char. */
            if (scanf(" %c", &c) != 1) return 0;
            a[i] = (int)(c - '0');
        }

        best_depth = 0;
        dfs(a, 20, 0);

        int remaining = 20 - 2 * best_depth;
        printf("%d\n", remaining);
    }

    return 0;
}
