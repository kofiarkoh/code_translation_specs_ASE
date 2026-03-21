#include <stdio.h>
#include <string.h>

#define MAXN 5050
#define MAXM 200000

static int n, m;

/* adjacency list storage */
static int head[MAXN];
static int to[MAXM];
static int nxt[MAXM];
static int outdeg[MAXN];
static int ecnt;

/* expected values */
static double E0[MAXN]; /* original graph expectations */
static double E[MAXN];  /* expectations for current evaluated graph */

/* candidate edge per node: store edge index in adjacency list, or -1 */
static int candEdgeIdx[MAXN];

/* removed edge for current evaluation: (ru, rv) or (0,0) for none */
static int rem_u, rem_v;

static void add_edge(int u, int v) {
    to[ecnt] = v;
    nxt[ecnt] = head[u];
    head[u] = ecnt;
    outdeg[u]++;
    ecnt++;
}

/*
 * Compute expectations for the graph with at most one directed edge (rem_u, rem_v) removed.
 *
 * Recurrence per spec:
 *   E[n] = 0
 *   For u != n: E[u] = 1 + average_{(u->v) available} E[v]
 *
 * The specification implies a model where this recurrence is well-defined and finite.
 * We implement a fixed-point iteration (Gauss-Seidel style) to avoid assuming acyclicity.
 * Convergence is assumed by the preconditions; otherwise behavior is undefined.
 */
static double compute_expectation_with_removal(void) {
    int i, it;
    const int MAX_IT = 200000;
    const double EPS = 1e-12;

    for (i = 1; i <= n; i++) E[i] = 0.0;
    E[n] = 0.0;

    for (it = 0; it < MAX_IT; it++) {
        double maxdiff = 0.0;

        /* update in reverse order to better match typical "increasing index" reachability assumptions */
        for (i = n - 1; i >= 1; i--) {
            double sum = 0.0;
            int cnt = 0;

            for (int e = head[i]; e != -1; e = nxt[e]) {
                int v = to[e];
                if (i == rem_u && v == rem_v) continue; /* remove exactly this directed edge */
                sum += E[v];
                cnt++;
            }

            /* Per spec, if cnt==0 behavior is undefined; we do not guard beyond avoiding division by zero. */
            if (cnt == 0) continue;

            double newE = 1.0 + sum / (double)cnt;
            double diff = newE - E[i];
            if (diff < 0) diff = -diff;
            if (diff > maxdiff) maxdiff = diff;
            E[i] = newE;
        }

        E[n] = 0.0;

        if (maxdiff < EPS) break;
    }

    return E[1];
}

int main(void) {
    if (scanf("%d %d", &n, &m) != 2) return 0;

    /* fixed-size internal storage as implied by spec */
    if (n >= MAXN || m > MAXM) {
        /* undefined behavior per spec; we just proceed best-effort */
    }

    for (int i = 1; i <= n; i++) {
        head[i] = -1;
        outdeg[i] = 0;
        candEdgeIdx[i] = -1;
    }
    ecnt = 0;

    for (int i = 0; i < m; i++) {
        int u, v;
        if (scanf("%d %d", &u, &v) != 2) return 0;
        add_edge(u, v);
    }

    /* compute original expectations */
    rem_u = 0; rem_v = 0;
    double best = compute_expectation_with_removal();
    for (int i = 1; i <= n; i++) E0[i] = E[i];

    /* choose at most one candidate outgoing edge per node:
       if outdeg[u] > 1, pick edge (u->v) with maximum E0[v] */
    for (int u = 1; u <= n; u++) {
        if (u == n) continue;
        if (outdeg[u] <= 1) {
            candEdgeIdx[u] = -1;
            continue;
        }
        int bestEdge = -1;
        double bestVal = -1.0;
        for (int e = head[u]; e != -1; e = nxt[e]) {
            int v = to[e];
            double val = E0[v];
            if (bestEdge == -1 || val > bestVal) {
                bestVal = val;
                bestEdge = e;
            }
        }
        candEdgeIdx[u] = bestEdge;
    }

    /* evaluate each graph with one candidate edge removed, take minimum */
    for (int u = 1; u <= n; u++) {
        int e = candEdgeIdx[u];
        if (e < 0) continue;
        rem_u = u;
        rem_v = to[e];
        double val = compute_expectation_with_removal();
        if (val < best) best = val;
    }

    /* fixed decimal precision output */
    printf("%.10f\n", best);
    return 0;
}
