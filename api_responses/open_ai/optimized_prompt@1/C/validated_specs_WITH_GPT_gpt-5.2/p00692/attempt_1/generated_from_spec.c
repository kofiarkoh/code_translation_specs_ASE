#include <stdio.h>
#include <string.h>

#define MAX_CARDS 20

static int best_removed;

/* Check whether positions i and j (i<j) are adjacent in a 5x4 grid
   under the allowed offsets, given the current length n.
   The grid is filled row-major with the current sequence prefix [0..n-1]. */
static int is_allowed_adjacent(int i, int j) {
    int ri = i / 4, ci = i % 4;
    int rj = j / 4, cj = j % 4;

    /* Only allow the four "forward" neighbor directions:
       (r+1,c), (r,c+1), (r+1,c+1), (r+1,c-1) */
    if (rj == ri && cj == ci + 1) return 1;          /* right */
    if (rj == ri + 1 && cj == ci) return 1;          /* down */
    if (rj == ri + 1 && cj == ci + 1) return 1;      /* down-right */
    if (rj == ri + 1 && cj == ci - 1) return 1;      /* down-left */
    return 0;
}

static void dfs(const unsigned char *cards, int n, int removed_pairs) {
    int i, j;

    if (removed_pairs > best_removed) best_removed = removed_pairs;

    /* Explore all possible valid removals from this state. */
    for (i = 0; i < n; i++) {
        for (j = i + 1; j < n; j++) {
            if (cards[i] != cards[j]) continue;
            if (!is_allowed_adjacent(i, j)) continue;

            /* Remove positions i and j, compacting while preserving order. */
            unsigned char next[MAX_CARDS];
            int k, t = 0;
            for (k = 0; k < n; k++) {
                if (k == i || k == j) continue;
                next[t++] = cards[k];
            }
            dfs(next, n - 2, removed_pairs + 1);
        }
    }
}

int main(void) {
    int T;
    if (scanf("%d", &T) != 1) return 0;

    for (int tc = 0; tc < T; tc++) {
        unsigned char cards[MAX_CARDS];
        for (int i = 0; i < MAX_CARDS; i++) {
            /* Specification: "20 single-digit card values".
               Input is assumed well-formed; behavior unspecified otherwise. */
            int x;
            scanf("%d", &x);
            cards[i] = (unsigned char)(x & 0xFF);
        }

        best_removed = 0;
        dfs(cards, MAX_CARDS, 0);

        /* Minimum remaining = 20 - 2*M where M is max removable pairs. */
        printf("%d\n", MAX_CARDS - 2 * best_removed);
    }
    return 0;
}
