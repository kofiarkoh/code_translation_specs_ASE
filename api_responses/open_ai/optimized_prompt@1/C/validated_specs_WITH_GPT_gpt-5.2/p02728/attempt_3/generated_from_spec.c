#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MOD 1000000007LL

typedef struct Edge {
    int to;
    int next;
} Edge;

static int n;
static int *head;
static Edge *edges;
static int ecnt;

static int *parent;
static int *order;
static int *subsz;

static int64_t *inv;     /* inv[x] for 1..n (0 unused) */
static int64_t *ans;     /* per-vertex answer modulo MOD */

static int64_t mod_pow(int64_t a, int64_t e) {
    int64_t r = 1 % MOD;
    a %= MOD;
    while (e > 0) {
        if (e & 1) r = (r * a) % MOD;
        a = (a * a) % MOD;
        e >>= 1;
    }
    return r;
}

static void add_edge(int u, int v) {
    edges[ecnt].to = v;
    edges[ecnt].next = head[u];
    head[u] = ecnt++;
}

int main(void) {
    if (scanf("%d", &n) != 1) return 0;

    head = (int*)malloc((n + 1) * sizeof(int));
    edges = (Edge*)malloc((size_t)(2 * (n > 0 ? (n - 1) : 0) + 5) * sizeof(Edge));
    parent = (int*)malloc((n + 1) * sizeof(int));
    order = (int*)malloc((n + 1) * sizeof(int));
    subsz = (int*)malloc((n + 1) * sizeof(int));
    inv = (int64_t*)malloc((n + 1) * sizeof(int64_t));
    ans = (int64_t*)malloc((n + 1) * sizeof(int64_t));

    if (!head || !edges || !parent || !order || !subsz || !inv || !ans) {
        /* Unspecified behavior per spec; just exit if allocation fails. */
        return 0;
    }

    for (int i = 1; i <= n; i++) head[i] = -1;
    ecnt = 0;

    for (int i = 0; i < n - 1; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        add_edge(u, v);
        add_edge(v, u);
    }

    /* Precompute inverses 1..n using Fermat (MOD is prime). */
    inv[0] = 0;
    for (int i = 1; i <= n; i++) inv[i] = mod_pow(i, MOD - 2);

    /* Compute n! modulo MOD (global multiplier). */
    int64_t factn = 1;
    for (int i = 2; i <= n; i++) factn = (factn * i) % MOD;

    if (n == 1) {
        /* Removing the only vertex leaves no components: empty product = 1. */
        printf("%lld\n", (long long)factn);
        free(head); free(edges); free(parent); free(order); free(subsz); free(inv); free(ans);
        return 0;
    }

    /*
     * Root the tree at 1, compute parent[] and a traversal order (BFS/stack),
     * then compute subtree sizes in reverse order.
     */
    int top = 0;
    order[top++] = 1;
    parent[1] = 0;

    for (int i = 0; i < top; i++) {
        int u = order[i];
        for (int e = head[u]; e != -1; e = edges[e].next) {
            int v = edges[e].to;
            if (v == parent[u]) continue;
            parent[v] = u;
            order[top++] = v;
        }
    }

    for (int i = 1; i <= n; i++) subsz[i] = 1;
    for (int i = n - 1; i >= 0; i--) {
        int u = order[i];
        for (int e = head[u]; e != -1; e = edges[e].next) {
            int v = edges[e].to;
            if (v == parent[u]) continue;
            subsz[u] += subsz[v];
        }
    }

    /*
     * For each vertex v, removing v yields components:
     * - For each child c of v: component size = subsz[c]
     * - If v is not the root: the "rest of tree" component size = n - subsz[v]
     * Answer(v) = n! * product(inv[size]) over these components.
     *
     * This directly matches the specification; no extra factorials appear.
     */
    for (int v = 1; v <= n; v++) {
        int64_t prod = 1;
        for (int e = head[v]; e != -1; e = edges[e].next) {
            int u = edges[e].to;
            if (u == parent[v]) continue; /* child */
            prod = (prod * inv[subsz[u]]) % MOD;
        }
        if (parent[v] != 0) {
            int rest = n - subsz[v];
            /* rest should be >= 1 for non-root in a tree with n>=2 */
            prod = (prod * inv[rest]) % MOD;
        }
        ans[v] = (factn * prod) % MOD;
    }

    for (int i = 1; i <= n; i++) {
        printf("%lld\n", (long long)ans[i]);
    }

    free(head);
    free(edges);
    free(parent);
    free(order);
    free(subsz);
    free(inv);
    free(ans);
    return 0;
}
