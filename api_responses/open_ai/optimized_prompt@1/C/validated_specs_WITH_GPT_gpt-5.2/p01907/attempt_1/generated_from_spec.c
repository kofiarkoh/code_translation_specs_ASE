#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int to;
    int w;
    int next;
} Edge;

int main(void) {
    double P;
    int N;

    if (scanf("%lf", &P) != 1) return 0;
    if (scanf("%d", &N) != 1) return 0;

    /* Adjacency list for an undirected tree: 2*(N-1) directed edges */
    int m = (N > 1) ? (2 * (N - 1)) : 0;
    Edge *edges = (Edge *)malloc((size_t)m * sizeof(Edge));
    int *head = (int *)malloc((size_t)(N + 1) * sizeof(int));
    if (!head || (m > 0 && !edges)) {
        /* Specification: undefined behavior on allocation failure; exit quietly */
        return 0;
    }

    for (int i = 1; i <= N; i++) head[i] = -1;

    int ecnt = 0;
    for (int i = 0; i < N - 1; i++) {
        int x, y, c;
        if (scanf("%d %d %d", &x, &y, &c) != 3) return 0;

        edges[ecnt].to = y;
        edges[ecnt].w = c;
        edges[ecnt].next = head[x];
        head[x] = ecnt++;

        edges[ecnt].to = x;
        edges[ecnt].w = c;
        edges[ecnt].next = head[y];
        head[y] = ecnt++;
    }

    /* BFS from root 1 to assign depth and parent edge weight */
    int *depth = (int *)malloc((size_t)(N + 1) * sizeof(int));
    int *parentW = (int *)malloc((size_t)(N + 1) * sizeof(int));
    int *q = (int *)malloc((size_t)(N + 1) * sizeof(int));
    if (!depth || !parentW || !q) {
        /* Specification: undefined behavior on allocation failure; exit quietly */
        return 0;
    }

    for (int i = 1; i <= N; i++) {
        depth[i] = -1;     /* unvisited */
        parentW[i] = 0;    /* root contributes 0; also default for unreachable */
    }

    int qh = 0, qt = 0;
    if (N >= 1) {
        depth[1] = 0;
        parentW[1] = 0;
        q[qt++] = 1;
    }

    while (qh < qt) {
        int v = q[qh++];
        for (int e = head[v]; e != -1; e = edges[e].next) {
            int u = edges[e].to;
            if (depth[u] == -1) {
                depth[u] = depth[v] + 1;
                parentW[u] = edges[e].w;
                q[qt++] = u;
            }
        }
    }

    /* Aggregate sums:
       S1 = sum_v parentEdgeWeight(v) * P^depth(v)
       S2 = sum_v P^depth(v)
       For nodes not reached from 1 (invalid input per preconditions), depth=-1;
       behavior is not guaranteed by spec. We choose the narrowest safe behavior:
       ignore unreachable nodes (they contribute nothing). */
    double S1 = 0.0;
    double S2 = 0.0;

    /* Compute P^depth via iterative multiplication by depth levels to stay linear. */
    int maxDepth = 0;
    for (int v = 1; v <= N; v++) {
        if (depth[v] > maxDepth) maxDepth = depth[v];
    }

    /* Count nodes per depth and sum parent weights per depth */
    int *cnt = (int *)calloc((size_t)(maxDepth + 1), sizeof(int));
    long long *sumW = (long long *)calloc((size_t)(maxDepth + 1), sizeof(long long));
    if (!cnt || !sumW) {
        /* Specification: undefined behavior on allocation failure; exit quietly */
        return 0;
    }

    for (int v = 1; v <= N; v++) {
        int d = depth[v];
        if (d >= 0) {
            cnt[d] += 1;
            sumW[d] += (long long)parentW[v];
        }
    }

    double powPd = 1.0; /* P^0 */
    for (int d = 0; d <= maxDepth; d++) {
        S2 += (double)cnt[d] * powPd;
        S1 += (double)sumW[d] * powPd;
        powPd *= P;
    }

    double result = S1 * S2;
    /* Output one real number; no specific formatting mandated. */
    printf("%.10f\n", result);

    return 0;
}
