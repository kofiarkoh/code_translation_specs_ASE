#include <stdio.h>
#include <string.h>

#define MAX_ITEMS 1000
#define MAX_NAME_LEN 64
#define MAX_REL 200000  /* relationships stored bidirectionally => 2*MAX_REL edges */

typedef struct Edge {
    int to;
    int next;
} Edge;

static char names[MAX_ITEMS][MAX_NAME_LEN];
static int costv[MAX_ITEMS];

static int head[MAX_ITEMS];
static Edge edges[2 * MAX_REL];
static int edge_cnt = 0;

static int visited[MAX_ITEMS];
static int visit_mark = 1;

static int stackv[MAX_ITEMS];

static void add_edge(int u, int v) {
    edges[edge_cnt].to = v;
    edges[edge_cnt].next = head[u];
    head[u] = edge_cnt++;
}

static int find_index_by_name(const char *s, int n) {
    /* No error handling per spec; if not found, behavior is undefined. */
    for (int i = 0; i < n; i++) {
        if (strcmp(names[i], s) == 0) return i;
    }
    return -1;
}

static int reachable_min_cost(int start, int n) {
    /* Computes minimum of the *current* costs over all nodes reachable from start. */
    int top = 0;
    int minc = costv[start];

    visited[start] = visit_mark;
    stackv[top++] = start;

    while (top > 0) {
        int u = stackv[--top];
        if (costv[u] < minc) minc = costv[u];

        for (int e = head[u]; e != -1; e = edges[e].next) {
            int v = edges[e].to;
            if (visited[v] != visit_mark) {
                visited[v] = visit_mark;
                stackv[top++] = v;
            }
        }
    }
    return minc;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* Fixed maximum supported by the program; no explicit error handling per spec. */
    for (int i = 0; i < n; i++) head[i] = -1;

    for (int i = 0; i < n; i++) {
        /* Name token and integer cost. */
        if (scanf("%63s %d", names[i], &costv[i]) != 2) return 0;
    }

    int m;
    if (scanf("%d", &m) != 1) return 0;

    for (int i = 0; i < m; i++) {
        char a[MAX_NAME_LEN], b[MAX_NAME_LEN];
        if (scanf("%63s %63s", a, b) != 2) return 0;
        int u = find_index_by_name(a, n);
        int v = find_index_by_name(b, n);
        add_edge(u, v);
        add_edge(v, u);
    }

    /* Repeatedly, for each item, perform reachability-based minimum-cost evaluation
       and update that item's stored cost to the computed component minimum. */
    long long total = 0; /* Accumulation range is only well-defined if it fits. */
    for (int i = 0; i < n; i++) {
        int minc = reachable_min_cost(i, n);
        costv[i] = minc; /* mutation as specified */
        total += (long long)minc;
        visit_mark++; /* avoid clearing visited[]; overflow behavior not handled */
        if (visit_mark == 0) { /* extremely unlikely; no defined behavior required */
            /* If wrap occurs, fall back to clearing visited to preserve semantics. */
            for (int k = 0; k < n; k++) visited[k] = 0;
            visit_mark = 1;
        }
    }

    /* Output a single integer total. */
    printf("%lld\n", total);
    return 0;
}
