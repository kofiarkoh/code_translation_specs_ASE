#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MOD 1000000007LL

typedef struct {
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

static int64_t *fact;
static int64_t *invfact;

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

    head = (int*)malloc((size_t)(n + 1) * sizeof(int));
    edges = (Edge*)malloc((size_t)(2 * (n - 1) + 5) * sizeof(Edge));
    parent = (int*)malloc((size_t)(n + 1) * sizeof(int));
    order = (int*)malloc((size_t)(n + 1) * sizeof(int));
    subsz = (int*)malloc((size_t)(n + 1) * sizeof(int));
    fact = (int64_t*)malloc((size_t)(n + 1) * sizeof(int64_t));
    invfact = (int64_t*)malloc((size_t)(n + 1) * sizeof(int64_t));

    for (int i = 1; i <= n; i++) head[i] = -1;
    ecnt = 0;

    for (int i = 0; i < n - 1; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        add_edge(u, v);
        add_edge(v, u);
    }

    /* Precompute factorials and inverse factorials modulo prime MOD. */
    fact[0] = 1;
    for (int i = 1; i <= n; i++) fact[i] = (fact[i - 1] * (int64_t)i) % MOD;
    invfact[n] = mod_pow(fact[n], MOD - 2);
    for (int i = n; i >= 1; i--) invfact[i - 1] = (invfact[i] * (int64_t)i) % MOD;

    /* Build parent[] and a traversal order without recursion (stack-based DFS). */
    int *stack = (int*)malloc((size_t)(n + 1) * sizeof(int));
    int top = 0, ordsz = 0;

    parent[1] = 0;
    stack[top++] = 1;
    while (top) {
        int u = stack[--top];
        order[ordsz++] = u;
        for (int e = head[u]; e != -1; e = edges[e].next) {
            int v = edges[e].to;
            if (v == parent[u]) continue;
            parent[v] = u;
            stack[top++] = v;
        }
    }

    /* Compute subtree sizes in reverse order. */
    for (int i = 1; i <= n; i++) subsz[i] = 1;
    for (int i = ordsz - 1; i >= 0; i--) {
        int u = order[i];
        for (int e = head[u]; e != -1; e = edges[e].next) {
            int v = edges[e].to;
            if (v == parent[u]) continue;
            subsz[u] += subsz[v];
        }
    }

    /*
      For each vertex v, components after removing v correspond to:
        - each child subtree size subsz[child]
        - the "rest of tree" size n - subsz[v] (if v is not the root)
      Output: n! / prod(|C|!) mod MOD = fact[n] * prod(invfact[|C|]) mod MOD.
    */
    for (int v = 1; v <= n; v++) {
        int64_t ans = fact[n];
        for (int e = head[v]; e != -1; e = edges[e].next) {
            int u = edges[e].to;
            int comp;
            if (u == parent[v]) {
                comp = n - subsz[v];
            } else {
                comp = subsz[u];
            }
            if (comp > 0) ans = (ans * invfact[comp]) % MOD;
        }
        printf("%lld\n", (long long)ans);
    }

    free(stack);
    free(invfact);
    free(fact);
    free(subsz);
    free(order);
    free(parent);
    free(edges);
    free(head);
    return 0;
}
