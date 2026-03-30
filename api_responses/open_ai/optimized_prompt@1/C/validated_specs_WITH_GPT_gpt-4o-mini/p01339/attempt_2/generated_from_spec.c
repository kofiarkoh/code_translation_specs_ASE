#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007

typedef struct {
    int *edges; // adjacency list
    int *next;  // next edge index
    int *head;  // head of the list for each vertex
    int size;   // number of edges
} Graph;

Graph* create_graph(int N, int M) {
    Graph *g = malloc(sizeof(Graph));
    g->edges = malloc(M * sizeof(int));
    g->next = malloc(M * sizeof(int));
    g->head = malloc(N * sizeof(int));
    for (int i = 0; i < N; i++) {
        g->head[i] = -1; // initialize heads to -1
    }
    g->size = 0;
    return g;
}

void add_edge(Graph *g, int u, int v) {
    g->edges[g->size] = v;
    g->next[g->size] = g->head[u];
    g->head[u] = g->size;
    g->size++;
}

void dfs(Graph *g, int v, int *used, int *vs, int *index) {
    used[v] = 1; // mark as visited
    for (int i = g->head[v]; i != -1; i = g->next[i]) {
        int to = g->edges[i];
        if (!used[to]) {
            dfs(g, to, used, vs, index);
        }
    }
    vs[(*index)++] = v; // store the vertex in the order of finishing
}

void rdfs(Graph *g, int v, int *component, int comp_id) {
    component[v] = comp_id; // assign component id
    for (int i = g->head[v]; i != -1; i = g->next[i]) {
        int to = g->edges[i];
        if (component[to] == -1) {
            rdfs(g, to, component, comp_id);
        }
    }
}

int count_paths(Graph *g, int N) {
    int *used = calloc(N, sizeof(int));
    int *vs = malloc(N * sizeof(int));
    int index = 0;

    // First DFS to get the finishing order
    for (int i = 0; i < N; i++) {
        if (!used[i]) {
            dfs(g, i, used, vs, &index);
        }
    }

    // Create the transpose graph
    Graph *gt = create_graph(N, g->size);
    for (int u = 0; u < N; u++) {
        for (int i = g->head[u]; i != -1; i = g->next[i]) {
            int v = g->edges[i];
            add_edge(gt, v, u); // reverse the edge
        }
    }

    // Second DFS on the transposed graph
    int *component = malloc(N * sizeof(int));
    for (int i = 0; i < N; i++) {
        component[i] = -1; // initialize component ids
    }
    int comp_id = 0;

    for (int i = N - 1; i >= 0; i--) {
        if (component[vs[i]] == -1) {
            rdfs(gt, vs[i], component, comp_id++);
        }
    }

    // Count paths in each component
    int *path_count = calloc(comp_id, sizeof(int));
    for (int i = 0; i < comp_id; i++) {
        path_count[i] = 1; // each component has at least one path
    }

    // Calculate the number of paths in each component
    for (int u = 0; u < N; u++) {
        for (int i = g->head[u]; i != -1; i = g->next[i]) {
            int v = g->edges[i];
            if (component[u] == component[v]) {
                path_count[component[u]] = (path_count[component[u]] + path_count[component[v]]) % MOD;
            }
        }
    }

    // Sum up the paths from all components
    long long total_paths = 0;
    for (int i = 0; i < comp_id; i++) {
        total_paths = (total_paths + path_count[i]) % MOD;
    }

    // Clean up
    free(used);
    free(vs);
    free(component);
    free(path_count);
    free(gt->edges);
    free(gt->next);
    free(gt->head);
    free(gt);
    free(g->edges);
    free(g->next);
    free(g->head);
    free(g);

    return (int)total_paths; // return the total paths modulo MOD
}

int main() {
    int N, M;
    scanf("%d %d", &N, &M);
    Graph *g = create_graph(N, M);
    for (int i = 0; i < M; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        add_edge(g, u, v);
    }
    int result = count_paths(g, N);
    printf("%d\n", result);
    return 0;
}
