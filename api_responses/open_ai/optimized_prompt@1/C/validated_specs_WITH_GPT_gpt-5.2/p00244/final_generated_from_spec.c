#include <stdio.h>
#include <string.h>

#define MAXN 102
#define MAXADJ 102
#define INF 0x3f3f3f3f

typedef struct {
    int to;
    int w;
} Edge;

static Edge adj[MAXN][MAXADJ];
static int deg[MAXN];

/* dist[u][k] = best known cost to reach node u with exactly k remaining waivers, k in {0,1,2} */
static int dista[MAXN][3];
static unsigned char inq[MAXN][3];

/* Simple queue for SPFA-like relaxation over the expanded state space (node, remaining waivers). */
static int qnode[MAXN * 3 * 64];
static int qk[MAXN * 3 * 64];
static int qh, qt;

static void qpush(int u, int k) {
    qnode[qt] = u;
    qk[qt] = k;
    qt++;
    if (qt >= (int)(sizeof(qnode) / sizeof(qnode[0]))) qt = 0;
}

static void qpop(int *u, int *k) {
    *u = qnode[qh];
    *k = qk[qh];
    qh++;
    if (qh >= (int)(sizeof(qnode) / sizeof(qnode[0]))) qh = 0;
}

static int qempty(void) {
    return qh == qt;
}

int main(void) {
    int n, m;
    while (scanf("%d", &n) == 1) {
        if (n == 0) break;
        if (scanf("%d", &m) != 1) break;

        memset(deg, 0, sizeof(deg));

        for (int i = 0; i < m; i++) {
            int a, b, c;
            scanf("%d %d %d", &a, &b, &c);

            /* Specification does not define index base; use narrowest assumption: 1..n. */
            a--; b--;

            if (a >= 0 && a < n && b >= 0 && b < n) {
                if (deg[a] < MAXADJ) {
                    adj[a][deg[a]].to = b;
                    adj[a][deg[a]].w = c;
                    deg[a]++;
                }
                if (deg[b] < MAXADJ) {
                    adj[b][deg[b]].to = a;
                    adj[b][deg[b]].w = c;
                    deg[b]++;
                }
            }
            /* Out-of-range indices / capacity overflow: undefined per spec; we silently ignore. */
        }

        for (int i = 0; i < n; i++) {
            for (int k = 0; k < 3; k++) {
                dista[i][k] = INF;
                inq[i][k] = 0;
            }
        }

        /* Start at first node with exactly 2 remaining waivers, cost 0. */
        dista[0][2] = 0;

        qh = qt = 0;
        qpush(0, 2);
        inq[0][2] = 1;

        while (!qempty()) {
            int u, k;
            qpop(&u, &k);
            inq[u][k] = 0;

            int du = dista[u][k];
            if (du == INF) continue;

            for (int ei = 0; ei < deg[u]; ei++) {
                int v = adj[u][ei].to;
                int w = adj[u][ei].w;

                /* Transition using a waiver (if remaining waivers > 0): cost unchanged. */
                if (k > 0) {
                    int nk = k - 1;
                    if (du < dista[v][nk]) {
                        dista[v][nk] = du;
                        if (!inq[v][nk]) {
                            inq[v][nk] = 1;
                            qpush(v, nk);
                        }
                    }
                }

                /* Transition without using a waiver: permitted only when remaining waivers is not exactly 1. */
                if (k != 1) {
                    int nd = du + w;
                    if (nd < dista[v][k]) {
                        dista[v][k] = nd;
                        if (!inq[v][k]) {
                            inq[v][k] = 1;
                            qpush(v, k);
                        }
                    }
                }
            }
        }

        /* Answer is min cost at destination with remaining waivers 0 or 2; k==1 is not accepted. */
        int dest = n - 1;
        int ans = dista[dest][0];
        if (dista[dest][2] < ans) ans = dista[dest][2];

        printf("%d\n", ans);
    }
    return 0;
}
