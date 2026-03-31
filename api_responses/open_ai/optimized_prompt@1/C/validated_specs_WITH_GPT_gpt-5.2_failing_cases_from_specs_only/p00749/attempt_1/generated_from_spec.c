#include <stdio.h>
#include <string.h>

#define MAX_W 10
#define MAX_H 60
#define MAX_CELLS (MAX_W * MAX_H)
#define MAX_COMP 99

typedef struct {
    int used;
    int intrinsic_mass;      /* number of cells */
    int intrinsic_moment;    /* sum of column indices of its own cells */
    int min_col, max_col;    /* footprint from own cells only */
    int parent;              /* supporting component below (or -1) */
    int child;               /* supported component above (or -1); at most one per spec assumption */
    int agg_mass;            /* aggregated mass (intrinsic + supported descendants) */
    int agg_moment;          /* aggregated moment */
    int indeg;               /* number of children (0 or 1 under spec assumption) */
} Comp;

static char grid[MAX_H][MAX_W + 1];
static int comp_id[MAX_H][MAX_W];

static int in_bounds(int r, int c, int h, int w) {
    return (r >= 0 && r < h && c >= 0 && c < w);
}

int main(void) {
    int w, h;
    while (scanf("%d", &w) == 1) {
        if (w <= 0) break;
        if (scanf("%d", &h) != 1) break;

        /* Read grid rows; each row has at least w characters per spec. */
        for (int r = 0; r < h; r++) {
            if (scanf("%s", grid[r]) != 1) grid[r][0] = '\0';
            grid[r][w] = '\0';
        }

        /* Initialize component ids to -1. */
        for (int r = 0; r < h; r++)
            for (int c = 0; c < w; c++)
                comp_id[r][c] = -1;

        Comp comps[MAX_COMP + 1];
        memset(comps, 0, sizeof(comps));
        for (int i = 0; i <= MAX_COMP; i++) {
            comps[i].min_col = 0x3fffffff;
            comps[i].max_col = -0x3fffffff;
            comps[i].parent = -1;
            comps[i].child = -1;
        }

        int ncomp = 0;

        /* Identify connected components of non-empty equal symbols (orthogonal adjacency). */
        for (int r0 = 0; r0 < h; r0++) {
            for (int c0 = 0; c0 < w; c0++) {
                char sym = grid[r0][c0];
                if (sym == '.' || sym == '\0') continue; /* treat '.' as empty marker (narrowest reasonable) */
                if (comp_id[r0][c0] != -1) continue;

                if (ncomp >= MAX_COMP) {
                    /* Capacity overflow: unspecified behavior by contract; proceed without extra handling. */
                }
                int cid = ncomp++;
                if (cid > MAX_COMP) cid = MAX_COMP; /* clamp to avoid out-of-bounds; behavior unspecified anyway */

                comps[cid].used = 1;
                comps[cid].intrinsic_mass = 0;
                comps[cid].intrinsic_moment = 0;
                comps[cid].min_col = 0x3fffffff;
                comps[cid].max_col = -0x3fffffff;
                comps[cid].parent = -1;
                comps[cid].child = -1;

                /* BFS/DFS using a simple queue over cell indices. */
                int q[MAX_CELLS];
                int qh = 0, qt = 0;
                q[qt++] = r0 * w + c0;
                comp_id[r0][c0] = cid;

                while (qh < qt) {
                    int idx = q[qh++];
                    int r = idx / w;
                    int c = idx % w;

                    comps[cid].intrinsic_mass += 1;
                    comps[cid].intrinsic_moment += c;
                    if (c < comps[cid].min_col) comps[cid].min_col = c;
                    if (c > comps[cid].max_col) comps[cid].max_col = c;

                    static const int dr[4] = { -1, 1, 0, 0 };
                    static const int dc[4] = { 0, 0, -1, 1 };
                    for (int k = 0; k < 4; k++) {
                        int nr = r + dr[k], nc = c + dc[k];
                        if (!in_bounds(nr, nc, h, w)) continue;
                        if (comp_id[nr][nc] != -1) continue;
                        if (grid[nr][nc] != sym) continue;
                        comp_id[nr][nc] = cid;
                        q[qt++] = nr * w + nc;
                    }
                }
            }
        }

        /* Derive support relation based on immediate vertical adjacency:
           if a cell of component A has a non-empty cell directly below belonging to component B,
           then A is supported by B (A.parent = B). Each component has at most one parent per spec. */
        for (int r = 0; r < h - 1; r++) {
            for (int c = 0; c < w; c++) {
                int a = comp_id[r][c];
                int b = comp_id[r + 1][c];
                if (a == -1 || b == -1) continue;
                if (a == b) continue;
                /* a is above b */
                if (comps[a].parent == -1) {
                    comps[a].parent = b;
                } else {
                    /* Multiple supports: unspecified by contract; keep first observed. */
                }
            }
        }

        /* Build child pointers and indegrees (each parent may have at most one child under spec assumption). */
        for (int i = 0; i < ncomp && i <= MAX_COMP; i++) {
            if (!comps[i].used) continue;
            int p = comps[i].parent;
            if (p >= 0 && p <= MAX_COMP) {
                comps[p].child = i; /* if multiple children exist, behavior unspecified; last wins */
                comps[p].indeg += 1;
            }
        }

        /* Bottom-up evaluation: aggregate supported masses/moments into supporting components.
           Use a queue of components with indeg==0 (no supported components above them). */
        int q[MAX_COMP + 1];
        int qh = 0, qt = 0;
        for (int i = 0; i < ncomp && i <= MAX_COMP; i++) {
            if (!comps[i].used) continue;
            comps[i].agg_mass = comps[i].intrinsic_mass;
            comps[i].agg_moment = comps[i].intrinsic_moment;
            if (comps[i].indeg == 0) q[qt++] = i;
        }

        int stable = 1;

        while (qh < qt) {
            int v = q[qh++];

            /* Check strict-inside condition using integer arithmetic:
               center = 0.5 + moment/mass
               require: center > min_col  AND center < (max_col + 1)
               Multiply by 2*mass:
                 2*center*mass = mass + 2*moment
               Conditions:
                 mass + 2*moment > 2*min_col*mass
                 mass + 2*moment < 2*(max_col+1)*mass
            */
            int mass = comps[v].agg_mass;
            int moment = comps[v].agg_moment;
            int minc = comps[v].min_col;
            int maxc = comps[v].max_col;

            /* mass is positive for any non-empty component */
            long long lhs = (long long)mass + 2LL * (long long)moment;
            long long lo = 2LL * (long long)minc * (long long)mass;
            long long hi = 2LL * (long long)(maxc + 1) * (long long)mass;

            if (!(lhs > lo && lhs < hi)) {
                stable = 0;
                break;
            }

            /* Propagate aggregation to parent (supporting component below). */
            int p = comps[v].parent;
            if (p >= 0 && p <= MAX_COMP) {
                comps[p].agg_mass += comps[v].agg_mass;
                comps[p].agg_moment += comps[v].agg_moment;
                comps[p].indeg -= 1;
                if (comps[p].indeg == 0) q[qt++] = p;
            }
        }

        /* If there are cycles or unresolved indegrees, behavior is not defined by contract.
           We do not add extra handling; remaining components would not be checked. */

        puts(stable ? "STABLE" : "UNSTABLE");
    }
    return 0;
}
