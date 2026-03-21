#include <stdio.h>

#define MAXN 1000

int main(void) {
    int n, m;
    static int a[MAXN];
    static int b[MAXN];

    if (scanf("%d", &n) != 1) return 0;
    if (scanf("%d", &m) != 1) return 0;

    /* Preconditions say lengths are non-negative and within capacity.
       If violated, behavior is unspecified; we do not add explicit handling. */
    for (int i = 0; i < n; i++) {
        (void)scanf("%d", &a[i]);
    }
    for (int j = 0; j < m; j++) {
        (void)scanf("%d", &b[j]);
    }

    int best = 0;

    /* For each starting index s in the second sequence, greedily match
       the longest prefix of b[s..] as a subsequence of a with strictly
       increasing positions. Greedy is correct for subsequence matching:
       always taking the earliest possible match in a maximizes the number
       of matches for a fixed b-suffix. */
    for (int s = 0; s < m; s++) {
        int i = 0;      /* current scan position in a */
        int matched = 0;

        for (int j = s; j < m; j++) {
            int found = 0;
            while (i < n) {
                if (a[i] == b[j]) {
                    i++;          /* next match must be strictly after this */
                    found = 1;
                    matched++;
                    break;
                }
                i++;
            }
            if (!found) break;    /* cannot extend contiguous segment further */
        }

        if (matched > best) best = matched; /* invariant: best never decreases */
    }

    printf("%d", best);
    return 0;
}
