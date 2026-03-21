#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>

#define MAX_HW 512
#define MAX_ROWBUF 512 /* at most 511 chars + '\0' as read */

int main(void) {
    int H, W;
    if (scanf("%d %d", &H, &W) != 2) {
        return 1; /* no defined output */
    }

    /* Bounds/assumptions violations are undefined per spec; we do not add checks. */

    static char grid[MAX_HW][MAX_HW];
    char rowbuf[MAX_ROWBUF];

    for (int i = 0; i < H; i++) {
        if (scanf("%511s", rowbuf) != 1) {
            return 1; /* no defined output */
        }
        for (int j = 0; j < W; j++) {
            grid[i][j] = rowbuf[j]; /* spec: row length at least W */
        }
    }

    const int N = H * W;
    /* Use a byte visited array; N <= 512*512. */
    uint8_t *visited = NULL;
    int *queue = NULL;

    if (N > 0) {
        visited = (uint8_t *)calloc((size_t)N, 1);
        queue = (int *)malloc((size_t)N * sizeof(int));
        if (!visited || !queue) {
            free(visited);
            free(queue);
            return 1; /* not specified; choose non-zero exit with no defined output */
        }
    }

    uint64_t total = 0;

    for (int si = 0; si < H; si++) {
        for (int sj = 0; sj < W; sj++) {
            int sidx = si * W + sj;
            if (N > 0 && visited[sidx]) continue;

            /* BFS/queue over region defined by edges between different characters. */
            uint64_t dots = 0, nondots = 0;
            int qh = 0, qt = 0;

            if (N > 0) {
                visited[sidx] = 1;
                queue[qt++] = sidx;
            }

            while (qh < qt) {
                int idx = queue[qh++];
                int i = idx / W;
                int j = idx % W;

                char c = grid[i][j];
                if (c == '.') dots++;
                else nondots++;

                /* Explore 4-neighbors where neighbor char differs. */
                /* Up */
                if (i > 0) {
                    int ni = i - 1, nj = j;
                    int nidx = ni * W + nj;
                    if (!visited[nidx] && grid[ni][nj] != c) {
                        visited[nidx] = 1;
                        queue[qt++] = nidx;
                    }
                }
                /* Down */
                if (i + 1 < H) {
                    int ni = i + 1, nj = j;
                    int nidx = ni * W + nj;
                    if (!visited[nidx] && grid[ni][nj] != c) {
                        visited[nidx] = 1;
                        queue[qt++] = nidx;
                    }
                }
                /* Left */
                if (j > 0) {
                    int ni = i, nj = j - 1;
                    int nidx = ni * W + nj;
                    if (!visited[nidx] && grid[ni][nj] != c) {
                        visited[nidx] = 1;
                        queue[qt++] = nidx;
                    }
                }
                /* Right */
                if (j + 1 < W) {
                    int ni = i, nj = j + 1;
                    int nidx = ni * W + nj;
                    if (!visited[nidx] && grid[ni][nj] != c) {
                        visited[nidx] = 1;
                        queue[qt++] = nidx;
                    }
                }
            }

            total += dots * nondots; /* 64-bit unsigned arithmetic */
        }
    }

    free(visited);
    free(queue);

    printf("%" PRIu64 "\n", total);
    return 0;
}
