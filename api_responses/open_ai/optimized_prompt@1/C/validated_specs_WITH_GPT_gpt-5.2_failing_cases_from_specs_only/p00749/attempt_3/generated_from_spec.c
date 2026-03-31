#include <stdio.h>
#include <string.h>

#define MAXH 60
#define MAXW 10
#define MAXN (MAXH*MAXW)
#define MAXC 99

typedef struct {
    int parent;      /* support parent component index (0 if none yet) */
    int child_head;  /* head of singly-linked child list (component index) */
    int child_tail;  /* tail for append */
    int next_sib;    /* next sibling in parent's child list */

    int m;           /* aggregated mass (starts intrinsic) */
    int mx;          /* aggregated horizontal moment (starts intrinsic) */
    int xl, xr;      /* footprint [min col, max col] */
} Comp;

static int uf_parent[MAXN];
static int uf_rankv[MAXN];

static int uf_find(int x) {
    while (uf_parent[x] != x) {
        uf_parent[x] = uf_parent[uf_parent[x]];
        x = uf_parent[x];
    }
    return x;
}

static void uf_union(int a, int b) {
    int ra = uf_find(a), rb = uf_find(b);
    if (ra == rb) return;
    if (uf_rankv[ra] < uf_rankv[rb]) {
        uf_parent[ra] = rb;
    } else if (uf_rankv[ra] > uf_rankv[rb]) {
        uf_parent[rb] = ra;
    } else {
        uf_parent[rb] = ra;
        uf_rankv[ra]++;
    }
}

static int comp_of_rep[MAXN]; /* maps union-find representative -> internal component index (1..cc), 0 if unseen */
static Comp comps[MAXC + 1];  /* 1-based component table */

static int dfs_check(int idx) {
    /* post-order: process children first */
    int ch = comps[idx].child_head;
    while (ch != 0) {
        if (!dfs_check(ch)) return 0;
        /* aggregate child's totals into parent */
        comps[idx].m += comps[ch].m;
        comps[idx].mx += comps[ch].mx;
        ch = comps[ch].next_sib;
    }

    /* balance check: xl < x < xr+1 where x = 0.5 + mx/m */
    /* Use integer inequalities to avoid floating issues:
       xl < 0.5 + mx/m  < xr+1
       Multiply by 2m (m>0): 2m*xl < m + 2mx  and  m + 2mx < 2m*(xr+1)
    */
    int m = comps[idx].m;
    int mx = comps[idx].mx;
    int xl = comps[idx].xl;
    int xr = comps[idx].xr;

    /* m should be > 0 for any real component; if not, treat as failure narrowly */
    if (m <= 0) return 0;

    long long left = 2LL * m * (long long)xl;
    long long mid  = (long long)m + 2LL * (long long)mx;
    long long right = 2LL * m * (long long)(xr + 1);

    if (!(left < mid && mid < right)) return 0;
    return 1;
}

int main(void) {
    int w, h;
    char grid[MAXH][MAXW + 8]; /* extra for newline/terminator */
    while (scanf("%d %d", &w, &h) == 2) {
        if (w <= 0) break;

        for (int r = 0; r < h; r++) {
            /* read a token; spec says at least w chars, no spaces */
            if (scanf("%s", grid[r]) != 1) return 0;
        }

        int n = w * h;

        /* init union-find */
        for (int i = 0; i < n; i++) {
            uf_parent[i] = i;
            uf_rankv[i] = 0;
        }

        /* build connectivity for same-symbol orthogonal adjacency */
        for (int r = 0; r < h; r++) {
            for (int c = 0; c < w; c++) {
                char ch = grid[r][c];
                if (ch == '.') continue;
                int id = r * w + c;
                if (c + 1 < w && grid[r][c + 1] == ch) {
                    uf_union(id, r * w + (c + 1));
                }
                if (r + 1 < h && grid[r + 1][c] == ch) {
                    uf_union(id, (r + 1) * w + c);
                }
            }
        }

        /* clear component mapping and table */
        for (int i = 0; i < n; i++) comp_of_rep[i] = 0;
        for (int i = 0; i <= MAXC; i++) {
            comps[i].parent = 0;
            comps[i].child_head = 0;
            comps[i].child_tail = 0;
            comps[i].next_sib = 0;
            comps[i].m = 0;
            comps[i].mx = 0;
            comps[i].xl = 0;
            comps[i].xr = -1;
        }

        int cc = 0;

        /* bottom-up, left-to-right scan */
        for (int r = h - 1; r >= 0; r--) {
            for (int c = 0; c < w; c++) {
                char ch = grid[r][c];
                if (ch == '.') continue;

                int cell = r * w + c;
                int rep = uf_find(cell);
                int A = comp_of_rep[rep];
                if (A == 0) {
                    /* create new internal component record */
                    cc++;
                    if (cc > MAXC) {
                        /* unspecified by spec; exit quietly */
                        return 0;
                    }
                    A = cc;
                    comp_of_rep[rep] = A;
                    comps[A].parent = 0;
                    comps[A].child_head = comps[A].child_tail = 0;
                    comps[A].next_sib = 0;
                    comps[A].m = 0;
                    comps[A].mx = 0;
                    comps[A].xl = c;
                    comps[A].xr = c;
                }

                /* accumulate intrinsic properties */
                comps[A].m += 1;
                comps[A].mx += c;
                if (c < comps[A].xl) comps[A].xl = c;
                if (c > comps[A].xr) comps[A].xr = c;

                /* support link attempt per rules */
                if (r == h - 1) continue;                 /* bottom row: no parent attempt */
                if (grid[r + 1][c] == '.') continue;      /* empty below */
                if (comps[A].parent != 0) continue;       /* already has parent: frozen */

                int below = (r + 1) * w + c;
                int repB = uf_find(below);
                int B = comp_of_rep[repB];
                if (B == 0) {
                    /* By scan order, below cell is in a lower row and thus should have been seen already.
                       If not, keep narrow behavior: create it now so links can be formed. */
                    cc++;
                    if (cc > MAXC) return 0;
                    B = cc;
                    comp_of_rep[repB] = B;
                    comps[B].parent = 0;
                    comps[B].child_head = comps[B].child_tail = 0;
                    comps[B].next_sib = 0;
                    comps[B].m = 0;
                    comps[B].mx = 0;
                    comps[B].xl = c;
                    comps[B].xr = c;
                }

                if (B == A) continue; /* same component */

                /* assign parent and append to B's child list */
                comps[A].parent = B;
                comps[A].next_sib = 0;
                if (comps[B].child_head == 0) {
                    comps[B].child_head = comps[B].child_tail = A;
                } else {
                    comps[comps[B].child_tail].next_sib = A;
                    comps[B].child_tail = A;
                }
            }
        }

        /* stability evaluation from component index 1 only */
        int stable = 0;
        if (cc >= 1) {
            comps[1].parent = 1; /* forced to be its own parent for bookkeeping */
            stable = dfs_check(1);
        } else {
            /* No components: spec does not define; narrow choice: UNSTABLE (no root component index 1). */
            stable = 0;
        }

        puts(stable ? "STABLE" : "UNSTABLE");
    }
    return 0;
}
