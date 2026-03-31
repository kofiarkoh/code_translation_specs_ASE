#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 1000
#define MAXM 200000

typedef struct {
    int to;
    int next;
    int eid; /* original edge id */
} AdjEdge;

typedef struct {
    int u, v;
} InputEdge;

static int n, m;

/* Original graph adjacency */
static int headG[MAXN];
static int headGR[MAXN];
static AdjEdge EG[MAXM];
static AdjEdge EGR[MAXM];
static int ecG = 0, ecGR = 0;

/* SCC (Kosaraju) */
static int vis[MAXN];
static int order[MAXN];
static int ord_sz = 0;
static int comp[MAXN];
static int comp_cnt = 0;

/* For SCC-internal bridge-like test (directed strong bridges via dominators) */
static int headS[MAXN];
static int headSR[MAXN];
static AdjEdge ES[MAXM];
static AdjEdge ESR[MAXM];
static int ecS = 0, ecSR = 0;

/* Condensation graph (SCC DAG) */
static int headC[MAXN];
static int headCR[MAXN];
static AdjEdge EC[3 * MAXM];   /* capacity per spec hint */
static AdjEdge ECR[3 * MAXM];
static int ecC = 0, ecCR = 0;

/* For condensation edge multiplicity and reachability */
static int *pair_u = NULL, *pair_v = NULL, *pair_cnt = NULL;
static int pair_sz = 0, pair_cap = 0;

/* Output labels */
static char out_same[MAXM]; /* 1 => "same", 0 => "diff" */

/* Utility: add edge to adjacency list */
static void add_edge(AdjEdge *E, int *head, int *ec, int from, int to, int eid) {
    E[*ec].to = to;
    E[*ec].eid = eid;
    E[*ec].next = head[from];
    head[from] = *ec;
    (*ec)++;
}

/* Kosaraju DFS1 on G */
static void dfs1(int u) {
    vis[u] = 1;
    for (int e = headG[u]; e != -1; e = EG[e].next) {
        int v = EG[e].to;
        if (!vis[v]) dfs1(v);
    }
    order[ord_sz++] = u;
}

/* Kosaraju DFS2 on GR */
static void dfs2(int u, int cid) {
    comp[u] = cid;
    for (int e = headGR[u]; e != -1; e = EGR[e].next) {
        int v = EGR[e].to;
        if (comp[v] == -1) dfs2(v, cid);
    }
}

/* Simple dynamic array for SCC-pair counts */
static void pair_add_count(int cu, int cv) {
    /* linear search is too slow; use sort later: store all pairs then sort */
    (void)cu; (void)cv;
}

/* Sorting helper for pairs */
typedef struct {
    int a, b;
} Pair;

static int cmp_pair(const void *p1, const void *p2) {
    const Pair *x = (const Pair*)p1;
    const Pair *y = (const Pair*)p2;
    if (x->a != y->a) return (x->a < y->a) ? -1 : 1;
    if (x->b != y->b) return (x->b < y->b) ? -1 : 1;
    return 0;
}

/* Build SCC condensation graph and multiplicities */
static void build_condensation(const InputEdge *inE) {
    /* Collect all cross-SCC pairs (with multiplicity) */
    Pair *pairs = (Pair*)malloc((size_t)m * sizeof(Pair));
    int pc = 0;
    for (int i = 0; i < m; i++) {
        int u = inE[i].u, v = inE[i].v;
        int cu = comp[u], cv = comp[v];
        if (cu != cv) {
            pairs[pc].a = cu;
            pairs[pc].b = cv;
            pc++;
        }
    }
    qsort(pairs, (size_t)pc, sizeof(Pair), cmp_pair);

    /* Compress to unique pairs with counts */
    pair_u = (int*)malloc((size_t)(pc ? pc : 1) * sizeof(int));
    pair_v = (int*)malloc((size_t)(pc ? pc : 1) * sizeof(int));
    pair_cnt = (int*)malloc((size_t)(pc ? pc : 1) * sizeof(int));
    pair_sz = 0;

    for (int i = 0; i < pc; ) {
        int a = pairs[i].a, b = pairs[i].b;
        int j = i + 1;
        while (j < pc && pairs[j].a == a && pairs[j].b == b) j++;
        pair_u[pair_sz] = a;
        pair_v[pair_sz] = b;
        pair_cnt[pair_sz] = j - i;
        pair_sz++;
        i = j;
    }
    free(pairs);

    /* Build condensation adjacency with unique edges only */
    for (int i = 0; i < comp_cnt; i++) {
        headC[i] = -1;
        headCR[i] = -1;
    }
    ecC = ecCR = 0;
    for (int i = 0; i < pair_sz; i++) {
        int a = pair_u[i], b = pair_v[i];
        add_edge(EC, headC, &ecC, a, b, -1);
        add_edge(ECR, headCR, &ecCR, b, a, -1);
    }
}

/* Find multiplicity count for SCC pair (a,b) via binary search in compressed arrays */
static int pair_count_lookup(int a, int b) {
    int lo = 0, hi = pair_sz - 1;
    while (lo <= hi) {
        int mid = (lo + hi) >> 1;
        int ma = pair_u[mid], mb = pair_v[mid];
        if (ma < a || (ma == a && mb < b)) lo = mid + 1;
        else if (ma > a || (ma == a && mb > b)) hi = mid - 1;
        else return pair_cnt[mid];
    }
    return 0;
}

/* Reachability in condensation graph excluding direct edge length-1:
   Determine if there exists a path from s to t with length >= 2.
   Equivalent: exists neighbor x of s with x != t such that x can reach t. */
static int exists_len_ge_2_path(int s, int t) {
    static int seen[MAXN];
    static int q[MAXN];
    int qh = 0, qt = 0;

    /* BFS/DFS from all neighbors of s except t */
    for (int i = 0; i < comp_cnt; i++) seen[i] = 0;

    for (int e = headC[s]; e != -1; e = EC[e].next) {
        int x = EC[e].to;
        if (x == t) continue;
        if (!seen[x]) {
            seen[x] = 1;
            q[qt++] = x;
        }
    }

    while (qh < qt) {
        int u = q[qh++];
        if (u == t) return 1;
        for (int e = headC[u]; e != -1; e = EC[e].next) {
            int v = EC[e].to;
            if (!seen[v]) {
                seen[v] = 1;
                q[qt++] = v;
            }
        }
    }
    return 0;
}

/* ---------- Dominator computation (Lengauer-Tarjan) for directed graphs ---------- */
/* This implementation computes dominators in a directed graph rooted at r.
   Nodes are 0..(N-1). Graph given by adjacency head/edges arrays.
   Only nodes reachable from r are considered; others get idom = -1. */

typedef struct {
    int N;
    int r;
    int *head;
    AdjEdge *E;
} DomGraph;

static void dominators(const DomGraph *G, int *idom_out) {
    int N = G->N;
    int r = G->r;

    int *arr = (int*)malloc((size_t)N * sizeof(int));      /* dfs number -> vertex */
    int *rev = (int*)malloc((size_t)N * sizeof(int));      /* vertex -> dfs number (0 if unreachable), use 1..T */
    int *par = (int*)malloc((size_t)N * sizeof(int));      /* parent in DFS tree (vertex) */
    int *semi = (int*)malloc((size_t)N * sizeof(int));     /* semi-dominator (dfs number) */
    int *idom = (int*)malloc((size_t)N * sizeof(int));     /* immediate dominator (dfs number) */
    int *ancestor = (int*)malloc((size_t)N * sizeof(int)); /* union-find ancestor (vertex) */
    int *label = (int*)malloc((size_t)N * sizeof(int));    /* label (vertex) */
    int *bucket_head = (int*)malloc((size_t)N * sizeof(int));
    int *bucket_next = (int*)malloc((size_t)N * sizeof(int));
    int *child = (int*)malloc((size_t)N * sizeof(int));
    int *size = (int*)malloc((size_t)N * sizeof(int));

    /* Predecessor lists for reachable nodes: build via scanning edges each time is too slow.
       Since N<=1000, we can build predecessor adjacency for this dominator run by scanning all edges once. */
    int *pred_head = (int*)malloc((size_t)N * sizeof(int));
    int *pred_to = (int*)malloc((size_t)(MAXM + 5) * sizeof(int)); /* upper bound; safe for this program */
    int *pred_next = (int*)malloc((size_t)(MAXM + 5) * sizeof(int));
    int pred_ec = 0;

    for (int i = 0; i < N; i++) {
        rev[i] = 0;
        idom_out[i] = -1;
        pred_head[i] = -1;
    }

    /* DFS numbering */
    int T = 0;
    int *stack = (int*)malloc((size_t)N * sizeof(int));
    int *it = (int*)malloc((size_t)N * sizeof(int));
    int sp = 0;

    stack[sp] = r;
    it[sp] = G->head[r];
    par[r] = -1;
    rev[r] = -1; /* mark discovered */

    while (sp >= 0) {
        int u = stack[sp];
        int *eit = &it[sp];
        if (*eit == G->head[u] && rev[u] == -1) {
            /* first time we see u: assign dfs number */
            T++;
            rev[u] = T;
            arr[T - 1] = u;
            semi[u] = T;
            label[u] = u;
            ancestor[u] = -1;
            bucket_head[u] = -1;
            child[u] = -1;
            size[u] = 1;
        }
        int e = *eit;
        if (e == -1) {
            sp--;
            continue;
        }
        *eit = G->E[e].next;
        int v = G->E[e].to;
        if (rev[v] == 0) {
            par[v] = u;
            rev[v] = -1; /* discovered */
            sp++;
            stack[sp] = v;
            it[sp] = G->head[v];
        }
    }

    /* Build predecessor lists restricted to reachable nodes */
    for (int u = 0; u < N; u++) {
        if (rev[u] <= 0) continue;
        for (int e = G->head[u]; e != -1; e = G->E[e].next) {
            int v = G->E[e].to;
            if (rev[v] <= 0) continue;
            pred_to[pred_ec] = u;
            pred_next[pred_ec] = pred_head[v];
            pred_head[v] = pred_ec;
            pred_ec++;
        }
    }

    /* Union-find with path compression and eval/link (LT algorithm) */
    /* compress(v): */
    auto void compress_fn(int v) {
        if (ancestor[ancestor[v]] != -1) {
            compress_fn(ancestor[v]);
            if (semi[label[ancestor[v]]] < semi[label[v]]) label[v] = label[ancestor[v]];
            ancestor[v] = ancestor[ancestor[v]];
        }
    }
    /* eval(v): */
    auto int eval_fn(int v) {
        if (ancestor[v] == -1) return label[v];
        compress_fn(v);
        return (semi[label[ancestor[v]]] >= semi[label[v]]) ? label[v] : label[ancestor[v]];
    }
    /* link(v,w): */
    auto void link_fn(int v, int w) {
        int s = w;
        while (child[s] != -1 && semi[label[w]] < semi[label[child[s]]]) {
            if (size[s] + size[child[child[s]]] >= 2 * size[child[s]]) {
                ancestor[child[s]] = s;
                child[s] = child[child[s]];
            } else {
                size[child[s]] = size[s];
                ancestor[s] = child[s];
                s = child[s];
            }
        }
        label[s] = label[w];
        size[v] += size[w];
        if (size[v] < 2 * size[w]) {
            int tmp = s;
            s = child[v];
            child[v] = tmp;
        }
        while (s != -1) {
            ancestor[s] = v;
            s = child[s];
        }
    }

    /* Main LT loop: vertices in reverse DFS order (excluding root) */
    int root = r;
    for (int i = T - 1; i >= 1; i--) {
        int w = arr[i];
        /* compute semi[w] */
        int sw = semi[w];
        for (int pe = pred_head[w]; pe != -1; pe = pred_next[pe]) {
            int v = pred_to[pe];
            int u = eval_fn(v);
            if (semi[u] < sw) sw = semi[u];
        }
        semi[w] = sw;

        /* add w to bucket[vertex with dfs number sw] */
        int vs = arr[sw - 1];
        bucket_next[w] = bucket_head[vs];
        bucket_head[vs] = w;

        /* link parent(w), w */
        link_fn(par[w], w);

        /* for each v in bucket[parent(w)] */
        int pw = par[w];
        int v = bucket_head[pw];
        bucket_head[pw] = -1;
        while (v != -1) {
            int nextv = bucket_next[v];
            int u = eval_fn(v);
            idom[v] = (semi[u] < semi[v]) ? u : pw;
            v = nextv;
        }
    }

    /* Final step: explicitly define idom */
    idom[root] = root;
    for (int i = 1; i < T; i++) {
        int w = arr[i];
        if (idom[w] != arr[semi[w] - 1]) idom[w] = idom[idom[w]];
    }

    /* Output idom as vertex indices; unreachable remain -1 */
    for (int i = 0; i < T; i++) {
        int v = arr[i];
        int dv = idom[v];
        idom_out[v] = (dv == v) ? v : dv;
    }

    free(arr);
    free(rev);
    free(par);
    free(semi);
    free(idom);
    free(ancestor);
    free(label);
    free(bucket_head);
    free(bucket_next);
    free(child);
    free(size);
    free(pred_head);
    free(pred_to);
    free(pred_next);
    free(stack);
    free(it);
}

/* Determine "diff" edges within SCCs using strong bridge characterization:
   For edge (u->v) inside SCC:
     it is "diff" iff it is a strong bridge, i.e., either u dominates v in G_scc rooted at r
     OR v dominates u in reverse graph rooted at r (equivalently u dominates v in reverse rooted at r).
   We compute dominators once per SCC for both directions. */
static void classify_internal_edges(const InputEdge *inE) {
    /* Group vertices by SCC */
    int *scc_head = (int*)malloc((size_t)comp_cnt * sizeof(int));
    int *scc_next = (int*)malloc((size_t)n * sizeof(int));
    for (int i = 0; i < comp_cnt; i++) scc_head[i] = -1;
    for (int v = 0; v < n; v++) {
        int c = comp[v];
        scc_next[v] = scc_head[c];
        scc_head[c] = v;
    }

    /* Build adjacency lists for all internal edges (original direction and reverse) */
    for (int i = 0; i < n; i++) {
        headS[i] = -1;
        headSR[i] = -1;
    }
    ecS = ecSR = 0;
    for (int i = 0; i < m; i++) {
        int u = inE[i].u, v = inE[i].v;
        if (comp[u] == comp[v]) {
            add_edge(ES, headS, &ecS, u, v, i);
            add_edge(ESR, headSR, &ecSR, v, u, i);
        }
    }

    int *idom_f = (int*)malloc((size_t)n * sizeof(int));
    int *idom_r = (int*)malloc((size_t)n * sizeof(int));
    int *mark = (int*)malloc((size_t)n * sizeof(int));

    for (int c = 0; c < comp_cnt; c++) {
        int r = scc_head[c];
        if (r == -1) continue;

        /* Mark vertices in this SCC */
        for (int v = scc_head[c]; v != -1; v = scc_next[v]) mark[v] = 1;

        /* Dominators in forward internal graph rooted at r */
        DomGraph Gf = { n, r, headS, ES };
        dominators(&Gf, idom_f);

        /* Dominators in reverse internal graph rooted at r */
        DomGraph Gr = { n, r, headSR, ESR };
        dominators(&Gr, idom_r);

        /* For each internal edge in this SCC, decide if strong bridge */
        for (int u = scc_head[c]; u != -1; u = scc_next[u]) {
            for (int e = headS[u]; e != -1; e = ES[e].next) {
                int v = ES[e].to;
                int eid = ES[e].eid;
                if (!mark[v]) continue; /* should not happen */
                /* strong bridge test */
                int is_strong_bridge = 0;
                if (idom_f[v] == u) is_strong_bridge = 1;
                if (idom_r[u] == v) is_strong_bridge = 1;
                if (is_strong_bridge) out_same[eid] = 0; /* diff */
                else out_same[eid] = 1; /* same */
            }
        }

        /* Unmark */
        for (int v = scc_head[c]; v != -1; v = scc_next[v]) mark[v] = 0;
    }

    free(scc_head);
    free(scc_next);
    free(idom_f);
    free(idom_r);
    free(mark);
}

int main(void) {
    if (scanf("%d %d", &n, &m) != 2) return 0;

    if (n < 0 || n > MAXN || m < 0 || m > MAXM) {
        /* unspecified behavior per spec; just exit */
        return 0;
    }

    InputEdge *inE = (InputEdge*)malloc((size_t)(m ? m : 1) * sizeof(InputEdge));

    for (int i = 0; i < n; i++) {
        headG[i] = -1;
        headGR[i] = -1;
    }

    for (int i = 0; i < m; i++) {
        int a, b;
        if (scanf("%d %d", &a, &b) != 2) return 0;
        a--; b--;
        inE[i].u = a;
        inE[i].v = b;
        add_edge(EG, headG, &ecG, a, b, i);
        add_edge(EGR, headGR, &ecGR, b, a, i);
        out_same[i] = 0; /* default "diff" unless proven "same" */
    }

    /* SCC via Kosaraju */
    for (int i = 0; i < n; i++) vis[i] = 0;
    ord_sz = 0;
    for (int i = 0; i < n; i++) if (!vis[i]) dfs1(i);

    for (int i = 0; i < n; i++) comp[i] = -1;
    comp_cnt = 0;
    for (int i = n - 1; i >= 0; i--) {
        int v = order[i];
        if (comp[v] == -1) {
            dfs2(v, comp_cnt);
            comp_cnt++;
        }
    }

    /* Cross-SCC classification needs condensation multiplicity and length>=2 path */
    build_condensation(inE);

    /* First classify internal edges (within SCC) using dominators */
    classify_internal_edges(inE);

    /* Then classify cross-SCC edges */
    for (int i = 0; i < m; i++) {
        int u = inE[i].u, v = inE[i].v;
        int cu = comp[u], cv = comp[v];
        if (cu == cv) continue; /* already set by internal classification */

        int cnt = pair_count_lookup(cu, cv);
        if (cnt >= 2) {
            out_same[i] = 0; /* diff */
            continue;
        }
        /* cnt should be 1 for this edge's SCC pair */
        if (exists_len_ge_2_path(cu, cv)) {
            out_same[i] = 0; /* diff */
            continue;
        }
        out_same[i] = 1; /* same */
    }

    for (int i = 0; i < m; i++) {
        if (out_same[i]) puts("same");
        else puts("diff");
    }

    free(inE);
    free(pair_u);
    free(pair_v);
    free(pair_cnt);
    return 0;
}
