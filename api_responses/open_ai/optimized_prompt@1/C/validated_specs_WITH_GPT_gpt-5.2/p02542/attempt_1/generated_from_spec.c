#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

/*
  Translation notes / narrow interpretations chosen due to spec ambiguity:

  - Marker character: '#'
  - Empty traversable character: '.'
  - All other characters are non-traversable.

  - Reachability: from a marked cell (r,c), you may move only Down or Right,
    and you may step only on empty cells '.' (the destination empty cell is '.').
    The marked cell itself is not required to be '.'.

  - Distance: shortest-path length in number of moves (edges) under the above movement
    restriction. With only down/right moves and unit weights, if reachable then
    distance is (dr + dc) where dr = r2-r1, dc = c2-c1, but only if there exists
    a monotone path through '.' cells. We compute reachability via DP.

  - "Up to 100 farthest candidates per marked cell": we select at most 100 reachable
    empty cells with greatest distance; ties are resolved by internal ordering:
    we scan cells in row-major order and keep the first 100 encountered among equal
    distances after sorting by (distance desc, row asc, col asc).

  - Assignment: maximum total distance matching between marked cells and empty cells,
    one-to-one, using only the candidate edges.

  - Optimization model: min-cost max-flow on a bipartite graph with costs = -distance,
    so minimizing cost maximizes sum of distances.

  - Sentinel behavior:
      * If infeasible (cannot send required flow), mcmf returns INT_MIN; main negates
        and prints it (overflow/UB in C). To emulate "very large negative number"
        deterministically, we instead print -(long long)INT_MIN which is 2147483648.
      * If cannot find augmenting path (algorithmic failure), return INT_MAX; main
        negates and prints -INT_MAX.
    The spec distinguishes these; in this implementation:
      - If maxflow < required, return INT_MIN (infeasible).
      - If shortest path cannot be found while still needing flow, also treated as
        infeasible (INT_MIN). INT_MAX is reserved for internal unexpected failure
        (not expected here).
*/

enum { MAXH = 128, MAXW = 128 };
enum { MAXCAND = 100 };

typedef struct {
    int to, next;
    int cap;
    int cost;
} Edge;

typedef struct {
    int dist;
    int r, c;
    int id; /* empty cell id */
} Cand;

static int H, W;
static char grid[MAXH][MAXW + 5];

static int is_marker(char ch) { return ch == '#'; }
static int is_empty(char ch)  { return ch == '.'; }

static int cmp_cand(const void *a, const void *b) {
    const Cand *x = (const Cand*)a;
    const Cand *y = (const Cand*)b;
    if (x->dist != y->dist) return (y->dist - x->dist); /* desc */
    if (x->r != y->r) return (x->r - y->r);
    return (x->c - y->c);
}

/* Min-cost max-flow (successive SPFA on residual graph). */
typedef struct {
    int N;
    int *head;
    Edge *E;
    int ec;
    int Ecap;
} Graph;

static void g_init(Graph *g, int N, int Ecap) {
    g->N = N;
    g->head = (int*)malloc((size_t)N * sizeof(int));
    g->E = (Edge*)malloc((size_t)Ecap * sizeof(Edge));
    g->ec = 0;
    g->Ecap = Ecap;
    for (int i = 0; i < N; i++) g->head[i] = -1;
}

static void g_add_edge(Graph *g, int u, int v, int cap, int cost) {
    /* forward */
    g->E[g->ec].to = v;
    g->E[g->ec].cap = cap;
    g->E[g->ec].cost = cost;
    g->E[g->ec].next = g->head[u];
    g->head[u] = g->ec++;
    /* backward */
    g->E[g->ec].to = u;
    g->E[g->ec].cap = 0;
    g->E[g->ec].cost = -cost;
    g->E[g->ec].next = g->head[v];
    g->head[v] = g->ec++;
}

static int spfa(Graph *g, int s, int t, int *dist, int *pv, int *pe) {
    const int INF = INT_MAX / 4;
    int N = g->N;
    int *inQ = (int*)calloc((size_t)N, sizeof(int));
    int *q = (int*)malloc((size_t)N * sizeof(int));
    int qh = 0, qt = 0;

    for (int i = 0; i < N; i++) {
        dist[i] = INF;
        pv[i] = -1;
        pe[i] = -1;
    }
    dist[s] = 0;
    q[qt++] = s;
    inQ[s] = 1;

    while (qh != qt) {
        int u = q[qh++];
        if (qh == N) qh = 0;
        inQ[u] = 0;
        for (int ei = g->head[u]; ei != -1; ei = g->E[ei].next) {
            Edge *e = &g->E[ei];
            if (e->cap <= 0) continue;
            int v = e->to;
            int nd = dist[u] + e->cost;
            if (dist[u] != INF && nd < dist[v]) {
                dist[v] = nd;
                pv[v] = u;
                pe[v] = ei;
                if (!inQ[v]) {
                    q[qt++] = v;
                    if (qt == N) qt = 0;
                    inQ[v] = 1;
                }
            }
        }
    }

    int ok = (dist[t] != INF);
    free(inQ);
    free(q);
    return ok;
}

static int mcmf_min_cost(Graph *g, int s, int t, int need_flow, long long *out_cost) {
    int N = g->N;
    int *dist = (int*)malloc((size_t)N * sizeof(int));
    int *pv   = (int*)malloc((size_t)N * sizeof(int));
    int *pe   = (int*)malloc((size_t)N * sizeof(int));

    int flow = 0;
    long long cost = 0;

    while (flow < need_flow) {
        if (!spfa(g, s, t, dist, pv, pe)) {
            /* infeasible: cannot augment further */
            free(dist); free(pv); free(pe);
            return INT_MIN;
        }
        int add = need_flow - flow;
        for (int v = t; v != s; v = pv[v]) {
            if (v < 0) { /* should not happen */
                free(dist); free(pv); free(pe);
                return INT_MAX;
            }
            Edge *e = &g->E[pe[v]];
            if (e->cap < add) add = e->cap;
        }
        for (int v = t; v != s; v = pv[v]) {
            int ei = pe[v];
            g->E[ei].cap -= add;
            g->E[ei ^ 1].cap += add;
            cost += (long long)add * (long long)g->E[ei].cost;
        }
        flow += add;
    }

    free(dist); free(pv); free(pe);
    *out_cost = cost;
    return 0;
}

int main(void) {
    if (scanf("%d %d", &H, &W) != 2) return 0;

    for (int r = 0; r < H; r++) {
        /* Read a token; spec says line has at least W chars. */
        if (scanf("%s", grid[r]) != 1) return 0;
    }

    /* Collect marked cells and empty cells. */
    int maxCells = H * W;
    int *mark_r = (int*)malloc((size_t)maxCells * sizeof(int));
    int *mark_c = (int*)malloc((size_t)maxCells * sizeof(int));
    int *empty_id = (int*)malloc((size_t)maxCells * sizeof(int));
    int *empty_r = (int*)malloc((size_t)maxCells * sizeof(int));
    int *empty_c = (int*)malloc((size_t)maxCells * sizeof(int));

    int M = 0, E = 0;
    for (int r = 0; r < H; r++) {
        for (int c = 0; c < W; c++) {
            char ch = grid[r][c];
            if (is_marker(ch)) {
                mark_r[M] = r;
                mark_c[M] = c;
                M++;
            }
            if (is_empty(ch)) {
                empty_id[r * W + c] = E;
                empty_r[E] = r;
                empty_c[E] = c;
                E++;
            } else {
                empty_id[r * W + c] = -1;
            }
        }
    }

    /* Build candidate edges: for each marked cell, compute reachable empties and pick top 100 farthest. */
    /* DP reachability from (sr,sc) to all cells with only down/right through '.' */
    unsigned char *reach = (unsigned char*)malloc((size_t)maxCells);
    Cand *cands_all = (Cand*)malloc((size_t)E * sizeof(Cand));

    /* We'll store candidate lists in a flat array for graph construction. */
    int *cand_start = (int*)malloc((size_t)(M + 1) * sizeof(int));
    Cand *cand_flat = (Cand*)malloc((size_t)M * MAXCAND * sizeof(Cand));
    int flat_n = 0;

    for (int i = 0; i < M; i++) {
        int sr = mark_r[i], sc = mark_c[i];
        memset(reach, 0, (size_t)maxCells);

        /* Only cells (r>=sr, c>=sc) can be reached. */
        for (int r = sr; r < H; r++) {
            for (int c = sc; c < W; c++) {
                if (!is_empty(grid[r][c])) continue;
                int idx = r * W + c;
                if (r == sr && c == sc) {
                    /* If the marked cell is also '.', allow starting there. But marker is '#', so usually false. */
                    reach[idx] = 1;
                } else {
                    unsigned char ok = 0;
                    if (r > sr) ok |= reach[(r - 1) * W + c];
                    if (c > sc) ok |= reach[r * W + (c - 1)];
                    /* Additionally, allow stepping from the marked cell into adjacent empty cells. */
                    if (r == sr && c == sc + 1) ok |= 1; /* from (sr,sc) right */
                    if (c == sc && r == sr + 1) ok |= 1; /* from (sr,sc) down */
                    reach[idx] = ok;
                }
            }
        }

        int cn = 0;
        for (int r = sr; r < H; r++) {
            for (int c = sc; c < W; c++) {
                int idx = r * W + c;
                if (!reach[idx]) continue;
                int id = empty_id[idx];
                if (id < 0) continue;
                int dist = (r - sr) + (c - sc);
                cands_all[cn].dist = dist;
                cands_all[cn].r = r;
                cands_all[cn].c = c;
                cands_all[cn].id = id;
                cn++;
            }
        }

        qsort(cands_all, (size_t)cn, sizeof(Cand), cmp_cand);

        cand_start[i] = flat_n;
        int take = cn < MAXCAND ? cn : MAXCAND;
        for (int k = 0; k < take; k++) {
            cand_flat[flat_n++] = cands_all[k];
        }
    }
    cand_start[M] = flat_n;

    /* Construct flow network:
       source -> each mark (cap 1, cost 0)
       each empty -> sink (cap 1, cost 0)
       mark -> empty for candidates (cap 1, cost = -dist)
       Also allow leaving a mark unassigned? Spec says "optional 'unassigned' routing" but postconditions
       define "at most one" on both sides; objective is max sum, so unassigned is allowed.
       However, output is "maximum achievable sum over all valid one-to-one pairings" which can be empty.
       To model optional unassigned, we do NOT force sending flow for every mark; instead we compute
       maximum weight matching of any size. But min-cost max-flow needs a fixed flow.
       Narrowest consistent interpretation: pair each marked cell with a distinct reachable empty cell
       (i.e., perfect on marks) if possible; otherwise infeasible sentinel. This matches the spec's
       explicit infeasibility handling and "deficits" language.
    */
    int required_flow = M;

    int S = 0;
    int mark_base = 1;
    int empty_base = mark_base + M;
    int T = empty_base + E;
    int N = T + 1;

    /* Edge capacity estimate: 2*(S->M + E->T + candidates) */
    int totalCandEdges = flat_n;
    int Ecap = 2 * (M + E + totalCandEdges) + 16;

    Graph g;
    g_init(&g, N, Ecap);

    for (int i = 0; i < M; i++) g_add_edge(&g, S, mark_base + i, 1, 0);
    for (int j = 0; j < E; j++) g_add_edge(&g, empty_base + j, T, 1, 0);

    for (int i = 0; i < M; i++) {
        for (int p = cand_start[i]; p < cand_start[i + 1]; p++) {
            int eid = cand_flat[p].id;
            int dist = cand_flat[p].dist;
            g_add_edge(&g, mark_base + i, empty_base + eid, 1, -dist);
        }
    }

    long long mincost = 0;
    int rc = mcmf_min_cost(&g, S, T, required_flow, &mincost);

    long long out;
    if (rc == 0) {
        /* objective is max sum distances = -mincost */
        out = -mincost;
    } else if (rc == INT_MIN) {
        /* infeasible sentinel; main negates and prints it */
        out = -(long long)INT_MIN; /* deterministic "very large negative number" per spec intent */
        out = -out; /* print negative */
    } else {
        /* augmenting path failure sentinel; main negates and prints it */
        out = -(long long)INT_MAX;
    }

    printf("%lld\n", out);

    free(mark_r); free(mark_c);
    free(empty_id); free(empty_r); free(empty_c);
    free(reach);
    free(cands_all);
    free(cand_start);
    free(cand_flat);
    free(g.head);
    free(g.E);
    return 0;
}
