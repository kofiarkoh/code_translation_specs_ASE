#include <stdio.h>

#define N_MAX 4
#define SZ (2 * N_MAX + 1)
#define CENTER N_MAX

/* Fixed cube geometry mapping: RIGHT(top, front).
   This table encodes the globally agreed labeling/orientation convention.
   Indices 1..6 are used; 0 means "undefined" (should not occur per spec). */
static const int RIGHT_MAP[7][7] = {
    {0,0,0,0,0,0,0},
    /* top=1 */ {0,0,3,5,2,4,0},
    /* top=2 */ {0,4,0,1,6,0,3},
    /* top=3 */ {0,2,6,0,0,1,5},
    /* top=4 */ {0,5,1,0,0,6,2},
    /* top=5 */ {0,3,0,6,1,0,4},
    /* top=6 */ {0,0,4,2,5,3,0}
};

typedef struct {
    int top;
    int front;
} Ori;

static int right_face(int top, int front) {
    return RIGHT_MAP[top][front];
}

static int left_face(int top, int front) {
    return 7 - right_face(top, front);
}

static int back_face(int top, int front) {
    return 7 - front;
}

static void roll_north(Ori *o) {
    /* Move north: associated value is FRONT; cube rolls so FRONT becomes TOP. */
    int old_top = o->top;
    int old_front = o->front;
    o->top = old_front;
    o->front = 7 - old_top;
}

static void roll_south(Ori *o) {
    /* Move south: associated value is BACK; cube rolls so BACK becomes TOP. */
    int old_top = o->top;
    int old_front = o->front;
    o->top = 7 - old_front;
    o->front = old_top;
}

static void roll_west(Ori *o) {
    /* Move west: associated value is LEFT; cube rolls so LEFT becomes TOP.
       Front stays the same; update top, then front unchanged. */
    int old_top = o->top;
    int old_front = o->front;
    int old_left = left_face(old_top, old_front);
    o->top = old_left;
    o->front = old_front;
}

static void roll_east(Ori *o) {
    /* Move east: associated value is RIGHT; cube rolls so RIGHT becomes TOP.
       Front stays the same. */
    int old_top = o->top;
    int old_front = o->front;
    int old_right = right_face(old_top, old_front);
    o->top = old_right;
    o->front = old_front;
}

int main(void) {
    int n;
    while (scanf("%d", &n) == 1) {
        if (n == 0) break;

        int height[SZ][SZ];
        int rec_top[SZ][SZ];

        for (int r = 0; r < SZ; r++) {
            for (int c = 0; c < SZ; c++) {
                height[r][c] = 0;
                rec_top[r][c] = 0;
            }
        }

        for (int i = 0; i < n; i++) {
            Ori o;
            int r = CENTER, c = CENTER;
            scanf("%d %d", &o.top, &o.front);

            for (;;) {
                int curh = height[r][c];

                int best_dir = -1; /* 0=N,1=S,2=W,3=E */
                int best_val = -1;

                /* North */
                if (height[r - 1][c] < curh) {
                    int val = o.front;
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 0;
                    }
                }
                /* South */
                if (height[r + 1][c] < curh) {
                    int val = back_face(o.top, o.front);
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 1;
                    }
                }
                /* West */
                if (height[r][c - 1] < curh) {
                    int val = left_face(o.top, o.front);
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 2;
                    }
                }
                /* East */
                if (height[r][c + 1] < curh) {
                    int val = right_face(o.top, o.front);
                    if (val > 3 && val > best_val) {
                        best_val = val;
                        best_dir = 3;
                    }
                }

                if (best_dir < 0) break;

                if (best_dir == 0) {
                    r -= 1;
                    roll_north(&o);
                } else if (best_dir == 1) {
                    r += 1;
                    roll_south(&o);
                } else if (best_dir == 2) {
                    c -= 1;
                    roll_west(&o);
                } else { /* best_dir == 3 */
                    c += 1;
                    roll_east(&o);
                }
            }

            height[r][c] += 1;
            rec_top[r][c] = o.top;
        }

        int counts[7] = {0,0,0,0,0,0,0};
        for (int r = 0; r < SZ; r++) {
            for (int c = 0; c < SZ; c++) {
                int v = rec_top[r][c];
                if (v >= 1 && v <= 6) counts[v]++;
            }
        }

        printf("%d %d %d %d %d %d\n",
               counts[1], counts[2], counts[3],
               counts[4], counts[5], counts[6]);
    }
    return 0;
}
