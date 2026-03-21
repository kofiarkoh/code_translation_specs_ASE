#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int to;
    int next;
} Edge;

static int bfs_farthest(int n, const int *head, const Edge *edges, int start, int *depth, int *queue, int *out_maxdepth) {
    /* Invariant per spec: unvisited marked as 0; visited get positive depth exactly once. */
    for (int i = 0; i < n; i++) depth[i] = 0;

    int qh = 0, qt = 0;
    depth[start] = 1; /* positive depth */
    queue[qt++] = start;

    int far = start;
    int maxd = 1;

    while (qh < qt) {
        int v = queue[qh++];
        int dv = depth[v];

        if (dv > maxd) {
            maxd = dv;
            far = v;
        }

        for (int ei = head[v]; ei != -1; ei = edges[ei].next) {
            int u = edges[ei].to;
            if (depth[u] == 0) {
                depth[u] = dv + 1;
                queue[qt++] = u;
            }
        }
    }

    if (out_maxdepth) *out_maxdepth = maxd;
    return far;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* Fixed-size proportional to n; behavior unspecified if n too large per spec. */
    int *head = (int *)malloc((size_t)n * sizeof(int));
    int *depth = (int *)malloc((size_t)n * sizeof(int));
    int *queue = (int *)malloc((size_t)n * sizeof(int));
    Edge *edges = (Edge *)malloc((size_t)(2LL * (n > 0 ? (n - 1) : 0)) * sizeof(Edge));

    if (!head || !depth || !queue || (!edges && n > 1)) {
        /* No explicit error handling required; exit quietly. */
        return 0;
    }

    for (int i = 0; i < n; i++) head[i] = -1;

    int ec = 0;
    for (int i = 0; i < n - 1; i++) {
        int a, b;
        if (scanf("%d %d", &a, &b) != 2) break;
        a--; b--; /* map 1-based to 0-based */

        edges[ec].to = b;
        edges[ec].next = head[a];
        head[a] = ec++;

        edges[ec].to = a;
        edges[ec].next = head[b];
        head[b] = ec++;
    }

    /* First traversal from an arbitrary vertex (0) to find farthest s. */
    int s = 0;
    if (n > 0) {
        s = bfs_farthest(n, head, edges, 0, depth, queue, NULL);
    }

    /* Second traversal from s to get diameter length D as maximum depth value (in vertices). */
    int D = 1;
    if (n > 0) {
        (void)bfs_farthest(n, head, edges, s, depth, queue, &D);
    }

    if (D % 3 == 0) {
        puts("Second");
    } else {
        puts("First");
    }

    free(head);
    free(depth);
    free(queue);
    free(edges);
    return 0;
}
