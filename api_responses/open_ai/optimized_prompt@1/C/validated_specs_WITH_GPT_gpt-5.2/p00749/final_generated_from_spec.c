#include <stdio.h>
#include <string.h>

#define MAXH 60
#define MAXW 10
#define MAXN (MAXH * MAXW)

static int w, h;
static char g[MAXH][MAXW + 5];

/* Component id per cell (0 = none) */
static int comp[MAXH][MAXW];

/* DSU for component formation (same char, 4-neighborhood) */
static int parent_dsu[MAXN];
static int find_dsu(int x) {
    while (parent_dsu[x] != x) {
        parent_dsu[x] = parent_dsu[parent_dsu[x]];
        x = parent_dsu[x];
    }
    return x;
}
static void union_dsu(int a, int b) {
    a = find_dsu(a);
    b = find_dsu(b);
    if (a != b) parent_dsu[b] = a;
}

/* Per internal node id (1..cnt) */
static int m[MAXN + 5];
static int mx[MAXN + 5];
static int xl[MAXN + 5];
static int xr[MAXN + 5];

static int node_of_root[MAXN]; /* map DSU root index -> internal node id (0 if none) */
static int cnt;

/* Parent/child structure (single-parent, children appended in scan order) */
static int par[MAXN + 5];
static int first_child[MAXN + 5];
static int next_sib[MAXN + 5];
static int last_child[MAXN + 5];

/* processed flag to prevent double aggregation (as per spec) */
static int done[MAXN + 5];

static void add_child(int p, int ch) {
    if (first_child[p] == 0) {
        first_child[p] = ch;
        last_child[p] = ch;
    } else {
        next_sib[last_child[p]] = ch;
        last_child[p] = ch;
    }
}

static int dfs_check(int u) {
    /* depth-first over child/sibling links */
    for (int v = first_child[u]; v != 0; v = next_sib[v]) {
        if (!dfs_check(v)) return 0;
    }

    /* compute x = 0.5 + (mx/m) using real division */
    double x = 0.5 + ((double)mx[u] / (double)m[u]);

    /* unstable if x <= xl or x >= xr+1 (open interval (xl, xr+1)) */
    if (x <= (double)xl[u] || x >= (double)(xr[u] + 1)) return 0;

    /* aggregate into parent if not already done */
    if (!done[u]) {
        int p = par[u];
        /* root is forced to be its own parent; aggregation then adds to itself,
           matching the described behavior (no special-case stated). */
        m[p] += m[u];
        mx[p] += mx[u];
        done[u] = 1;
    }
    return 1;
}

int main(void) {
    while (1) {
        if (scanf("%d %d", &w, &h) != 2) return 0;
        if (w <= 0) break;

        for (int r = 0; r < h; r++) {
            scanf("%s", g[r]);
        }

        /* init DSU for all cells */
        int ncell = w * h;
        for (int i = 0; i < ncell; i++) parent_dsu[i] = i;

        /* union same-character occupied neighbors (4-neighborhood) */
        for (int r = 0; r < h; r++) {
            for (int c = 0; c < w; c++) {
                if (g[r][c] == '.') continue;
                int id = r * w + c;
                if (r + 1 < h && g[r + 1][c] == g[r][c]) union_dsu(id, (r + 1) * w + c);
                if (c + 1 < w && g[r][c + 1] == g[r][c]) union_dsu(id, r * w + (c + 1));
            }
        }

        /* clear mappings and per-node arrays */
        memset(node_of_root, 0, sizeof(node_of_root));
        cnt = 0;

        /* clear per-cell comp */
        for (int r = 0; r < h; r++) {
            for (int c = 0; c < w; c++) comp[r][c] = 0;
        }

        /* assign internal node ids in bottom-up, left-to-right scan order */
        for (int r = h - 1; r >= 0; r--) {
            for (int c = 0; c < w; c++) {
                if (g[r][c] == '.') continue;
                int root = find_dsu(r * w + c);
                int nid = node_of_root[root];
                if (nid == 0) {
                    nid = ++cnt;
                    node_of_root[root] = nid;
                }
                comp[r][c] = nid;
            }
        }

        /* init per-node fields */
        for (int i = 1; i <= cnt; i++) {
            m[i] = 0;
            mx[i] = 0;
            xl[i] =  1000000000;
            xr[i] = -1000000000;
            par[i] = 0;
            first_child[i] = next_sib[i] = last_child[i] = 0;
            done[i] = 0;
        }

        /* compute mass and moment from all occupied cells */
        for (int r = 0; r < h; r++) {
            for (int c = 0; c < w; c++) {
                int nid = comp[r][c];
                if (nid == 0) continue;
                m[nid] += 1;
                mx[nid] += c;
            }
        }

        /* bottom-up scan to build support footprint and parent/child relation */
        for (int r = h - 1; r >= 0; r--) {
            for (int c = 0; c < w; c++) {
                int nid = comp[r][c];
                if (nid == 0) continue;

                int supported = 0;
                if (r == h - 1) {
                    supported = 1;
                } else {
                    if (g[r + 1][c] != '.') supported = 1;
                }
                if (supported) {
                    if (c < xl[nid]) xl[nid] = c;
                    if (c > xr[nid]) xr[nid] = c;
                }

                if (r != h - 1 && g[r + 1][c] != '.') {
                    int below = comp[r + 1][c];
                    /* first-parent-wins */
                    if (par[nid] == 0) {
                        par[nid] = below;
                        add_child(below, nid);
                    }
                }
            }
        }

        /* root selection: internal node ID 1 is forced to be its own parent */
        par[1] = 1;

        /* Stability check starting from node 1 only (assumes at least one component exists) */
        int ok = dfs_check(1);

        puts(ok ? "STABLE" : "UNSTABLE");
    }
    return 0;
}
