#include <stdio.h>
#include <limits.h>

#define MAXV 200
#define MAXK 8

int main(void) {
    int n, m, k;
    if (scanf("%d %d %d", &n, &m, &k) != 3) return 0;

    int req[MAXK];
    for (int i = 0; i < k; i++) {
        if (scanf("%d", &req[i]) != 1) return 0;
    }

    /* Use a large sentinel for "disconnected". Keep it well below INT_MAX
       to reduce (but not eliminate) overflow risk in additions. The spec
       explicitly allows undefined behavior on overflow, so we do not add
       extra checks beyond this conservative choice. */
    const int INF = 0x3f3f3f3f;

    static int dist[MAXV][MAXV];

    /* Initialize all-pairs distances */
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            dist[i][j] = (i == j) ? 0 : INF;
        }
    }

    /* Read edges; undirected; keep minimum weight if multiple edges exist */
    for (int e = 0; e < m; e++) {
        int u, v, w;
        if (scanf("%d %d %d", &u, &v, &w) != 3) return 0;
        if (w < dist[u][v]) {
            dist[u][v] = w;
            dist[v][u] = w;
        }
    }

    /* Floyd-Warshall all-pairs shortest paths.
       Assumes shortest paths are well-defined (no relevant negative cycles). */
    for (int mid = 0; mid < n; mid++) {
        for (int i = 0; i < n; i++) {
            int dim = dist[i][mid];
            if (dim == INF) continue;
            for (int j = 0; j < n; j++) {
                int dmj = dist[mid][j];
                if (dmj == INF) continue;
                int cand = dim + dmj; /* may overflow: spec allows undefined behavior */
                if (cand < dist[i][j]) dist[i][j] = cand;
            }
        }
    }

    /* Evaluate minimum over all permutations of required vertices.
       Route may start anywhere; no return-to-start. */
    int best = INF;

    if (k <= 1) {
        /* Visiting 0 or 1 required vertex costs 0 by empty sum convention. */
        best = 0;
    } else {
        int used[MAXK] = {0};
        int perm[MAXK];

        /* Backtracking permutation search */
        /* Narrowest interpretation: exactly once each required vertex. */
        struct Frame {
            int depth;
            int current_cost;
        };

        /* Implement recursion iteratively to avoid relying on deep recursion,
           though k<=8 so recursion would be fine; either is acceptable. */
        /* We'll use a simple recursive function for clarity. */
        void (*dfs)(int, int) = NULL;

        void dfs_impl(int depth, int current_cost) {
            if (depth == k) {
                if (current_cost < best) best = current_cost;
                return;
            }
            for (int i = 0; i < k; i++) {
                if (used[i]) continue;
                int add = 0;
                if (depth > 0) {
                    int a = req[perm[depth - 1]];
                    int b = req[i];
                    add = dist[a][b];
                }
                used[i] = 1;
                perm[depth] = i;
                dfs_impl(depth + 1, current_cost + add); /* may overflow: spec allows UB */
                used[i] = 0;
            }
        }

        dfs = dfs_impl;
        dfs(0, 0);
    }

    printf("%d\n", best);
    return 0;
}
