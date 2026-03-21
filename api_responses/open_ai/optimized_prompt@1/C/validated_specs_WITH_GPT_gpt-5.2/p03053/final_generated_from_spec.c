#include <stdio.h>
#include <stdlib.h>

#define MAXH 1000
#define MAXW 1000
#define MAXN (MAXH * MAXW)

int main(void) {
    int H, W;
    if (scanf("%d %d", &H, &W) != 2) return 0;

    static char grid[MAXH][MAXW + 1];
    static int dist[MAXH][MAXW];

    for (int i = 0; i < H; i++) {
        /* Input rows are specified to have exactly W characters of the two cell types. */
        if (scanf("%1000s", grid[i]) != 1) return 0;
    }

    /* Multi-source BFS from all source cells. */
    static int qx[MAXN];
    static int qy[MAXN];
    int head = 0, tail = 0;

    /* Initialize distances: sources to 0, others to -1 (unreached). */
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            if (grid[i][j] == '#') { /* Treat '#' as source cell. */
                dist[i][j] = 0;
                qx[tail] = i;
                qy[tail] = j;
                tail++;
            } else {
                dist[i][j] = -1;
            }
        }
    }

    int maxd = 0;
    while (head < tail) {
        int x = qx[head];
        int y = qy[head];
        head++;

        int d = dist[x][y];
        if (d > maxd) maxd = d;

        /* Orthogonal neighbors only. */
        if (x > 0 && dist[x - 1][y] == -1) {
            dist[x - 1][y] = d + 1;
            qx[tail] = x - 1; qy[tail] = y; tail++;
        }
        if (x + 1 < H && dist[x + 1][y] == -1) {
            dist[x + 1][y] = d + 1;
            qx[tail] = x + 1; qy[tail] = y; tail++;
        }
        if (y > 0 && dist[x][y - 1] == -1) {
            dist[x][y - 1] = d + 1;
            qx[tail] = x; qy[tail] = y - 1; tail++;
        }
        if (y + 1 < W && dist[x][y + 1] == -1) {
            dist[x][y + 1] = d + 1;
            qx[tail] = x; qy[tail] = y + 1; tail++;
        }
    }

    printf("%d\n", maxd);
    return 0;
}
