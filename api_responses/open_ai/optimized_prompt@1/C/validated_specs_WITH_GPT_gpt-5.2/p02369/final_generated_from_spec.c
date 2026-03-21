#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *to;
    int count;
    int cap;
} Adj;

int main(void) {
    int n_vertices, n_edges;
    if (scanf("%d", &n_vertices) != 1) return 0;
    if (scanf("%d", &n_edges) != 1) return 0;

    /* Graph representation: adjacency lists preserving duplicates and self-loops. */
    Adj *adj = (Adj *)calloc((size_t)n_vertices, sizeof(Adj));
    int *indeg = (int *)calloc((size_t)n_vertices, sizeof(int));
    int *removed = (int *)calloc((size_t)n_vertices, sizeof(int));

    /* No explicit error handling per specification; behavior undefined on allocation failure. */

    for (int i = 0; i < n_edges; i++) {
        int u, v;
        if (scanf("%d %d", &u, &v) != 2) return 0;

        /* Ensure capacity for outgoing edges of u. */
        if (adj[u].count == adj[u].cap) {
            int newcap = (adj[u].cap == 0) ? 4 : (adj[u].cap * 2);
            int *newto = (int *)realloc(adj[u].to, (size_t)newcap * sizeof(int));
            adj[u].to = newto;
            adj[u].cap = newcap;
        }
        adj[u].to[adj[u].count++] = v;

        /* Track indegree for Kahn-style removals; duplicates and self-loops count. */
        indeg[v] += 1;
    }

    /*
      Cycle detection via repeated scan for a non-removed vertex with indegree 0,
      remove it, and decrement indegrees of its outgoing neighbors.
      This matches the stated O(V^2 + E) style (vertex-scan step per removal).
    */
    int removed_count = 0;
    while (removed_count < n_vertices) {
        int found = -1;
        for (int v = 0; v < n_vertices; v++) {
            if (!removed[v] && indeg[v] == 0) {
                found = v;
                break;
            }
        }
        if (found < 0) {
            /* No removable vertex remains => cycle exists. */
            printf("1");
            return 0;
        }

        removed[found] = 1;
        removed_count++;

        for (int i = 0; i < adj[found].count; i++) {
            int w = adj[found].to[i];
            indeg[w] -= 1;
        }
    }

    /* All vertices removed => acyclic. */
    printf("0");
    return 0;
}
