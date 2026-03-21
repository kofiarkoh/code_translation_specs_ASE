#include <stdio.h>

#define MAXN 110

static int n;
static int score_map[MAXN];      /* indexed by "number of strictly greater elements" */
static int a[3][MAXN];           /* three sequences */
static int derived[3][MAXN];     /* per-sequence derived scores */
static int total[MAXN];          /* total score per element */

static void compute_derived_for_seq(int s) {
    for (int i = 0; i < n; i++) {
        int greater = 0;
        for (int j = 0; j < n; j++) {
            if (a[s][j] > a[s][i]) greater++;
        }
        /* Preconditions guarantee 0 <= greater <= n-1 and score_map is defined there. */
        derived[s][i] = score_map[greater];
    }
}

static int count_strictly_greater_total_than_first(void) {
    for (int i = 0; i < n; i++) {
        total[i] = derived[0][i] + derived[1][i] + derived[2][i];
    }
    int c = 0;
    for (int i = 1; i < n; i++) {
        if (total[i] > total[0]) c++;
    }
    return c;
}

int main(void) {
    if (scanf("%d", &n) != 1) return 0;

    for (int i = 0; i < n; i++) {
        if (scanf("%d", &score_map[i]) != 1) return 0;
    }

    for (int s = 0; s < 3; s++) {
        for (int i = 0; i < n; i++) {
            if (scanf("%d", &a[s][i]) != 1) return 0;
        }
    }

    /* If n <= 1, then there are 0 other elements; condition is already satisfied with increase 0. */
    if (n <= 1) {
        printf("0\n");
        return 0;
    }

    int best = -1;

    for (int s = 0; s < 3; s++) {
        int orig0 = a[s][0];

        /* Consider increasing only a[s][0] to match another element in same sequence,
           or that value plus one, but strictly greater than original. */
        for (int j = 1; j < n; j++) {
            int base = a[s][j];

            for (int add = 0; add <= 1; add++) {
                int newv = base + add;
                if (newv <= orig0) continue; /* must be strictly greater than original */

                a[s][0] = newv;

                /* Recompute derived scores for all sequences (n is small; simplest faithful approach). */
                compute_derived_for_seq(0);
                compute_derived_for_seq(1);
                compute_derived_for_seq(2);

                int greater_count = count_strictly_greater_total_than_first();
                if (greater_count <= 7) {
                    int inc = newv - orig0; /* non-negative by construction */
                    if (best < 0 || inc < best) best = inc;
                }
            }
        }

        /* Restore original first element as required by invariant. */
        a[s][0] = orig0;
    }

    if (best >= 0) {
        printf("%d\n", best);
    } else {
        printf("No solution\n");
    }

    return 0;
}
