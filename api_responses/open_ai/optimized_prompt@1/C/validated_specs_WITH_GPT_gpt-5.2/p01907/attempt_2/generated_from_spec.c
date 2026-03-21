#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Edge {
    int to;
    int w;
    int next;
} Edge;

int main(void) {
    double P;
    int N;

    if (scanf("%lf", &P) != 1) return 0;
    if (scanf("%d", &N) != 1) return 0;

    /* Handle N <= 0 narrowly: no nodes => empty sums, b = 1, a = 0 => output 0. */
    if (N <= 0) {
        printf("%.10f\n", 0.0);
        return 0;
    }

    int m = (N > 0) ? (N - 1) : 0;
    int E = (m > 0) ? (2 * m) : 0;

    int *head = (int *)malloc((size_t)(N + 1) * sizeof(int));
    Edge *edges = (Edge *)malloc((size_t)(E > 0 ? E : 1) * sizeof(Edge));
    if (!head || !edges) {
        /* Undefined behavior per spec on allocation failure; return. */
        return 0;
    }
    for (int i = 0; i <= N; i++) head[i] = -1;

    int ec = 0;
    for (int i = 0; i < m; i++) {
        int x, y;
        int c;
        if (scanf("%d %d %d", &x, &y, &c) != 3) {
            /* Incomplete input: stop building; traversal will use what was read. */
            break;
        }
        if (ec + 1 < E) {
            edges[ec].to = y;
            edges[ec].w = c;
            edges[ec].next = head[x];
            head[x] = ec++;
            edges[ec].to = x;
            edges[ec].w = c;
            edges[ec].next = head[y];
            head[y] = ec++;
        } else if (E == 0) {
            /* No edges expected; ignore. */
        } else {
            /* If input has more than N-1 edges, ignore extras to keep bounds. */
        }
    }

    unsigned char *vis = (unsigned char *)calloc((size_t)(N + 1), 1);
    int *depth = (int *)calloc((size_t)(N + 1), sizeof(int));
    int *parW = (int *)calloc((size_t)(N + 1), sizeof(int));
    int *queue = (int *)malloc((size_t)(N + 1) * sizeof(int));
    if (!vis || !depth || !parW || !queue) {
        return 0;
    }

    /* Traversal from node 1: BFS assigns first-discovery parent edge and depth. */
    int qh = 0, qt = 0;
    vis[1] = 1;
    depth[1] = 0;
    parW[1] = 0;
    queue[qt++] = 1;

    int maxDepth = 0;

    while (qh < qt) {
        int u = queue[qh++];
        int du = depth[u];
        for (int ei = head[u]; ei != -1; ei = edges[ei].next) {
            int v = edges[ei].to;
            if (v < 1 || v > N) continue; /* Spec says indices are in range; be defensive. */
            if (!vis[v]) {
                vis[v] = 1;
                depth[v] = du + 1;
                parW[v] = edges[ei].w;
                if (depth[v] > maxDepth) maxDepth = depth[v];
                queue[qt++] = v;
            }
        }
    }

    /* Precompute powers P^d up to maxDepth reached. */
    double *powP = (double *)malloc((size_t)(maxDepth + 1) * sizeof(double));
    if (!powP) {
        return 0;
    }
    powP[0] = 1.0;
    for (int d = 1; d <= maxDepth; d++) {
        powP[d] = powP[d - 1] * P;
    }

    /* Accumulate over all nodes 1..N, including unreached nodes with default depth=0, parW=0. */
    double a = 0.0;
    double sumP = 0.0;
    for (int v = 1; v <= N; v++) {
        int d = depth[v];
        double pd = (d >= 0 && d <= maxDepth) ? powP[d] : 1.0; /* d should be within range. */
        a += (double)parW[v] * pd;
        sumP += pd;
    }
    double b = 1.0 + sumP;
    double out = a * b;

    printf("%.10f\n", out);

    free(powP);
    free(queue);
    free(parW);
    free(depth);
    free(vis);
    free(edges);
    free(head);
    return 0;
}
