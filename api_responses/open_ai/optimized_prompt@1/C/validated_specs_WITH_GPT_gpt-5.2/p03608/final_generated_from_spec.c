#include <stdio.h>

#define MAXN 200
#define MAXR 8
#define INF 1000000000

static int N, M, R;
static int req[MAXR];
static int dist[MAXN][MAXN];

static int used[MAXR];
static int perm[MAXR];
static int best = INF;

static void dfs(int depth, int acc) {
    int i;
    if (depth == R) {
        if (acc < best) best = acc;
        return;
    }

    for (i = 0; i < R; i++) {
        if (!used[i]) {
            int add = 0;
            if (depth > 0) {
                int u = req[perm[depth - 1]];
                int v = req[i];
                add = dist[u][v];
            }
            used[i] = 1;
            perm[depth] = i;
            dfs(depth + 1, acc + add);
            used[i] = 0;
        }
    }
}

int main(void) {
    int i, j, k;
    if (scanf("%d %d %d", &N, &M, &R) != 3) return 0;

    for (i = 0; i < R; i++) {
        scanf("%d", &req[i]);
        req[i]--; /* convert to 0-based */
    }

    for (i = 0; i < N; i++) {
        for (j = 0; j < N; j++) {
            dist[i][j] = (i == j) ? 0 : INF;
        }
    }

    for (i = 0; i < M; i++) {
        int A, B, C;
        scanf("%d %d %d", &A, &B, &C);
        A--; B--;
        if (C < dist[A][B]) {
            dist[A][B] = C;
            dist[B][A] = C;
        }
    }

    /* All-pairs shortest paths (Floyd-Warshall) */
    for (k = 0; k < N; k++) {
        for (i = 0; i < N; i++) {
            for (j = 0; j < N; j++) {
                int via = dist[i][k] + dist[k][j];
                if (via < dist[i][j]) dist[i][j] = via;
            }
        }
    }

    for (i = 0; i < R; i++) used[i] = 0;
    best = INF;
    dfs(0, 0);

    printf("%d\n", best);
    return 0;
}
