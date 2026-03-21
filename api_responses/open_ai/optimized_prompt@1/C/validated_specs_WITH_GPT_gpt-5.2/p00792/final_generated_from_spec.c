#include <stdio.h>
#include <string.h>

int main(void) {
    int n;
    while (scanf("%d", &n) == 1) {
        if (n < 0) break;

        /* Fixed 4D boolean grid, indices expected 0..8 */
        static unsigned char mir[9][9][9][9];
        memset(mir, 0, sizeof(mir));

        for (int i = 0; i < n; i++) {
            char s[256];
            int x, y;
            if (scanf("%255s %d %d", s, &x, &y) != 3) return 0;

            if (s[0] == 'x') {
                /* segment (x,y) -> (x+1,y) */
                mir[x][y][x + 1][y] = 1;
            } else {
                /* segment (x,y) -> (x,y+1) */
                mir[x][y][x][y + 1] = 1;
            }
        }

        int X = 50, Y = 0;
        int d = 0;
        int v = 1;
        int wall = 1;

        /* Direction increments */
        const int dx[4] = { +50, -50, -50, +50 };
        const int dy[4] = { +50, +50, -50, -50 };

        /* Reflection mappings */
        const int tr0[4] = { 3, 2, 1, 0 };
        const int tr1[4] = { 1, 0, 3, 2 };

        int i = 0;
        for (;;) {
            /* Termination checks at start of iteration, enabled only once i >= 2 */
            if (i >= 2) {
                if (X == 100 && Y == 50 && d == 0) {
                    /* Special internal-state termination forces output position */
                    X = 75;
                    Y = 25;
                    break;
                }
                if (wall == 1 && (X == 0 || Y == 0 || X == 800 || Y == 800)) {
                    /* Boundary termination gated by wall==1 */
                    break;
                }
            }

            /* Move */
            X += dx[d];
            Y += dy[d];

            /* Assume no reflection for this moved step */
            wall = 1;

            /* Exactly one mirror-crossing test based on current v */
            if (v) {
                int a = X / 100;
                int b = (Y - 50) / 100;
                int c = X / 100;
                int e = (Y + 50) / 100;
                if (mir[a][b][c][e]) {
                    d = tr1[d];
                    wall = 0;
                }
            } else {
                int a = (X - 50) / 100;
                int b = Y / 100;
                int c = (X + 50) / 100;
                int e = Y / 100;
                if (mir[a][b][c][e]) {
                    d = tr0[d];
                    wall = 0;
                }
            }

            /* End of iteration */
            i++;
            v = !v;
        }

        printf("%d %d\n", X, Y);
    }
    return 0;
}
