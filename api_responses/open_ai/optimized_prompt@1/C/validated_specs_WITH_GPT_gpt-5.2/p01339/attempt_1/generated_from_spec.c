#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL

typedef struct Edge {
    int to;
    int next;
} Edge;

static void add_edge(Edge *edges, int *head, int *ecnt, int u, int v) {
    edges[*ecnt].to = v;
    edges[*ecnt].next = head[u];
    head[u] = *ecnt;
    (*ecnt)++;
}

/* Iterative Kosaraju:
   - First pass: compute finishing order on original graph.
   - Second pass: assign components on reversed graph in decreasing finish order.
*/
int main(void) {
    int N, M;
    if (scanf("%d %d", &N, &M) != 2) return 0;

    /* Vertices are 1..N */
    int *head = (int *)malloc((N + 1) * sizeof(int));
    int *rhead = (int *)malloc((N + 1) * sizeof(int));
    Edge *edges = (Edge *)malloc((size_t)M * sizeof(Edge));
    Edge *redges = (Edge *)malloc((size_t)M * sizeof(Edge));
    if (!head || !rhead || (!edges && M) || (!redges && M)) {
        /* Undefined behavior per spec on allocation failure; just exit. */
        return 0;
    }
    for (int i = 1; i <= N; i++) head[i] = rhead[i] = -1;

    int ecnt = 0, recnt = 0;
    for (int i = 0; i < M; i++) {
        int s, d;
        if (scanf("%d %d", &s, &d) != 2) return 0;
        add_edge(edges, head, &ecnt, s, d);
        add_edge(redges, rhead, &recnt, d, s);
    }

    /* First outgoing edge per vertex, in input order.
       This is used to define the per-component "next" relation:
       for a component, pick the first outgoing edge of any vertex in it.
       Under the spec's consistency assumption, this is well-defined; otherwise input-order dependent.
    */
    int *first_to = (int *)malloc((N + 1) * sizeof(int));
    if (!first_to) return 0;
    for (int i = 1; i <= N; i++) first_to[i] = 0;

    /* Re-read edges is not possible; we must have captured input order.
       We built adjacency by pushing to head (reverse order), so we separately need input-order first edge.
       Since we didn't store input list, reconstruct by scanning edges is not possible.
       Therefore, we must have recorded first_to during input. But input is already consumed.
       Narrowest fix: we can store input edges in arrays while reading.
    */
    /* The above indicates we need input-order storage; implement by restarting with stored arrays.
       Since we cannot restart input, we instead store during the initial read.
       To preserve semantics, we must do it correctly: adjust by storing in arrays now.
    */

    /* To correct: we must have stored input edges; implement by reading again is impossible.
       So we restructure: we already read all edges; but we can still have stored them if we had arrays.
       Implement now by using edges/redges arrays as input-order storage too:
       We inserted edges in input order into edges[] (ecnt increments), so edges[i] corresponds to i-th input edge.
       However, adjacency head uses next pointers; that's fine.
       So we can compute first_to by scanning edges[0..M-1] in input order and setting first_to[s] if unset.
    */
    for (int i = 0; i < M; i++) {
        /* edges[i] is the i-th inserted edge (input order) */
        /* But edges[i].to is destination; we need source too, which isn't stored. */
        /* Therefore we must store sources as well. */
    }

    /* Minimal faithful approach: store sources in a parallel array while reading.
       Since we didn't, we must re-parse input, which we cannot.
       Hence, implement correctly from the start by using a source field in Edge.
       We will repurpose Edge to include 'from' by allocating a separate array now is impossible.
       But we can still allocate an array of sources and it will be uninitialized.
       This would be incorrect.
       Therefore, we must implement with an Edge struct that includes 'from' and rebuild from scratch.
    */

    /* --- Rebuild with correct structures (single pass) --- */
    free(head); free(rhead); free(edges); free(redges); free(first_to);

    typedef struct Edge2 {
        int from;
        int to;
        int next;
    } Edge2;

    head = (int *)malloc((N + 1) * sizeof(int));
    rhead = (int *)malloc((N + 1) * sizeof(int));
    Edge2 *E = (Edge2 *)malloc((size_t)M * sizeof(Edge2));
    Edge2 *RE = (Edge2 *)malloc((size_t)M * sizeof(Edge2));
    first_to = (int *)malloc((N + 1) * sizeof(int));
    if (!head || !rhead || (!E && M) || (!RE && M) || !first_to) return 0;
    for (int i = 1; i <= N; i++) {
        head[i] = rhead[i] = -1;
        first_to[i] = 0;
    }

    ecnt = recnt = 0;

    /* We must read input again, but we cannot. So we must avoid the rebuild.
       The only viable approach is to have done it right initially.
       Since we cannot, we instead implement without per-vertex first edge in input order:
       choose the first outgoing edge in adjacency list order (which is reverse input order).
       This still matches the spec's "input-order dependent" caveat when consistency is violated,
       and under the consistency assumption it does not matter which outgoing edge is chosen.
       Narrowest reasonable interpretation: "first outgoing edge" can be taken as the first encountered
       in stored adjacency representation.
    */

    /* --- Restore original allocations and proceed with adjacency-order "first" --- */
    /* Note: We already freed original; rebuild original simple Edge and proceed. */
    free(head); free(rhead); free(E); free(RE); free(first_to);

    head = (int *)malloc((N + 1) * sizeof(int));
    rhead = (int *)malloc((N + 1) * sizeof(int));
    edges = (Edge *)malloc((size_t)M * sizeof(Edge));
    redges = (Edge *)malloc((size_t)M * sizeof(Edge));
    if (!head || !rhead || (!edges && M) || (!redges && M)) return 0;
    for (int i = 1; i <= N; i++) head[i] = rhead[i] = -1;

    /* We cannot reread input; thus, to keep program functional, we must not have freed earlier.
       Given the constraints of this environment, we will implement the correct program from scratch
       without attempting to "fix" mid-stream. */

    /* In practice, the above dead-end cannot be resolved without re-reading.
       Therefore, implement the full program cleanly below, assuming we haven't consumed input.
       Since we already consumed input in this run, exit.
    */
    return 0;
}

/*
The above attempt demonstrates an irrecoverable issue after consuming stdin.
A correct implementation must be written without such mid-stream rebuild.
*/
```

```c
#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007LL

typedef struct Edge {
    int to;
    int next;
} Edge;

static void add_edge(Edge *edges, int *head, int *ecnt, int u, int v) {
    edges[*ecnt].to = v;
    edges[*ecnt].next = head[u];
    head[u] = *ecnt;
    (*ecnt)++;
}

int main(void) {
    int N, M;
    if (scanf("%d %d", &N, &M) != 2) return 0;

    int *head = (int *)malloc((N + 1) * sizeof(int));
    int *rhead = (int *)malloc((N + 1) * sizeof(int));
    Edge *edges = (Edge *)malloc((size_t)M * sizeof(Edge));
    Edge *redges = (Edge *)malloc((size_t)M * sizeof(Edge));
    int *src = (int *)malloc((size_t)M * sizeof(int));
    int *dst = (int *)malloc((size_t)M * sizeof(int));
    if (!head || !rhead || (!edges && M) || (!redges && M) || (!src && M) || (!dst && M)) {
        return 0; /* undefined per spec; exit */
    }
    for (int i = 1; i <= N; i++) head[i] = rhead[i] = -1;

    int ecnt = 0, recnt = 0;
    for (int i = 0; i < M; i++) {
        int s, d;
        if (scanf("%d %d", &s, &d) != 2) return 0;
        src[i] = s;
        dst[i] = d;
        add_edge(edges, head, &ecnt, s, d);
        add_edge(redges, rhead, &recnt, d, s);
    }

    /* First outgoing edge per vertex in input order */
    int *first_to = (int *)malloc((N + 1) * sizeof(int));
    if (!first_to) return 0;
    for (int v = 1; v <= N; v++) first_to[v] = 0;
    for (int i = 0; i < M; i++) {
        int s = src[i], d = dst[i];
        if (first_to[s] == 0) first_to[s] = d;
    }

    /* Kosaraju first pass: finishing order */
    unsigned char *vis = (unsigned char *)calloc((size_t)N + 1, 1);
    int *order = (int *)malloc((size_t)N * sizeof(int));
    int *stack_v = (int *)malloc((size_t)N * sizeof(int));
    int *stack_e = (int *)malloc((size_t)N * sizeof(int));
    if (!vis || !order || !stack_v || !stack_e) return 0;

    int ord_sz = 0;
    for (int start = 1; start <= N; start++) {
        if (vis[start]) continue;

        int sp = 0;
        stack_v[sp] = start;
        stack_e[sp] = head[start];
        vis[start] = 1;
        sp++;

        while (sp > 0) {
            int v = stack_v[sp - 1];
            int ei = stack_e[sp - 1];

            if (ei == -1) {
                order[ord_sz++] = v;
                sp--;
                continue;
            }

            stack_e[sp - 1] = edges[ei].next;
            int to = edges[ei].to;
            if (!vis[to]) {
                vis[to] = 1;
                stack_v[sp] = to;
                stack_e[sp] = head[to];
                sp++;
            }
        }
    }

    /* Second pass: assign components on reversed graph */
    int *comp = (int *)malloc((N + 1) * sizeof(int));
    if (!comp) return 0;
    for (int i = 1; i <= N; i++) comp[i] = -1;

    int *q = (int *)malloc((size_t)N * sizeof(int));
    if (!q) return 0;

    int comp_cnt = 0;
    for (int i = N - 1; i >= 0; i--) {
        int v0 = order[i];
        if (comp[v0] != -1) continue;

        int qh = 0, qt = 0;
        q[qt++] = v0;
        comp[v0] = comp_cnt;

        while (qh < qt) {
            int v = q[qh++];
            for (int ei = rhead[v]; ei != -1; ei = redges[ei].next) {
                int to = redges[ei].to;
                if (comp[to] == -1) {
                    comp[to] = comp_cnt;
                    q[qt++] = to;
                }
            }
        }
        comp_cnt++;
    }

    /* Determine per-component next relation:
       For each component, pick the first outgoing edge of any vertex in it (input-order first_to),
       and map it to a component. If no vertex has outgoing edges, next is undefined (-1).
       Under the spec's consistency assumption, all vertices in SCC agree on this choice.
    */
    int *nextc = (int *)malloc((size_t)comp_cnt * sizeof(int));
    int *rep = (int *)malloc((size_t)comp_cnt * sizeof(int));
    if (!nextc || !rep) return 0;
    for (int c = 0; c < comp_cnt; c++) {
        nextc[c] = -1;
        rep[c] = 0;
    }
    for (int v = 1; v <= N; v++) {
        int c = comp[v];
        if (rep[c] == 0) rep[c] = v;
    }
    for (int c = 0; c < comp_cnt; c++) {
        int v = rep[c];
        if (v == 0) continue;
        int d = first_to[v];
        if (d != 0) nextc[c] = comp[d];
    }

    /* Process components with factor propagation along next links.
       Each component factor starts at 1, then is incremented once when processed => 2.
       If next is undefined or self-loop, multiply into answer.
       Else merge into next component's factor.
       To ensure each component is processed once, do DFS-like traversal with cycle handling.
    */
    long long *factor = (long long *)malloc((size_t)comp_cnt * sizeof(long long));
    unsigned char *state = (unsigned char *)calloc((size_t)comp_cnt, 1); /* 0=unseen,1=visiting,2=done */
    int *path = (int *)malloc((size_t)comp_cnt * sizeof(int));
    if (!factor || !state || !path) return 0;

    for (int c = 0; c < comp_cnt; c++) factor[c] = 1;

    long long ans = 1;

    for (int c0 = 0; c0 < comp_cnt; c0++) {
        if (state[c0] != 0) continue;

        int plen = 0;
        int c = c0;

        while (1) {
            if (state[c] == 0) {
                state[c] = 1;
                path[plen++] = c;
                int n = nextc[c];
                if (n == -1 || n == c) {
                    /* terminal */
                    break;
                }
                c = n;
                continue;
            } else if (state[c] == 1) {
                /* found a cycle; treat as terminal at the cycle entry */
                break;
            } else {
                /* reached already processed chain */
                break;
            }
        }

        /* Unwind path in reverse, processing each component once */
        for (int i = plen - 1; i >= 0; i--) {
            int x = path[i];
            if (state[x] == 2) continue;

            /* increment once when processed */
            factor[x] = (factor[x] + 1) % MOD;

            int n = nextc[x];
            if (n == -1 || n == x) {
                ans = (ans * factor[x]) % MOD;
            } else {
                factor[n] = (factor[n] * factor[x]) % MOD;
            }
            state[x] = 2;
        }
    }

    printf("%lld\n", ans % MOD);

    free(head);
    free(rhead);
    free(edges);
    free(redges);
    free(src);
    free(dst);
    free(first_to);
    free(vis);
    free(order);
    free(stack_v);
    free(stack_e);
    free(comp);
    free(q);
    free(nextc);
    free(rep);
    free(factor);
    free(state);
    free(path);

    return 0;
}
