#include <stdio.h>
#include <string.h>

#define MAXN 205

static int char_index(char ch) {
    if (ch >= 'A' && ch <= 'Z') return ch - 'A';
    if (ch >= 'a' && ch <= 'z') return 26 + (ch - 'a');
    return -1; /* unspecified by spec if non-alphabetic */
}

static char index_char(int idx) {
    if (idx >= 0 && idx < 26) return (char)('A' + idx);
    return (char)('a' + (idx - 26));
}

/* Standard DFS augmenting path for bipartite matching (rows -> cols) */
static int N;
static int matchR[MAXN];      /* matchR[col] = row or -1 */
static unsigned char seen[MAXN];

static int dfs_augment(int u, unsigned char adj[MAXN][MAXN]) {
    for (int v = 0; v < N; v++) {
        if (!adj[u][v] || seen[v]) continue;
        seen[v] = 1;
        if (matchR[v] == -1 || dfs_augment(matchR[v], adj)) {
            matchR[v] = u;
            return 1;
        }
    }
    return 0;
}

static int max_matching(unsigned char adj[MAXN][MAXN]) {
    for (int v = 0; v < N; v++) matchR[v] = -1;
    int m = 0;
    for (int u = 0; u < N; u++) {
        memset(seen, 0, (size_t)N);
        if (dfs_augment(u, adj)) m++;
    }
    return m;
}

int main(void) {
    if (scanf("%d", &N) != 1) return 0;

    static char grid[MAXN][MAXN];
    static char linebuf[4096];

    /* Read N lines; each must contain at least N characters; extra ignored. */
    for (int i = 0; i < N; i++) {
        if (scanf("%s", linebuf) != 1) return 0;
        for (int j = 0; j < N; j++) {
            grid[i][j] = linebuf[j];
        }
    }

    /* We build the matching lexicographically by maximizing counts of earlier chars.
       For each character in fixed order, we try to force as many edges of that char
       as possible while keeping feasibility for a perfect matching. */
    static unsigned char allowed[MAXN][MAXN];
    static unsigned char forced[MAXN][MAXN];

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            allowed[i][j] = 1; /* complete bipartite graph always feasible */
            forced[i][j] = 0;
        }
    }

    int counts[52];
    for (int k = 0; k < 52; k++) counts[k] = 0;

    for (int k = 0; k < 52; k++) {
        char target = index_char(k);

        /* Collect candidate edges of this character that are still allowed and not forced. */
        int candU[MAXN * MAXN];
        int candV[MAXN * MAXN];
        int candCount = 0;

        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                if (allowed[i][j] && !forced[i][j] && grid[i][j] == target) {
                    candU[candCount] = i;
                    candV[candCount] = j;
                    candCount++;
                }
            }
        }

        /* Greedily force edges of this character when doing so preserves existence
           of a perfect matching. This yields the maximum possible count for this
           character given earlier forced decisions. */
        int forcedThis = 0;
        for (int e = 0; e < candCount; e++) {
            int u = candU[e], v = candV[e];

            /* Build adjacency = allowed with all forced edges required, and additionally require (u,v). */
            static unsigned char adj[MAXN][MAXN];
            for (int i = 0; i < N; i++) {
                for (int j = 0; j < N; j++) {
                    adj[i][j] = allowed[i][j];
                }
            }

            /* Enforce forced edges by restricting their incident row/col to only that edge. */
            for (int i = 0; i < N; i++) {
                for (int j = 0; j < N; j++) {
                    if (forced[i][j]) {
                        for (int jj = 0; jj < N; jj++) adj[i][jj] = 0;
                        for (int ii = 0; ii < N; ii++) adj[ii][j] = 0;
                        adj[i][j] = 1;
                    }
                }
            }

            /* Enforce candidate edge (u,v) similarly. */
            for (int jj = 0; jj < N; jj++) adj[u][jj] = 0;
            for (int ii = 0; ii < N; ii++) adj[ii][v] = 0;
            adj[u][v] = 1;

            if (max_matching(adj) == N) {
                /* Commit: force (u,v) and remove other edges in its row/col from allowed. */
                forced[u][v] = 1;
                forcedThis++;

                for (int jj = 0; jj < N; jj++) {
                    if (jj != v) allowed[u][jj] = 0;
                }
                for (int ii = 0; ii < N; ii++) {
                    if (ii != u) allowed[ii][v] = 0;
                }
            }
        }

        counts[k] = forcedThis;
    }

    /* At this point, forced edges may be fewer than N (if some characters never forced).
       Compute any perfect matching consistent with allowed+forced, then count characters. */
    static unsigned char finalAdj[MAXN][MAXN];
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            finalAdj[i][j] = allowed[i][j];
        }
    }
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (forced[i][j]) {
                for (int jj = 0; jj < N; jj++) finalAdj[i][jj] = 0;
                for (int ii = 0; ii < N; ii++) finalAdj[ii][j] = 0;
                finalAdj[i][j] = 1;
            }
        }
    }

    (void)max_matching(finalAdj); /* spec assumes always feasible */

    /* matchR[col] = row; derive chosen edges and counts exactly. */
    int finalCounts[52];
    for (int k = 0; k < 52; k++) finalCounts[k] = 0;

    for (int col = 0; col < N; col++) {
        int row = matchR[col];
        if (row < 0) continue; /* unspecified; but spec says perfect matching exists */
        int idx = char_index(grid[row][col]);
        if (idx >= 0 && idx < 52) finalCounts[idx]++;
    }

    /* Output sorted multiset in fixed order. */
    for (int k = 0; k < 52; k++) {
        char ch = index_char(k);
        for (int t = 0; t < finalCounts[k]; t++) putchar(ch);
    }
    putchar('\n');
    return 0;
}
