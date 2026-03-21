#include <stdio.h>

#define MAXH 105
#define MAXW 105

/* We use a padded grid with an extra border of empty cells around the input.
   This allows "outside of the grid" to be represented as reachable empty space
   starting from (0,0) in the padded coordinate system. */

static int occ[MAXH + 2][MAXW + 2];     /* 1 = occupied, 0 = empty */
static unsigned char ext[MAXH + 2][MAXW + 2]; /* 1 = exterior-reachable empty */

typedef struct {
    int r, c;
} Node;

static Node q[(MAXH + 2) * (MAXW + 2)];

static void neighbors(int r, int c, int out_r[6], int out_c[6]) {
    /* Hex grid adjacency on an "offset coordinates" layout:
       rows are indexed by r; parity of r determines diagonal neighbors.
       This is a standard choice; the specification does not mandate a specific
       embedding, only 6-neighbor connectivity. */
    static const int dr[6] = {0, 0, -1, -1, 1, 1};
    int parity = r & 1;
    /* For even rows: upper-left/lower-left are (c-1), upper-right/lower-right are (c)
       For odd rows:  upper-left/lower-left are (c),   upper-right/lower-right are (c+1) */
    int dc_ul = parity ? 0 : -1;
    int dc_ur = parity ? 1 : 0;

    out_r[0] = r + dr[0]; out_c[0] = c - 1;      /* left */
    out_r[1] = r + dr[1]; out_c[1] = c + 1;      /* right */
    out_r[2] = r + dr[2]; out_c[2] = c + dc_ul;  /* up-left */
    out_r[3] = r + dr[3]; out_c[3] = c + dc_ur;  /* up-right */
    out_r[4] = r + dr[4]; out_c[4] = c + dc_ul;  /* down-left */
    out_r[5] = r + dr[5]; out_c[5] = c + dc_ur;  /* down-right */
}

int main(void) {
    int w, h;
    if (scanf("%d %d", &w, &h) != 2) return 0;

    /* Read occupancy into padded grid at [1..h][1..w]. Border remains 0. */
    for (int r = 1; r <= h; r++) {
        for (int c = 1; c <= w; c++) {
            int v;
            scanf("%d", &v);
            occ[r][c] = (v == 1) ? 1 : 0;
        }
    }

    /* Exterior marking: BFS flood fill from outside cell (0,0) over empty cells. */
    int head = 0, tail = 0;
    ext[0][0] = 1;
    q[tail++] = (Node){0, 0};

    while (head < tail) {
        Node cur = q[head++];
        int nr[6], nc[6];
        neighbors(cur.r, cur.c, nr, nc);
        for (int k = 0; k < 6; k++) {
            int rr = nr[k], cc = nc[k];
            if (rr < 0 || rr > h + 1 || cc < 0 || cc > w + 1) continue;
            if (ext[rr][cc]) continue;
            if (occ[rr][cc]) continue; /* cannot traverse occupied */
            ext[rr][cc] = 1;
            q[tail++] = (Node){rr, cc};
        }
    }

    /* Boundary counting: for each occupied cell, count edges adjacent to exterior empty. */
    long long boundary = 0;
    for (int r = 1; r <= h; r++) {
        for (int c = 1; c <= w; c++) {
            if (!occ[r][c]) continue;
            int nr[6], nc[6];
            neighbors(r, c, nr, nc);
            for (int k = 0; k < 6; k++) {
                int rr = nr[k], cc = nc[k];
                /* Neighbors are within padded bounds by construction for r,c in [1..h],[1..w]. */
                if (rr < 0 || rr > h + 1 || cc < 0 || cc > w + 1) continue;
                if (!occ[rr][cc] && ext[rr][cc]) boundary++;
            }
        }
    }

    printf("%lld\n", boundary);
    return 0;
}
