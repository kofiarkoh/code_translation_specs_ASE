#include <stdio.h>
#include <string.h>
#include <limits.h>

#define MAXN 1010  /* fixed internal maximum; must be large enough for valid inputs */

/* Min-heap priority queue for Dijkstra */
typedef struct {
    long long d;
    int r, c;
} Node;

static Node heap[MAXN * MAXN + 5];
static int hsz = 0;

static void heap_push(Node x) {
    int i = ++hsz;
    heap[i] = x;
    while (i > 1) {
        int p = i >> 1;
        if (heap[p].d <= heap[i].d) break;
        Node tmp = heap[p];
        heap[p] = heap[i];
        heap[i] = tmp;
        i = p;
    }
}

static Node heap_pop(void) {
    Node ret = heap[1];
    heap[1] = heap[hsz--];
    int i = 1;
    for (;;) {
        int l = i << 1, r = l + 1, m = i;
        if (l <= hsz && heap[l].d < heap[m].d) m = l;
        if (r <= hsz && heap[r].d < heap[m].d) m = r;
        if (m == i) break;
        Node tmp = heap[i];
        heap[i] = heap[m];
        heap[m] = tmp;
        i = m;
    }
    return ret;
}

static int heap_empty(void) { return hsz == 0; }

/* Internal storage with margin; spec says hazard marking does not do bounds checks.
   We therefore allocate a padded grid and place input at offset +1,+1 so that
   marking neighbors at +/-1 stays within allocated storage for valid inputs. */
static char grid[MAXN + 2][MAXN + 2];
static unsigned char danger[MAXN + 2][MAXN + 2];
static unsigned char finalized[MAXN + 2][MAXN + 2];
static long long distv[MAXN + 2][MAXN + 2];

int main(void) {
    int H, W;
    long long A_in, B;
    if (scanf("%d %d %lld %lld", &H, &W, &A_in, &B) != 4) return 0;

    /* Initialize padding to non-special values */
    for (int i = 0; i < H + 2; i++) {
        for (int j = 0; j < W + 2; j++) {
            grid[i][j] = '.';
            danger[i][j] = 0;
            finalized[i][j] = 0;
            distv[i][j] = LLONG_MAX / 4;
        }
    }

    int sr = -1, sc = -1, gr = -1, gc = -1;

    char buf[MAXN + 5];
    for (int i = 1; i <= H; i++) {
        if (scanf("%s", buf) != 1) return 0;
        for (int j = 1; j <= W; j++) {
            char ch = buf[j - 1];
            grid[i][j] = ch;
            if (ch == 's') { sr = i; sc = j; }
            if (ch == 'g') { gr = i; gc = j; }
        }
    }

    /* Derived hazard (dangerous) marking: mark '*' and its 8 neighbors.
       Per spec, this conceptually does not do explicit boundary checks. */
    for (int i = 1; i <= H; i++) {
        for (int j = 1; j <= W; j++) {
            if (grid[i][j] == '*') {
                for (int di = -1; di <= 1; di++) {
                    for (int dj = -1; dj <= 1; dj++) {
                        danger[i + di][j + dj] = 1;
                    }
                }
            }
        }
    }

    /* Dijkstra */
    distv[sr][sc] = 0;
    heap_push((Node){0, sr, sc});

    /* Movement deltas */
    static const int dr8[8] = {-1,-1,-1, 0, 0, 1, 1, 1};
    static const int dc8[8] = {-1, 0, 1,-1, 1,-1, 0, 1};
    static const int dr4[4] = {-1, 0, 0, 1};
    static const int dc4[4] = { 0,-1, 1, 0};

    while (!heap_empty()) {
        Node cur = heap_pop();
        int r = cur.r, c = cur.c;
        if (finalized[r][c]) continue;
        finalized[r][c] = 1;

        if (r == gr && c == gc) {
            printf("%lld\n", cur.d);
            return 0;
        }

        int curDanger = danger[r][c] ? 1 : 0;

        if (!curDanger) {
            /* up to 8-direction moves, but diagonal only allowed when destination is a wall */
            for (int k = 0; k < 8; k++) {
                int nr = r + dr8[k], nc = c + dc8[k];
                if (nr < 1 || nr > H || nc < 1 || nc > W) continue;

                int isDiag = (dr8[k] != 0 && dc8[k] != 0);
                int destWall = (grid[nr][nc] == '#');

                if (isDiag && !destWall) continue; /* diagonal only into wall */

                long long step = (isDiag ? 2 * A_in : A_in) + (destWall ? B : 0);
                long long nd = cur.d + step;
                if (nd < distv[nr][nc]) {
                    distv[nr][nc] = nd;
                    heap_push((Node){nd, nr, nc});
                }
            }
        } else {
            /* only 4 orthogonal moves; additionally, cannot move into wall from dangerous */
            for (int k = 0; k < 4; k++) {
                int nr = r + dr4[k], nc = c + dc4[k];
                if (nr < 1 || nr > H || nc < 1 || nc > W) continue;

                int destWall = (grid[nr][nc] == '#');
                if (destWall) continue; /* dangerous -> wall forbidden */

                long long step = A_in; /* orthogonal into non-wall */
                long long nd = cur.d + step;
                if (nd < distv[nr][nc]) {
                    distv[nr][nc] = nd;
                    heap_push((Node){nd, nr, nc});
                }
            }
        }
    }

    printf("INF\n");
    return 0;
}
