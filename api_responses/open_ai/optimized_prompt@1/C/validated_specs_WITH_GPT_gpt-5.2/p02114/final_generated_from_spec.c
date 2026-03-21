#include <stdio.h>

#define MAXN 300                 /* supports n up to 298 with a 1-cell border */
#define MAXCELLS 100010          /* traversal storage limit per specification */

static unsigned char grid[MAXN][MAXN]; /* 0/1: whether cell is 'o' */
static unsigned char vis[MAXN][MAXN];  /* 0/1: visited */
static int qx[MAXCELLS];
static int qy[MAXCELLS];

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* Read n*n characters; accept any whitespace between cells. */
    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= n; j++) {
            char c;
            if (scanf(" %c", &c) != 1) return 0;
            grid[i][j] = (unsigned char)(c == 'o');
        }
    }

    int C = 0;

    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= n; j++) {
            if (grid[i][j] && !vis[i][j]) {
                C++;

                /* BFS flood fill for this component */
                int head = 0, tail = 0;
                vis[i][j] = 1;
                qx[tail] = i;
                qy[tail] = j;
                tail++;

                while (head < tail) {
                    int x = qx[head];
                    int y = qy[head];
                    head++;

                    /* 4-neighbor adjacency */
                    int nx, ny;

                    nx = x - 1; ny = y;
                    if (grid[nx][ny] && !vis[nx][ny]) {
                        vis[nx][ny] = 1;
                        qx[tail] = nx; qy[tail] = ny; tail++;
                    }

                    nx = x + 1; ny = y;
                    if (grid[nx][ny] && !vis[nx][ny]) {
                        vis[nx][ny] = 1;
                        qx[tail] = nx; qy[tail] = ny; tail++;
                    }

                    nx = x; ny = y - 1;
                    if (grid[nx][ny] && !vis[nx][ny]) {
                        vis[nx][ny] = 1;
                        qx[tail] = nx; qy[tail] = ny; tail++;
                    }

                    nx = x; ny = y + 1;
                    if (grid[nx][ny] && !vis[nx][ny]) {
                        vis[nx][ny] = 1;
                        qx[tail] = nx; qy[tail] = ny; tail++;
                    }
                }
            }
        }
    }

    /* Output floor((C-1)/3) with C as component count.
       For C==0, this yields (-1)/3 == 0 in C integer division (trunc toward zero). */
    int ans = (C - 1) / 3;
    printf("%d\n", ans);

    return 0;
}
