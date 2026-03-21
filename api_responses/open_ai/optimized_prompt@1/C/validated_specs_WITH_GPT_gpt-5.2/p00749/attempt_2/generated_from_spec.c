#include <stdio.h>
#include <string.h>

#define MAXH 60
#define MAXW 10
#define MAXN (MAXH * MAXW)
#define MAXC MAXN

static int w, h;
static char grid[MAXH][MAXW + 5];

static int comp_id[MAXH][MAXW]; /* 0 for empty, else 1..ncomp */
static int ncomp;

static char comp_ch[MAXC + 1];
static int comp_m[MAXC + 1];
static int comp_mx[MAXC + 1];

/* support footprint; xl/xr only meaningful if has_support=1 */
static int comp_xl[MAXC + 1];
static int comp_xr[MAXC + 1];
static int comp_has_support[MAXC + 1];

/* scan-order node id assignment: node_id[comp] = 1..nnode, 0 if not assigned */
static int node_id_of_comp[MAXC + 1];
static int comp_of_node[MAXC + 1];
static int nnode;

/* parent/child as first-child / next-sibling lists over nodes 1..nnode */
static int parent_node[MAXC + 1];
static int first_child[MAXC + 1];
static int next_sibling[MAXC + 1];
static int last_child[MAXC + 1];

static int processed[MAXC + 1]; /* per node: aggregated upward already */

/* Flood fill to label components by 4-neighborhood with same character */
static void label_components(void) {
    int r, c;
    ncomp = 0;
    for (r = 0; r < h; r++) {
        for (c = 0; c < w; c++) comp_id[r][c] = 0;
    }

    for (r = 0; r < h; r++) {
        for (c = 0; c < w; c++) {
            if (grid[r][c] == '.' || comp_id[r][c] != 0) continue;

            ncomp++;
            comp_ch[ncomp] = grid[r][c];
            comp_m[ncomp] = 0;
            comp_mx[ncomp] = 0;

            /* stack for DFS */
            int st_r[MAXN], st_c[MAXN], sp = 0;
            st_r[sp] = r; st_c[sp] = c; sp++;
            comp_id[r][c] = ncomp;

            while (sp > 0) {
                sp--;
                int cr = st_r[sp], cc = st_c[sp];

                comp_m[ncomp] += 1;
                comp_mx[ncomp] += cc;

                /* 4-neighbors */
                const int dr[4] = {-1, 1, 0, 0};
                const int dc[4] = {0, 0, -1, 1};
                for (int k = 0; k < 4; k++) {
                    int nr = cr + dr[k], nc = cc + dc[k];
                    if (nr < 0 || nr >= h || nc < 0 || nc >= w) continue;
                    if (grid[nr][nc] != comp_ch[ncomp]) continue;
                    if (comp_id[nr][nc] != 0) continue;
                    comp_id[nr][nc] = ncomp;
                    st_r[sp] = nr; st_c[sp] = nc; sp++;
                }
            }
        }
    }
}

static void init_support_and_tree(void) {
    for (int i = 1; i <= ncomp; i++) {
        comp_has_support[i] = 0;
        comp_xl[i] = 0;
        comp_xr[i] = -1;
        node_id_of_comp[i] = 0;
    }
    nnode = 0;
    for (int i = 1; i <= MAXC; i++) {
        comp_of_node[i] = 0;
        parent_node[i] = 0;
        first_child[i] = 0;
        next_sibling[i] = 0;
        last_child[i] = 0;
        processed[i] = 0;
    }
}

/* Append child to parent's child list preserving attachment order */
static void attach_child(int parent, int child) {
    if (first_child[parent] == 0) {
        first_child[parent] = child;
        last_child[parent] = child;
    } else {
        next_sibling[last_child[parent]] = child;
        last_child[parent] = child;
    }
}

/* Bottom-up scan: assign node IDs, build single-parent tree, and compute support footprints */
static void scan_build(void) {
    for (int r = h - 1; r >= 0; r--) {
        for (int c = 0; c < w; c++) {
            if (grid[r][c] == '.') continue;

            int comp = comp_id[r][c];
            if (comp <= 0) continue;

            /* assign node id on first encounter in this scan */
            if (node_id_of_comp[comp] == 0) {
                nnode++;
                node_id_of_comp[comp] = nnode;
                comp_of_node[nnode] = comp;
                parent_node[nnode] = 0; /* unset */
            }

            /* support footprint contribution */
            int supported = 0;
            if (r == h - 1) {
                supported = 1;
            } else {
                if (grid[r + 1][c] != '.') supported = 1;
            }
            if (supported) {
                if (!comp_has_support[comp]) {
                    comp_has_support[comp] = 1;
                    comp_xl[comp] = c;
                    comp_xr[comp] = c;
                } else {
                    if (c < comp_xl[comp]) comp_xl[comp] = c;
                    if (c > comp_xr[comp]) comp_xr[comp] = c;
                }
            }

            /* parent/child relation via vertical adjacency, first parent wins */
            if (r != h - 1 && grid[r + 1][c] != '.') {
                int below_comp = comp_id[r + 1][c];
                if (below_comp > 0) {
                    int child_node = node_id_of_comp[comp];
                    /* below component must have a node id by now because we scan bottom-up */
                    int parent = node_id_of_comp[below_comp];
                    if (parent != 0 && parent_node[child_node] == 0) {
                        parent_node[child_node] = parent;
                        attach_child(parent, child_node);
                    }
                }
            }
        }
    }

    /* Root selection: node 1 is forced to be its own parent (even if it had none) */
    if (nnode >= 1) parent_node[1] = 1;
}

/* Recursive stability check; aggregates mass/moment upward on success */
static int dfs_check(int node) {
    /* process children first */
    for (int ch = first_child[node]; ch != 0; ch = next_sibling[ch]) {
        if (!dfs_check(ch)) return 0;
    }

    int comp = comp_of_node[node];
    int m = comp_m[comp];
    int mx = comp_mx[comp];

    /* center of mass: x = 0.5 + (mx / m) using real division */
    double x;
    if (m == 0) {
        /* Not expected from component formation; treat as failing strict containment */
        return 0;
    } else {
        x = 0.5 + ((double)mx / (double)m);
    }

    /* strict containment in (xl, xr+1); if no support footprint, fail */
    if (!comp_has_support[comp]) return 0;
    int xl = comp_xl[comp];
    int xr = comp_xr[comp];
    if (x <= (double)xl) return 0;
    if (x >= (double)(xr + 1)) return 0;

    /* aggregate into parent once */
    if (!processed[node]) {
        int p = parent_node[node];
        if (p != 0 && p != node) {
            int pcomp = comp_of_node[p];
            comp_m[pcomp] += comp_m[comp];
            comp_mx[pcomp] += comp_mx[comp];
        }
        processed[node] = 1;
    }
    return 1;
}

int main(void) {
    while (1) {
        if (scanf("%d %d", &w, &h) != 2) return 0;
        if (w <= 0) {
            /* h is read but not used for termination per spec */
            break;
        }

        for (int r = 0; r < h; r++) {
            char buf[MAXW + 5];
            scanf("%s", buf);
            /* assume at least w chars as per spec */
            for (int c = 0; c < w; c++) grid[r][c] = buf[c];
            grid[r][w] = '\0';
        }

        label_components();
        init_support_and_tree();
        scan_build();

        int ok;
        if (nnode < 1) {
            /* No components: root doesn't exist; narrow interpretation => not stable */
            ok = 0;
        } else {
            ok = dfs_check(1);
        }

        puts(ok ? "STABLE" : "UNSTABLE");
    }
    return 0;
}
