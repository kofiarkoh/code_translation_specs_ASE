#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int to;
    int next;
} Edge;

typedef struct {
    int u, v;
} InputEdge;

typedef struct {
    int a, b;
    int idx;
} PairItem;

static int cmp_pairitem(const void *pa, const void *pb) {
    const PairItem *x = (const PairItem *)pa;
    const PairItem *y = (const PairItem *)pb;
    if (x->a != y->a) return (x->a < y->a) ? -1 : 1;
    if (x->b != y->b) return (x->b < y->b) ? -1 : 1;
    return (x->idx < y->idx) ? -1 : (x->idx > y->idx);
}

/* Iterative Kosaraju: first pass order on original graph, second pass on reversed graph. */
static void kosaraju_scc(int n,
                         const int *head, const Edge *E,
                         const int *rhead, const Edge *RE,
                         int *comp, int *comp_cnt) {
    int *vis = (int *)calloc((size_t)n, sizeof(int));
    int *order = (int *)malloc((size_t)n * sizeof(int));
    int ord_sz = 0;

    int *stack_v = (int *)malloc((size_t)n * sizeof(int));
    int *stack_ei = (int *)malloc((size_t)n * sizeof(int));
    int sp;

    for (int s = 0; s < n; s++) {
        if (vis[s]) continue;

        sp = 0;
        stack_v[sp] = s;
        stack_ei[sp] = head[s];
        vis[s] = 1;
        sp++;

        while (sp > 0) {
            int v = stack_v[sp - 1];
            int ei = stack_ei[sp - 1];

            if (ei == -1) {
                order[ord_sz++] = v;
                sp--;
                continue;
            }

            stack_ei[sp - 1] = E[ei].next;
            int to = E[ei].to;
            if (!vis[to]) {
                vis[to] = 1;
                stack_v[sp] = to;
                stack_ei[sp] = head[to];
                sp++;
            }
        }
    }

    memset(vis, 0, (size_t)n * sizeof(int));
    int cid = 0;

    int *q = (int *)malloc((size_t)n * sizeof(int));
    for (int i = n - 1; i >= 0; i--) {
        int s = order[i];
        if (vis[s]) continue;

        int qh = 0, qt = 0;
        q[qt++] = s;
        vis[s] = 1;
        comp[s] = cid;

        while (qh < qt) {
            int v = q[qh++];
            for (int ei = rhead[v]; ei != -1; ei = RE[ei].next) {
                int to = RE[ei].to;
                if (!vis[to]) {
                    vis[to] = 1;
                    comp[to] = cid;
                    q[qt++] = to;
                }
            }
        }
        cid++;
    }

    *comp_cnt = cid;

    free(q);
    free(stack_v);
    free(stack_ei);
    free(order);
    free(vis);
}

/* Build condensation DAG adjacency (unique arcs) and also count multiplicities per (S,T). */
static void build_condensation(int m, const InputEdge *in, const int *comp, int comp_cnt,
                               int **out_head, Edge **out_E, int *out_ecnt,
                               int *pair_is_multi /* size m, 1 if its (S,T) has multiplicity>1 */) {
    PairItem *items = (PairItem *)malloc((size_t)m * sizeof(PairItem));
    int k = 0;
    for (int i = 0; i < m; i++) {
        int s = comp[in[i].u];
        int t = comp[in[i].v];
        if (s == t) {
            items[i].a = -1;
            items[i].b = -1;
            items[i].idx = i;
            continue;
        }
        items[i].a = s;
        items[i].b = t;
        items[i].idx = i;
        k++;
    }

    /* Sort all m items; intra-SCC have (-1,-1) and will group together but ignored. */
    qsort(items, (size_t)m, sizeof(PairItem), cmp_pairitem);

    /* Mark multiplicity>1 for each (S,T) group among inter-SCC edges. */
    for (int i = 0; i < m; ) {
        int a = items[i].a, b = items[i].b;
        int j = i + 1;
        while (j < m && items[j].a == a && items[j].b == b) j++;
        if (a != -1) {
            int cnt = j - i;
            if (cnt > 1) {
                for (int p = i; p < j; p++) pair_is_multi[items[p].idx] = 1;
            }
        }
        i = j;
    }

    /* Build unique condensation arcs from sorted groups. */
    int *head = (int *)malloc((size_t)comp_cnt * sizeof(int));
    for (int i = 0; i < comp_cnt; i++) head[i] = -1;

    /* Upper bound on unique arcs is m. */
    Edge *CE = (Edge *)malloc((size_t)m * sizeof(Edge));
    int ecnt = 0;

    for (int i = 0; i < m; ) {
        int a = items[i].a, b = items[i].b;
        int j = i + 1;
        while (j < m && items[j].a == a && items[j].b == b) j++;
        if (a != -1) {
            /* Add one arc a->b */
            CE[ecnt].to = b;
            CE[ecnt].next = head[a];
            head[a] = ecnt;
            ecnt++;
        }
        i = j;
    }

    free(items);

    *out_head = head;
    *out_E = CE;
    *out_ecnt = ecnt;
}

/* Compute reachability in condensation DAG: reach[s][t]=1 if t reachable from s. */
static void condensation_reachability(int comp_cnt, const int *head, const Edge *E,
                                      unsigned char *reach /* comp_cnt*comp_cnt */) {
    /* BFS/DFS from each source; comp_cnt <= n <= 1000 so O(C*(C+E)) is fine. */
    int *q = (int *)malloc((size_t)comp_cnt * sizeof(int));
    unsigned char *vis = (unsigned char *)malloc((size_t)comp_cnt);

    for (int s = 0; s < comp_cnt; s++) {
        memset(vis, 0, (size_t)comp_cnt);
        int qh = 0, qt = 0;
        vis[s] = 1;
        q[qt++] = s;

        while (qh < qt) {
            int v = q[qh++];
            for (int ei = head[v]; ei != -1; ei = E[ei].next) {
                int to = E[ei].to;
                if (!vis[to]) {
                    vis[to] = 1;
                    q[qt++] = to;
                }
            }
        }

        for (int t = 0; t < comp_cnt; t++) {
            reach[(size_t)s * (size_t)comp_cnt + (size_t)t] = vis[t];
        }
    }

    free(vis);
    free(q);
}

/* Intra-SCC edge classification:
   For each SCC, compute dominators on the SCC subgraph (with possible multi-edges).
   An edge u->v is "same" iff it is the only alternative-free way: i.e., u dominates v in SCC rooted at u.
   We compute dominators per SCC per chosen root u, but to keep it tractable, we compute dominators once per SCC
   for each possible root that appears as a source of an intra-SCC edge. With n<=1000 this is acceptable. */
static void classify_intra_scc(int n, int m,
                               const InputEdge *in,
                               const int *head, const Edge *E,
                               const int *comp, int comp_cnt,
                               char *ans /* 's' for same, 'd' for diff, only for intra edges here */) {
    /* Build vertex lists per SCC. */
    int *sz = (int *)calloc((size_t)comp_cnt, sizeof(int));
    for (int v = 0; v < n; v++) sz[comp[v]]++;

    int **verts = (int **)malloc((size_t)comp_cnt * sizeof(int *));
    int *pos = (int *)calloc((size_t)comp_cnt, sizeof(int));
    for (int c = 0; c < comp_cnt; c++) verts[c] = (int *)malloc((size_t)sz[c] * sizeof(int));
    for (int v = 0; v < n; v++) {
        int c = comp[v];
        verts[c][pos[c]++] = v;
    }

    /* Map global vertex -> local index within its SCC for bitset operations. */
    int *local = (int *)malloc((size_t)n * sizeof(int));
    for (int c = 0; c < comp_cnt; c++) {
        for (int i = 0; i < sz[c]; i++) local[verts[c][i]] = i;
    }

    /* For each SCC, precompute predecessor lists within SCC (as global vertices). */
    int **pred = (int **)malloc((size_t)n * sizeof(int *));
    int *pred_sz = (int *)calloc((size_t)n, sizeof(int));
    int *pred_cap = (int *)calloc((size_t)n, sizeof(int));

    for (int u = 0; u < n; u++) {
        for (int ei = head[u]; ei != -1; ei = E[ei].next) {
            int v = E[ei].to;
            if (comp[u] == comp[v]) {
                if (pred_sz[v] == pred_cap[v]) {
                    int nc = pred_cap[v] ? pred_cap[v] * 2 : 4;
                    int *np = (int *)realloc(pred[v], (size_t)nc * sizeof(int));
                    if (!np) exit(1);
                    pred[v] = np;
                    pred_cap[v] = nc;
                }
                pred[v][pred_sz[v]++] = u;
            }
        }
    }

    /* Determine which roots (sources) we need per SCC: those u that have at least one intra edge. */
    unsigned char *need_root = (unsigned char *)calloc((size_t)n, 1);
    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        if (comp[u] == comp[v]) need_root[u] = 1;
    }

    /* Bitset helpers. */
    /* Use 64-bit blocks. */
    for (int c = 0; c < comp_cnt; c++) {
        int k = sz[c];
        if (k <= 1) {
            /* Single-vertex SCC: any self-loop u->u is diff iff there is an alternative path u->u without that edge.
               In a 1-vertex SCC, alternative path exists iff there is another self-loop edge (parallel) or a cycle via other vertices (none).
               So only parallel self-loops make diff; otherwise same. We'll handle via dominator logic below too, but k=1 is special. */
        }

        /* Collect roots in this SCC. */
        int root_count = 0;
        for (int i = 0; i < k; i++) {
            int u = verts[c][i];
            if (need_root[u]) root_count++;
        }
        if (root_count == 0) continue;

        int blocks = (k + 63) / 64;
        unsigned long long *dom = (unsigned long long *)malloc((size_t)k * (size_t)blocks * sizeof(unsigned long long));
        unsigned long long *tmp = (unsigned long long *)malloc((size_t)blocks * sizeof(unsigned long long));
        unsigned char *reachable = (unsigned char *)malloc((size_t)k);

        for (int ri = 0; ri < k; ri++) {
            int r = verts[c][ri];
            if (!need_root[r]) continue;

            /* Compute reachable from r within SCC. */
            memset(reachable, 0, (size_t)k);
            int *q = (int *)malloc((size_t)k * sizeof(int));
            int qh = 0, qt = 0;
            reachable[ri] = 1;
            q[qt++] = r;
            while (qh < qt) {
                int x = q[qh++];
                for (int ei = head[x]; ei != -1; ei = E[ei].next) {
                    int y = E[ei].to;
                    if (comp[y] != c) continue;
                    int ly = local[y];
                    if (!reachable[ly]) {
                        reachable[ly] = 1;
                        q[qt++] = y;
                    }
                }
            }
            free(q);

            /* Initialize dominator sets:
               dom[r]= {r}; for other reachable nodes: all reachable nodes; for unreachable: empty. */
            for (int i = 0; i < k; i++) {
                unsigned long long *D = dom + (size_t)i * (size_t)blocks;
                if (!reachable[i]) {
                    for (int b = 0; b < blocks; b++) D[b] = 0ULL;
                } else if (i == ri) {
                    for (int b = 0; b < blocks; b++) D[b] = 0ULL;
                    D[ri / 64] |= 1ULL << (ri % 64);
                } else {
                    /* all reachable */
                    for (int b = 0; b < blocks; b++) D[b] = 0ULL;
                    for (int j = 0; j < k; j++) {
                        if (reachable[j]) {
                            D[j / 64] |= 1ULL << (j % 64);
                        }
                    }
                }
            }

            /* Iterative refinement. */
            int changed;
            do {
                changed = 0;
                for (int i = 0; i < k; i++) {
                    if (!reachable[i] || i == ri) continue;
                    int v = verts[c][i];

                    /* tmp = intersection of dom[p] over predecessors p of v that are reachable */
                    int first = 1;
                    for (int b = 0; b < blocks; b++) tmp[b] = 0ULL;

                    for (int pi = 0; pi < pred_sz[v]; pi++) {
                        int p = pred[v][pi];
                        if (comp[p] != c) continue;
                        int lp = local[p];
                        if (!reachable[lp]) continue;
                        unsigned long long *Dp = dom + (size_t)lp * (size_t)blocks;
                        if (first) {
                            for (int b = 0; b < blocks; b++) tmp[b] = Dp[b];
                            first = 0;
                        } else {
                            for (int b = 0; b < blocks; b++) tmp[b] &= Dp[b];
                        }
                    }

                    if (first) {
                        /* No reachable predecessors: intersection over empty set -> empty, then add self. */
                        for (int b = 0; b < blocks; b++) tmp[b] = 0ULL;
                    }

                    /* newDom = tmp U {i} */
                    tmp[i / 64] |= 1ULL << (i % 64);

                    unsigned long long *Dv = dom + (size_t)i * (size_t)blocks;
                    int diff = 0;
                    for (int b = 0; b < blocks; b++) {
                        if (Dv[b] != tmp[b]) { diff = 1; break; }
                    }
                    if (diff) {
                        for (int b = 0; b < blocks; b++) Dv[b] = tmp[b];
                        changed = 1;
                    }
                }
            } while (changed);

            /* For each intra edge u->v with u==r, decide:
               If v is reachable and u dominates v (i.e., u in dom[v]) then no alternative path avoiding edge? Actually:
               Edge is "diff" if there exists an alternative path u->v not using that edge.
               In a directed graph, edge u->v is a bridge-like edge for reachability from u to v iff u dominates v AND
               the edge is the only incoming from nodes reachable without passing through v? This is subtle.
               Narrowest faithful interpretation: check existence of alternative path excluding the edge directly.
               We'll do per edge BFS within SCC excluding that single edge, but only for intra edges; n<=1000 so OK. */
            (void)dom; /* dominator computed but not used due to subtlety; keep code simple and faithful. */

            /* Per-edge BFS excluding the edge (only for edges with source r). */
            for (int ei_idx = 0; ei_idx < m; ei_idx++) {
                int u = in[ei_idx].u, v = in[ei_idx].v;
                if (u != r) continue;
                if (comp[u] != c || comp[v] != c) continue;

                /* BFS from u to v within SCC excluding this specific edge occurrence. */
                unsigned char *seen = (unsigned char *)calloc((size_t)k, 1);
                int *qq = (int *)malloc((size_t)k * sizeof(int));
                int h = 0, t = 0;
                int lu = local[u], lv = local[v];
                seen[lu] = 1;
                qq[t++] = u;

                int found = 0;
                while (h < t && !found) {
                    int x = qq[h++];
                    for (int eix = head[x]; eix != -1; eix = E[eix].next) {
                        int y = E[eix].to;
                        if (comp[y] != c) continue;
                        /* Exclude only the edge occurrence being classified: match by endpoints and input index.
                           Since adjacency list edges are built from input in order, we cannot map eix->input index here.
                           Therefore, we exclude by (x==u && y==v) only for this edge, which would incorrectly exclude
                           parallel edges. To preserve semantics with parallel edges, we must exclude exactly one occurrence.
                           We handle this by counting how many times we've skipped (u->v) and skip only once. */
                    }
                }

                /* Re-run BFS with correct single-occurrence exclusion by scanning input edges for adjacency:
                   Build per-vertex outgoing list of input edge indices within SCC on the fly is too heavy.
                   Instead, we pre-built adjacency from input; we need mapping from adjacency edge to input index.
                   So we cannot proceed with the above approach; implement adjacency with input index mapping. */

                free(qq);
                free(seen);

                /* Mark as unknown for now; will be set in second pass with proper adjacency. */
                ans[ei_idx] = 0;
            }
        }

        free(reachable);
        free(tmp);
        free(dom);
    }

    /* Cleanup */
    for (int v = 0; v < n; v++) free(pred[v]);
    free(pred);
    free(pred_sz);
    free(pred_cap);
    for (int c = 0; c < comp_cnt; c++) free(verts[c]);
    free(verts);
    free(pos);
    free(sz);
    free(local);
    free(need_root);
}

/* Build adjacency with input-edge index mapping for exact exclusion of one edge occurrence. */
typedef struct {
    int to;
    int next;
    int idx; /* input edge index */
} EdgeI;

static void build_graph_with_indices(int n, int m, const InputEdge *in,
                                     int **out_head, EdgeI **out_E,
                                     int **out_rhead, EdgeI **out_RE) {
    int *head = (int *)malloc((size_t)n * sizeof(int));
    int *rhead = (int *)malloc((size_t)n * sizeof(int));
    for (int i = 0; i < n; i++) head[i] = rhead[i] = -1;

    EdgeI *E = (EdgeI *)malloc((size_t)m * sizeof(EdgeI));
    EdgeI *RE = (EdgeI *)malloc((size_t)m * sizeof(EdgeI));

    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        E[i].to = v;
        E[i].idx = i;
        E[i].next = head[u];
        head[u] = i;

        RE[i].to = u;
        RE[i].idx = i;
        RE[i].next = rhead[v];
        rhead[v] = i;
    }

    *out_head = head;
    *out_E = E;
    *out_rhead = rhead;
    *out_RE = RE;
}

static void classify_intra_exact(int n, int m, const InputEdge *in,
                                 const int *head, const EdgeI *E,
                                 const int *comp,
                                 char *out /* 's' or 'd' for intra edges; untouched for inter */) {
    /* For each intra edge, BFS within SCC excluding that edge index. n<=1000, m<=200000. */
    int *q = (int *)malloc((size_t)n * sizeof(int));
    unsigned char *vis = (unsigned char *)malloc((size_t)n);

    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        if (comp[u] != comp[v]) continue;

        memset(vis, 0, (size_t)n);
        int qh = 0, qt = 0;
        vis[u] = 1;
        q[qt++] = u;

        int found = 0;
        while (qh < qt && !found) {
            int x = q[qh++];
            for (int ei = head[x]; ei != -1; ei = E[ei].next) {
                if (E[ei].idx == i) continue; /* exclude exactly this edge occurrence */
                int y = E[ei].to;
                if (comp[y] != comp[u]) continue; /* stay within SCC */
                if (!vis[y]) {
                    vis[y] = 1;
                    if (y == v) { found = 1; break; }
                    q[qt++] = y;
                }
            }
        }

        out[i] = found ? 'd' : 's';
    }

    free(vis);
    free(q);
}

int main(void) {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) return 0;

    InputEdge *in = (InputEdge *)malloc((size_t)m * sizeof(InputEdge));
    for (int i = 0; i < m; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        in[i].u = u - 1;
        in[i].v = v - 1;
    }

    /* Build simple graph for SCC (no need for edge indices). */
    int *head = (int *)malloc((size_t)n * sizeof(int));
    int *rhead = (int *)malloc((size_t)n * sizeof(int));
    for (int i = 0; i < n; i++) head[i] = rhead[i] = -1;

    Edge *E = (Edge *)malloc((size_t)m * sizeof(Edge));
    Edge *RE = (Edge *)malloc((size_t)m * sizeof(Edge));

    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        E[i].to = v;
        E[i].next = head[u];
        head[u] = i;

        RE[i].to = u;
        RE[i].next = rhead[v];
        rhead[v] = i;
    }

    int *comp = (int *)malloc((size_t)n * sizeof(int));
    int comp_cnt = 0;
    kosaraju_scc(n, head, E, rhead, RE, comp, &comp_cnt);

    /* Inter-SCC: multiplicity and condensation reachability. */
    int *pair_is_multi = (int *)calloc((size_t)m, sizeof(int));
    int *chead = NULL;
    Edge *CE = NULL;
    int cecnt = 0;
    build_condensation(m, in, comp, comp_cnt, &chead, &CE, &cecnt, pair_is_multi);

    unsigned char *reach = (unsigned char *)malloc((size_t)comp_cnt * (size_t)comp_cnt);
    condensation_reachability(comp_cnt, chead, CE, reach);

    char *ans = (char *)malloc((size_t)m);
    for (int i = 0; i < m; i++) ans[i] = 0;

    /* Inter-SCC classification. */
    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        int s = comp[u], t = comp[v];
        if (s == t) continue;

        int diff = 0;
        if (pair_is_multi[i]) diff = 1;
        else {
            /* longer route exists iff there is some intermediate k with s->k and k->t */
            for (int k = 0; k < comp_cnt; k++) {
                if (k == s || k == t) continue;
                if (reach[(size_t)s * (size_t)comp_cnt + (size_t)k] &&
                    reach[(size_t)k * (size_t)comp_cnt + (size_t)t]) {
                    diff = 1;
                    break;
                }
            }
        }
        ans[i] = diff ? 'd' : 's';
    }

    /* Intra-SCC classification with exact edge exclusion. */
    int *headI = NULL, *rheadI = NULL;
    EdgeI *EI = NULL, *REI = NULL;
    build_graph_with_indices(n, m, in, &headI, &EI, &rheadI, &REI);
    (void)rheadI; (void)REI; /* not needed further */
    classify_intra_exact(n, m, in, headI, EI, comp, ans);

    for (int i = 0; i < m; i++) {
        if (ans[i] == 's') {
            puts("same");
        } else {
            puts("diff");
        }
    }

    free(headI);
    free(EI);
    free(rheadI);
    free(REI);

    free(ans);
    free(reach);
    free(CE);
    free(chead);
    free(pair_is_multi);
    free(comp);
    free(E);
    free(RE);
    free(head);
    free(rhead);
    free(in);
    return 0;
}
