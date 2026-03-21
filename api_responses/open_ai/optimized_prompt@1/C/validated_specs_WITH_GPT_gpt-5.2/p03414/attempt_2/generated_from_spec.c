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

static void *xmalloc(size_t n) {
    void *p = malloc(n);
    if (!p) {
        /* Unspecified behavior per spec; but avoid UB on OOM. */
        fprintf(stderr, "out of memory\n");
        exit(1);
    }
    return p;
}

static void *xcalloc(size_t n, size_t sz) {
    void *p = calloc(n, sz);
    if (!p) {
        fprintf(stderr, "out of memory\n");
        exit(1);
    }
    return p;
}

/* Kosaraju SCC: iterative DFS to avoid recursion depth issues. */
static void kosaraju_scc(int n, int m,
                         const int *head, const Edge *E,
                         const int *rhead, const Edge *RE,
                         int *comp, int *comp_cnt) {
    int *vis = (int *)xcalloc((size_t)n, sizeof(int));
    int *order = (int *)xmalloc((size_t)n * sizeof(int));
    int ord_sz = 0;

    /* Stack for iterative DFS with explicit iterator state. */
    int *st_v = (int *)xmalloc((size_t)n * sizeof(int));
    int *st_ei = (int *)xmalloc((size_t)n * sizeof(int));
    int sp;

    for (int s = 0; s < n; s++) {
        if (vis[s]) continue;

        sp = 0;
        st_v[sp] = s;
        st_ei[sp] = head[s];
        vis[s] = 1;
        sp++;

        while (sp > 0) {
            int v = st_v[sp - 1];
            int ei = st_ei[sp - 1];

            if (ei == -1) {
                /* finished v */
                order[ord_sz++] = v;
                sp--;
                continue;
            }

            /* advance iterator */
            st_ei[sp - 1] = E[ei].next;
            int to = E[ei].to;
            if (!vis[to]) {
                vis[to] = 1;
                st_v[sp] = to;
                st_ei[sp] = head[to];
                sp++;
            }
        }
    }

    for (int i = 0; i < n; i++) comp[i] = -1;
    int cid = 0;

    /* Second pass on reversed graph in reverse finishing order. */
    int *q = (int *)xmalloc((size_t)n * sizeof(int));
    for (int i = n - 1; i >= 0; i--) {
        int s = order[i];
        if (comp[s] != -1) continue;

        int qh = 0, qt = 0;
        q[qt++] = s;
        comp[s] = cid;

        while (qh < qt) {
            int v = q[qh++];
            for (int ei = rhead[v]; ei != -1; ei = RE[ei].next) {
                int to = RE[ei].to;
                if (comp[to] == -1) {
                    comp[to] = cid;
                    q[qt++] = to;
                }
            }
        }
        cid++;
    }

    *comp_cnt = cid;

    free(vis);
    free(order);
    free(st_v);
    free(st_ei);
    free(q);
}

/* Build SCC-local adjacency (only edges within SCC), with edge ids. */
typedef struct {
    int to;
    int id;
    int next;
} LEdge;

static void build_local_graph(int n, int m, const InputEdge *in,
                              const int *comp, int comp_cnt,
                              int **out_lhead, LEdge **out_LE, int *out_lm,
                              int **out_comp_size, int **out_comp_vertices, int **out_comp_vstart) {
    int *comp_size = (int *)xcalloc((size_t)comp_cnt, sizeof(int));
    for (int v = 0; v < n; v++) comp_size[comp[v]]++;

    int *vstart = (int *)xmalloc((size_t)(comp_cnt + 1) * sizeof(int));
    vstart[0] = 0;
    for (int c = 0; c < comp_cnt; c++) vstart[c + 1] = vstart[c] + comp_size[c];

    int *pos = (int *)xmalloc((size_t)comp_cnt * sizeof(int));
    memcpy(pos, vstart, (size_t)comp_cnt * sizeof(int));

    int *comp_vertices = (int *)xmalloc((size_t)n * sizeof(int));
    for (int v = 0; v < n; v++) {
        int c = comp[v];
        comp_vertices[pos[c]++] = v;
    }
    free(pos);

    int *lhead = (int *)xmalloc((size_t)n * sizeof(int));
    for (int i = 0; i < n; i++) lhead[i] = -1;

    /* Count intra-SCC edges to size LEdge array. */
    int lm = 0;
    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        if (comp[u] == comp[v]) lm++;
    }

    LEdge *LE = (LEdge *)xmalloc((size_t)lm * sizeof(LEdge));
    int idx = 0;
    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        if (comp[u] != comp[v]) continue;
        LE[idx].to = v;
        LE[idx].id = i;
        LE[idx].next = lhead[u];
        lhead[u] = idx;
        idx++;
    }

    *out_lhead = lhead;
    *out_LE = LE;
    *out_lm = lm;
    *out_comp_size = comp_size;
    *out_comp_vertices = comp_vertices;
    *out_comp_vstart = vstart;
}

/* For each SCC, compute dominators in the SCC's reverse graph to detect bridges:
   An intra-SCC edge (u->v) is "same" iff it is the only way to reach v from u,
   i.e., removing it breaks reachability u->v. In a strongly connected SCC,
   this holds iff u is the immediate dominator of v in the reverse graph rooted at v.
   (Equivalently, in original graph rooted at u, v is dominated by u; using reverse
   rooted at v is symmetric and convenient for per-destination processing.)
   We implement Lengauer-Tarjan dominators on the reverse of SCC subgraph, per SCC,
   with an arbitrary root r in that SCC. Then for each original edge (u->v),
   in reverse graph it's (v->u). The edge is a strong bridge in SCC iff
   idom[u] == v in the reverse dominator tree rooted at r AND u is reachable from r.
   However, we need per-edge alternative path from u to v excluding that edge.
   In SCC, an edge u->v is "same" iff it is a strong bridge AND it is the only
   edge from u to v? Actually strong bridge is about SCC connectivity; but our
   condition is about reachability u->v only. In a strongly connected SCC,
   u->v has no alternative u->v path iff in the graph with that edge removed,
   v is not reachable from u. This is exactly that edge is a bridge in the flow
   graph rooted at u (dominators in original). Doing per-edge root u is too expensive.
   Instead, we use the known characterization: in a strongly connected digraph,
   an edge (u,v) is a strong bridge iff it is a bridge in dominator tree of G rooted
   at some start s OR in dominator tree of reverse(G) rooted at s. But our property
   is weaker (only u->v reachability), and in SCC it coincides with (u,v) being
   a bridge in the flow graph rooted at u, which is not captured by a single global root.
   Therefore we choose the narrowest faithful approach: for each SCC, for each edge,
   test reachability with that edge excluded using BFS/DFS within SCC.
   With n<=1000, we can do per-edge BFS within SCC; worst-case m=200000, SCC size 1000
   => too slow. We instead do per-source BFS once per vertex within SCC, and for each
   edge (u,v) decide if there exists an alternative path from u to v not using that edge.
   We can do this by computing reachability from u in SCC, and additionally whether
   all u->v paths must start with that specific edge. This can be decided by computing
   dominators in the SCC flow graph rooted at u, per u. With SCC size up to 1000,
   doing dominators per vertex is heavy but still bounded by 1e6 nodes total across SCCs
   times algorithm complexity; but m can be 2e5, still feasible if SCCs are small.
   Given constraints, simplest faithful method: for each SCC, for each vertex u,
   run BFS/DFS in SCC once to compute reachability and also count number of distinct
   first-step choices to reach each node. We can compute for each u:
     - dist (or just visited)
     - number of predecessors in BFS tree doesn't help for directed.
   So we fall back to per-edge exclusion BFS but optimize by SCC size:
     - If SCC has k vertices and e edges, per-edge BFS is O(e*(k+e)) too big.
   We need a better exact method.

   Exact method for "edge is the only u->v path" in a directed graph:
   Edge (u,v) is "same" iff v is not reachable from u in graph with (u,v) removed.
   This is equivalent to (u,v) being a bridge in the flow graph rooted at u.
   We can compute dominators for each root u within SCC using Lengauer-Tarjan in O(E alpha)
   per root, still too much for k=1000 (1000 * 200k).
   But n<=1000 globally, so we can do per-vertex root on the whole SCC subgraph:
   O(n*(m)) ~ 2e8 maybe borderline but in C with n=1000, m=200k, could be high.

   Narrowest reasonable interpretation with constraints: implement per-edge exclusion
   reachability using bitsets and transitive closure per SCC:
     - Compute reachability matrix R within SCC using bitset Floyd-Warshall via repeated
       DFS: for each vertex u, compute reachable set as bitset by DFS; O(k*(e/word)).
     - Then for edge (u,v), if there exists a path u->v of length >=2, then "diff".
       But alternative path could be length 1 via parallel edge; handle separately.
       If no length>=2 path, but there is another parallel edge u->v, then "diff".
       Else "same".
   This matches spec: alternative path not using that edge; in SCC, any alternative path
   must be either another parallel edge or length>=2.
   So we just need to know if there exists any path u->v with at least one intermediate
   vertex. That is: exists w such that u->w and w->v reachable (w can be u or v? must be
   intermediate, so w != u and w != v, and path length>=2).
   We can compute reachability bitsets and then test if (Reach[u] & RevReach[v]) has
   some vertex other than u and v. Where RevReach[v] is set of nodes that can reach v.
   This detects existence of a path u -> ... -> v with at least one intermediate.
   Additionally, if u==v (self-loop), alternative path exists if there is a cycle from u
   back to u not using that edge; in SCC there is always some cycle, but could be only
   that self-loop in a singleton SCC. Our test with intermediate vertex will detect
   alternative cycle via other vertex; for singleton SCC, it will be false, and then
   parallel self-loops decide.
*/
static void compute_scc_reach_bitsets(int n, int comp_cnt,
                                      const int *comp, const int *comp_size,
                                      const int *comp_vertices, const int *vstart,
                                      const int *lhead, const LEdge *LE,
                                      unsigned long long **out_reach,
                                      unsigned long long **out_rreach,
                                      int *out_words) {
    int maxk = 0;
    for (int c = 0; c < comp_cnt; c++) if (comp_size[c] > maxk) maxk = comp_size[c];
    int words = (maxk + 63) / 64;
    *out_words = words;

    unsigned long long *reach = (unsigned long long *)xcalloc((size_t)n * (size_t)words, sizeof(unsigned long long));
    unsigned long long *rreach = (unsigned long long *)xcalloc((size_t)n * (size_t)words, sizeof(unsigned long long));

    int *idx_in_comp = (int *)xmalloc((size_t)n * sizeof(int));
    for (int c = 0; c < comp_cnt; c++) {
        for (int i = vstart[c]; i < vstart[c + 1]; i++) {
            idx_in_comp[comp_vertices[i]] = i - vstart[c];
        }
    }

    /* For each SCC, compute reachability from each vertex via bitset propagation on DAG of SCC? No, within SCC.
       Use repeated DFS per source with stack; k<=1000 so O(k*(k+e)) is ok with bitsets? We'll do DFS per source. */
    int *stack = (int *)xmalloc((size_t)n * sizeof(int));
    int *seen = (int *)xmalloc((size_t)n * sizeof(int));
    int seen_mark = 1;
    memset(seen, 0, (size_t)n * sizeof(int));

    for (int c = 0; c < comp_cnt; c++) {
        int k = comp_size[c];
        if (k == 0) continue;
        int base = vstart[c];

        for (int si = 0; si < k; si++) {
            int s = comp_vertices[base + si];
            /* DFS within SCC */
            int sp = 0;
            stack[sp++] = s;
            seen[s] = seen_mark;

            while (sp > 0) {
                int v = stack[--sp];
                int vi = idx_in_comp[v];
                reach[(size_t)s * words + (vi >> 6)] |= 1ULL << (vi & 63);

                for (int ei = lhead[v]; ei != -1; ei = LE[ei].next) {
                    int to = LE[ei].to;
                    if (comp[to] != c) continue;
                    if (seen[to] != seen_mark) {
                        seen[to] = seen_mark;
                        stack[sp++] = to;
                    }
                }
            }
            seen_mark++;
            if (seen_mark == 0) { /* wrap; reset */
                memset(seen, 0, (size_t)n * sizeof(int));
                seen_mark = 1;
            }
        }

        /* Compute reverse reachability within SCC by traversing reverse edges:
           easiest: for each target t, DFS on reversed local graph. Build reverse local adjacency on the fly. */
        /* Build reverse adjacency for this SCC */
        int *rhead_local = (int *)xmalloc((size_t)k * sizeof(int));
        for (int i = 0; i < k; i++) rhead_local[i] = -1;

        /* Count edges in SCC to size temp arrays */
        int ecount = 0;
        for (int i = base; i < vstart[c + 1]; i++) {
            int v = comp_vertices[i];
            for (int ei = lhead[v]; ei != -1; ei = LE[ei].next) {
                int to = LE[ei].to;
                if (comp[to] == c) ecount++;
            }
        }
        Edge *tmp = (Edge *)xmalloc((size_t)ecount * sizeof(Edge));
        int tidx = 0;
        for (int i = base; i < vstart[c + 1]; i++) {
            int v = comp_vertices[i];
            int vi = i - base;
            (void)vi;
            for (int ei = lhead[v]; ei != -1; ei = LE[ei].next) {
                int to = LE[ei].to;
                if (comp[to] != c) continue;
                int toi = idx_in_comp[to];
                tmp[tidx].to = i - base; /* reverse: to -> from index */
                tmp[tidx].next = rhead_local[toi];
                rhead_local[toi] = tidx;
                tidx++;
            }
        }

        for (int ti = 0; ti < k; ti++) {
            int t = comp_vertices[base + ti];
            int sp = 0;
            stack[sp++] = t;
            seen[t] = seen_mark;

            while (sp > 0) {
                int v = stack[--sp];
                int vi = idx_in_comp[v];
                rreach[(size_t)t * words + (vi >> 6)] |= 1ULL << (vi & 63);

                int vli = vi; /* local index */
                for (int ei = rhead_local[vli]; ei != -1; ei = tmp[ei].next) {
                    int from_local = tmp[ei].to;
                    int from = comp_vertices[base + from_local];
                    if (seen[from] != seen_mark) {
                        seen[from] = seen_mark;
                        stack[sp++] = from;
                    }
                }
            }
            seen_mark++;
            if (seen_mark == 0) {
                memset(seen, 0, (size_t)n * sizeof(int));
                seen_mark = 1;
            }
        }

        free(rhead_local);
        free(tmp);
    }

    free(idx_in_comp);
    free(stack);
    free(seen);

    *out_reach = reach;
    *out_rreach = rreach;
}

/* Condensation graph reachability (path length >=2) using bitsets on DAG. */
static void condensation_alt_paths(int comp_cnt, int m, const InputEdge *in, const int *comp,
                                  int **out_pair_u, int **out_pair_v, int *out_pairs,
                                  int **out_pair_count,
                                  unsigned long long **out_creach, int *out_cwords) {
    /* Collect unique SCC arcs and counts. */
    int *cu = (int *)xmalloc((size_t)m * sizeof(int));
    int *cv = (int *)xmalloc((size_t)m * sizeof(int));
    int *is_inter = (int *)xmalloc((size_t)m * sizeof(int));
    int inter = 0;
    for (int i = 0; i < m; i++) {
        int a = comp[in[i].u], b = comp[in[i].v];
        if (a != b) {
            cu[inter] = a;
            cv[inter] = b;
            is_inter[i] = 1;
            inter++;
        } else {
            is_inter[i] = 0;
        }
    }

    /* Sort arcs by (u,v) to count parallels. */
    int *idx = (int *)xmalloc((size_t)inter * sizeof(int));
    for (int i = 0; i < inter; i++) idx[i] = i;

    /* qsort comparator */
    struct CmpCtx { int *cu; int *cv; } ctx = { cu, cv };
    (void)ctx;
    int cmpfunc(const void *pa, const void *pb) {
        int ia = *(const int *)pa, ib = *(const int *)pb;
        if (cu[ia] != cu[ib]) return (cu[ia] < cu[ib]) ? -1 : 1;
        if (cv[ia] != cv[ib]) return (cv[ia] < cv[ib]) ? -1 : 1;
        return 0;
    }
    qsort(idx, (size_t)inter, sizeof(int), cmpfunc);

    int *pair_u = (int *)xmalloc((size_t)inter * sizeof(int));
    int *pair_v = (int *)xmalloc((size_t)inter * sizeof(int));
    int *pair_count = (int *)xmalloc((size_t)inter * sizeof(int));
    int pairs = 0;

    for (int i = 0; i < inter; ) {
        int j = i;
        int u = cu[idx[i]], v = cv[idx[i]];
        while (j < inter && cu[idx[j]] == u && cv[idx[j]] == v) j++;
        pair_u[pairs] = u;
        pair_v[pairs] = v;
        pair_count[pairs] = j - i;
        pairs++;
        i = j;
    }

    /* Build adjacency of condensation graph with unique arcs. */
    int *head = (int *)xmalloc((size_t)comp_cnt * sizeof(int));
    for (int i = 0; i < comp_cnt; i++) head[i] = -1;
    Edge *E = (Edge *)xmalloc((size_t)pairs * sizeof(Edge));
    for (int i = 0; i < pairs; i++) {
        E[i].to = pair_v[i];
        E[i].next = head[pair_u[i]];
        head[pair_u[i]] = i;
    }

    /* Topological order via Kahn (graph is DAG by definition). */
    int *indeg = (int *)xcalloc((size_t)comp_cnt, sizeof(int));
    for (int u = 0; u < comp_cnt; u++) {
        for (int ei = head[u]; ei != -1; ei = E[ei].next) indeg[E[ei].to]++;
    }
    int *q = (int *)xmalloc((size_t)comp_cnt * sizeof(int));
    int qh = 0, qt = 0;
    for (int i = 0; i < comp_cnt; i++) if (indeg[i] == 0) q[qt++] = i;

    int *topo = (int *)xmalloc((size_t)comp_cnt * sizeof(int));
    int tsz = 0;
    while (qh < qt) {
        int u = q[qh++];
        topo[tsz++] = u;
        for (int ei = head[u]; ei != -1; ei = E[ei].next) {
            int v = E[ei].to;
            if (--indeg[v] == 0) q[qt++] = v;
        }
    }
    /* If tsz < comp_cnt, input violated SCC DAG property; unspecified. */

    int words = (comp_cnt + 63) / 64;
    unsigned long long *creach = (unsigned long long *)xcalloc((size_t)comp_cnt * (size_t)words, sizeof(unsigned long long));

    /* DP in reverse topological order: reach[u] = union(reach[v]) + v for each edge u->v */
    for (int ti = tsz - 1; ti >= 0; ti--) {
        int u = topo[ti];
        unsigned long long *Ru = creach + (size_t)u * words;
        for (int ei = head[u]; ei != -1; ei = E[ei].next) {
            int v = E[ei].to;
            unsigned long long *Rv = creach + (size_t)v * words;
            for (int w = 0; w < words; w++) Ru[w] |= Rv[w];
            Ru[v >> 6] |= 1ULL << (v & 63);
        }
    }

    free(cu);
    free(cv);
    free(is_inter);
    free(idx);
    free(head);
    free(E);
    free(indeg);
    free(q);
    free(topo);

    *out_pair_u = pair_u;
    *out_pair_v = pair_v;
    *out_pairs = pairs;
    *out_pair_count = pair_count;
    *out_creach = creach;
    *out_cwords = words;
}

/* Binary search for pair index in sorted (pair_u, pair_v) by (u,v). */
static int find_pair(int pairs, const int *pair_u, const int *pair_v, int u, int v) {
    int lo = 0, hi = pairs - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (pair_u[mid] < u || (pair_u[mid] == u && pair_v[mid] < v)) {
            lo = mid + 1;
        } else if (pair_u[mid] > u || (pair_u[mid] == u && pair_v[mid] > v)) {
            hi = mid - 1;
        } else {
            return mid;
        }
    }
    return -1;
}

int main(void) {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) return 0;

    InputEdge *in = (InputEdge *)xmalloc((size_t)m * sizeof(InputEdge));

    int *head = (int *)xmalloc((size_t)n * sizeof(int));
    int *rhead = (int *)xmalloc((size_t)n * sizeof(int));
    for (int i = 0; i < n; i++) head[i] = rhead[i] = -1;

    Edge *E = (Edge *)xmalloc((size_t)m * sizeof(Edge));
    Edge *RE = (Edge *)xmalloc((size_t)m * sizeof(Edge));

    for (int i = 0; i < m; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        u--; v--;
        in[i].u = u;
        in[i].v = v;

        E[i].to = v;
        E[i].next = head[u];
        head[u] = i;

        RE[i].to = u;
        RE[i].next = rhead[v];
        rhead[v] = i;
    }

    int *comp = (int *)xmalloc((size_t)n * sizeof(int));
    int comp_cnt = 0;
    kosaraju_scc(n, m, head, E, rhead, RE, comp, &comp_cnt);

    /* Build SCC-local graph for intra-SCC reachability bitsets. */
    int *lhead;
    LEdge *LE;
    int lm;
    int *comp_size;
    int *comp_vertices;
    int *vstart;
    build_local_graph(n, m, in, comp, comp_cnt, &lhead, &LE, &lm, &comp_size, &comp_vertices, &vstart);

    unsigned long long *reach, *rreach;
    int scc_words;
    compute_scc_reach_bitsets(n, comp_cnt, comp, comp_size, comp_vertices, vstart, lhead, LE,
                             &reach, &rreach, &scc_words);

    /* Condensation reachability and parallel arc counts. */
    int *pair_u, *pair_v, *pair_count, pairs;
    unsigned long long *creach;
    int cwords;
    condensation_alt_paths(comp_cnt, m, in, comp, &pair_u, &pair_v, &pairs, &pair_count, &creach, &cwords);

    /* Sort pairs arrays by (u,v) for binary search: condensation_alt_paths already produced in sorted order
       because it iterated sorted arcs. So (pair_u, pair_v) are sorted. */

    /* Count parallel edges within SCC for (u,v) to handle length-1 alternative. */
    /* Since n<=1000, we can use a dense matrix of counts per SCC? Too big per SCC; use global hash by sorting. */
    int *idx_all = (int *)xmalloc((size_t)m * sizeof(int));
    for (int i = 0; i < m; i++) idx_all[i] = i;
    int cmp_uv(const void *pa, const void *pb) {
        int ia = *(const int *)pa, ib = *(const int *)pb;
        if (in[ia].u != in[ib].u) return (in[ia].u < in[ib].u) ? -1 : 1;
        if (in[ia].v != in[ib].v) return (in[ia].v < in[ib].v) ? -1 : 1;
        return 0;
    }
    qsort(idx_all, (size_t)m, sizeof(int), cmp_uv);

    int *par_count = (int *)xcalloc((size_t)m, sizeof(int));
    for (int i = 0; i < m; ) {
        int j = i;
        int u = in[idx_all[i]].u, v = in[idx_all[i]].v;
        while (j < m && in[idx_all[j]].u == u && in[idx_all[j]].v == v) j++;
        int cnt = j - i;
        for (int k = i; k < j; k++) par_count[idx_all[k]] = cnt;
        i = j;
    }
    free(idx_all);

    /* For each edge, classify. */
    for (int i = 0; i < m; i++) {
        int u = in[i].u, v = in[i].v;
        int cu = comp[u], cv = comp[v];

        int is_diff = 0;

        if (cu != cv) {
            /* Inter-SCC */
            int pidx = find_pair(pairs, pair_u, pair_v, cu, cv);
            int cnt = (pidx >= 0) ? pair_count[pidx] : 1; /* should exist */
            if (cnt > 1) {
                is_diff = 1;
            } else {
                /* Alternative path of length >=2 in condensation:
                   exists w != cv such that cu -> w and w -> cv.
                   Using reachability bitsets: w in Reach[cu] and cv in Reach[w].
                   Equivalent: (Reach[cu] without cv) intersects Pred[cv] (nodes that can reach cv).
                   We don't have Pred bitsets for condensation; compute via creach on reverse? Not built.
                   Instead: check if there exists w != cv with cu reaches w and w reaches cv:
                     w in Reach[cu] AND cv in Reach[w].
                   Iterate w over set bits in Reach[cu] (comp_cnt<=1000). */
                unsigned long long *Rcu = creach + (size_t)cu * cwords;
                for (int w = 0; w < comp_cnt; w++) {
                    if (w == cv) continue;
                    if ((Rcu[w >> 6] >> (w & 63)) & 1ULL) {
                        unsigned long long *Rw = creach + (size_t)w * cwords;
                        if ((Rw[cv >> 6] >> (cv & 63)) & 1ULL) {
                            is_diff = 1;
                            break;
                        }
                    }
                }
            }
        } else {
            /* Intra-SCC */
            if (par_count[i] > 1) {
                /* Another parallel edge u->v provides alternative path of length 1. */
                is_diff = 1;
            } else {
                /* Alternative path of length >=2 within SCC:
                   exists intermediate x != u,v with u->x and x->v.
                   Use reach[u] and reverse-reach[v] within SCC. */
                unsigned long long *Ru = reach + (size_t)u * scc_words;
                unsigned long long *Rv = rreach + (size_t)v * scc_words;

                /* Compute intersection and check for any bit other than u and v. */
                int found = 0;
                for (int w = 0; w < scc_words; w++) {
                    unsigned long long inter = Ru[w] & Rv[w];
                    if (inter) {
                        found = 1;
                        break;
                    }
                }
                if (found) {
                    /* Need to exclude u and v themselves from being "intermediate". */
                    /* If intersection contains only {u} or {v} or {u,v}, then no length>=2 path. */
                    int only_uv = 1;
                    for (int w = 0; w < scc_words; w++) {
                        unsigned long long inter = Ru[w] & Rv[w];
                        if (!inter) continue;

                        /* Clear bits for u and v if they fall in this word. */
                        int iu = u; int iv = v;
                        if ((iu >> 6) == w) inter &= ~(1ULL << (iu & 63));
                        if ((iv >> 6) == w) inter &= ~(1ULL << (iv & 63));

                        if (inter) { only_uv = 0; break; }
                    }
                    if (!only_uv) is_diff = 1;
                }
            }
        }

        if (is_diff) {
            fputs("diff\n", stdout);
        } else {
            fputs("same\n", stdout);
        }
    }

    free(in);
    free(head);
    free(rhead);
    free(E);
    free(RE);
    free(comp);
    free(lhead);
    free(LE);
    free(comp_size);
    free(comp_vertices);
    free(vstart);
    free(reach);
    free(rreach);
    free(pair_u);
    free(pair_v);
    free(pair_count);
    free(creach);
    free(par_count);

    return 0;
}
