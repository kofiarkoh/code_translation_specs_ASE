#include <stdio.h>
#include <stdlib.h>

#define MAXK 100

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) {
        return 0; /* behavior undefined by spec on malformed input; exit quietly */
    }

    /* Per-vertex adjacency with fixed maximum outdegree MAXK */
    int (*to)[MAXK] = NULL;
    int (*w)[MAXK] = NULL;
    int *deg = NULL;

    if (N > 0) {
        to = (int (*)[MAXK])malloc((size_t)N * sizeof(*to));
        w  = (int (*)[MAXK])malloc((size_t)N * sizeof(*w));
        deg = (int *)malloc((size_t)N * sizeof(*deg));
        if (!to || !w || !deg) {
            /* No explicit error handling required by spec */
            return 0;
        }
    }

    for (int i = 0; i < N; i++) deg[i] = 0;

    /* Read exactly N vertex descriptions; each U appears exactly once (assumed). */
    for (int i = 0; i < N; i++) {
        int U, K;
        if (scanf("%d %d", &U, &K) != 2) {
            return 0; /* undefined by spec */
        }
        deg[U] = K;
        for (int j = 0; j < K; j++) {
            int V, W;
            if (scanf("%d %d", &V, &W) != 2) {
                return 0; /* undefined by spec */
            }
            if (j < MAXK) {
                to[U][j] = V;
                w[U][j] = W;
            }
            /* If K > MAXK, behavior is undefined by spec; we do not attempt recovery. */
        }
    }

    int *dist = NULL;
    unsigned char *vis = NULL;

    if (N > 0) {
        dist = (int *)malloc((size_t)N * sizeof(*dist));
        vis  = (unsigned char *)malloc((size_t)N * sizeof(*vis));
        if (!dist || !vis) {
            return 0;
        }
    }

    for (int i = 0; i < N; i++) {
        dist[i] = -1;
        vis[i] = 0;
    }
    if (N > 0) dist[0] = 0;

    /* Dijkstra with O(N^2 + E) time, using -1 as "infinite/unreachable". */
    for (int iter = 0; iter < N; iter++) {
        int u = -1;
        int best = -1;

        for (int i = 0; i < N; i++) {
            if (vis[i]) continue;
            if (dist[i] < 0) continue;
            if (u < 0 || dist[i] < best) {
                u = i;
                best = dist[i];
            }
        }

        if (u < 0) break; /* no more reachable unvisited vertices */

        vis[u] = 1; /* finalize u; its dist will not change thereafter */

        int du = dist[u];
        int k = deg[u];
        if (k > MAXK) k = MAXK; /* precondition says K<=100; keep bounded */
        for (int j = 0; j < k; j++) {
            int v = to[u][j];
            int nd = du + w[u][j]; /* spec guarantees no overflow */
            if (!vis[v]) {
                if (dist[v] < 0 || nd < dist[v]) {
                    dist[v] = nd; /* costs never increase */
                }
            }
        }
    }

    for (int i = 0; i < N; i++) {
        printf("%d %d\n", i, dist[i]);
    }

    free(to);
    free(w);
    free(deg);
    free(dist);
    free(vis);
    return 0;
}
