#include <stdio.h>

#define N_MAX 4
#define SZ (2 * N_MAX + 1) /* 9 */
#define CENTER N_MAX       /* 4 */

/*
 * RIGHT(top, front) lookup table as used by the program.
 * The specification states this table is fixed and crucial, but does not
 * provide its concrete contents. The narrowest reasonable interpretation
 * that preserves the specified behavior is to define RIGHT for all
 * (top, front) pairs that can occur in valid inputs (i.e., consistent cube
 * orientations), using a standard cube geometry consistent with:
 *   BOTTOM = 7 - top, BACK = 7 - front, LEFT = 7 - RIGHT.
 *
 * We implement RIGHT(top, front) by precomputing it from a canonical cube
 * labeling and all 24 orientations, then filling a 7x7 table (indices 1..6).
 * Inputs are assumed to only use pairs for which RIGHT is defined.
 */

typedef struct {
    int top, bottom, front, back, left, right;
} Cube;

static int RIGHT_TBL[7][7]; /* 0 means undefined */

/* Rotate cube north: roll toward decreasing row (front face goes to top). */
static Cube rot_north(Cube c) {
    Cube n = c;
    n.top = c.front;
    n.bottom = c.back;
    n.front = c.bottom;
    n.back = c.top;
    return n;
}

/* Rotate cube south: roll toward increasing row (back face goes to top). */
static Cube rot_south(Cube c) {
    Cube n = c;
    n.top = c.back;
    n.bottom = c.front;
    n.front = c.top;
    n.back = c.bottom;
    return n;
}

/* Rotate cube west: roll toward decreasing col (left face goes to top). */
static Cube rot_west(Cube c) {
    Cube n = c;
    n.top = c.right;
    n.bottom = c.left;
    n.left = c.top;
    n.right = c.bottom;
    return n;
}

/* Rotate cube east: roll toward increasing col (right face goes to top). */
static Cube rot_east(Cube c) {
    Cube n = c;
    n.top = c.left;
    n.bottom = c.right;
    n.left = c.bottom;
    n.right = c.top;
    return n;
}

static void build_right_table(void) {
    int visited[7][7] = {{0}};
    Cube q[64];
    int head = 0, tail = 0;

    /* Canonical labeling consistent with opposite pairs summing to 7. */
    Cube start;
    start.top = 1;
    start.bottom = 6;
    start.front = 2;
    start.back = 5;
    start.left = 3;
    start.right = 4;

    q[tail++] = start;

    while (head < tail) {
        Cube c = q[head++];

        if (c.top >= 1 && c.top <= 6 && c.front >= 1 && c.front <= 6) {
            if (!visited[c.top][c.front]) {
                visited[c.top][c.front] = 1;
                RIGHT_TBL[c.top][c.front] = c.right;

                /* Enqueue neighbors in orientation space. */
                q[tail++] = rot_north(c);
                q[tail++] = rot_south(c);
                q[tail++] = rot_west(c);
                q[tail++] = rot_east(c);
            }
        }
    }
}

static int RIGHT_of(int top, int front) {
    return RIGHT_TBL[top][front];
}

static int LEFT_of(int top, int front) {
    return 7 - RIGHT_of(top, front);
}

static int BACK_of(int top, int front) {
    (void)top;
    return 7 - front;
}

static int BOTTOM_of(int top, int front) {
    (void)front;
    return 7 - top;
}

int main(void) {
    build_right_table();

    for (;;) {
        int n;
        if (scanf("%d", &n) != 1) return 0;
        if (n == 0) return 0;

        int height[SZ][SZ] = {{0}};
        int rec_top[SZ][SZ] = {{0}};

        for (int i = 0; i < n; i++) {
            int top, front;
            scanf("%d %d", &top, &front);

            int r = CENTER, c = CENTER;

            for (;;) {
                int curh = height[r][c];

                int best_val = 0;
                int best_dir = -1; /* 0=N,1=S,2=W,3=E */

                /* North */
                if (height[r - 1][c] < curh) {
                    int val = front; /* FRONT */
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 0;
                    }
                }
                /* South */
                if (height[r + 1][c] < curh) {
                    int val = BACK_of(top, front);
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 1;
                    }
                }
                /* West */
                if (height[r][c - 1] < curh) {
                    int val = LEFT_of(top, front);
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 2;
                    }
                }
                /* East */
                if (height[r][c + 1] < curh) {
                    int val = RIGHT_of(top, front);
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 3;
                    }
                }

                if (best_dir < 0) break;

                /* Update position and orientation using assignments in spec. */
                if (best_dir == 0) { /* North */
                    int oldtop = top, oldfront = front;
                    r -= 1;
                    front = oldtop;
                    top = BACK_of(oldtop, oldfront);
                } else if (best_dir == 1) { /* South */
                    int oldtop = top, oldfront = front;
                    r += 1;
                    front = BOTTOM_of(oldtop, oldfront);
                    top = oldfront;
                } else if (best_dir == 2) { /* West */
                    int oldtop = top, oldfront = front;
                    c -= 1;
                    front = oldfront;
                    top = RIGHT_of(oldtop, oldfront);
                } else { /* East */
                    int oldtop = top, oldfront = front;
                    c += 1;
                    front = oldfront;
                    top = LEFT_of(oldtop, oldfront);
                }
            }

            height[r][c] += 1;
            rec_top[r][c] = top;
        }

        int cnt[7] = {0};
        for (int r = 0; r < SZ; r++) {
            for (int c = 0; c < SZ; c++) {
                int v = rec_top[r][c];
                if (v >= 1 && v <= 6) cnt[v]++;
            }
        }

        printf("%d %d %d %d %d %d\n",
               cnt[1], cnt[2], cnt[3], cnt[4], cnt[5], cnt[6]);
    }
}
