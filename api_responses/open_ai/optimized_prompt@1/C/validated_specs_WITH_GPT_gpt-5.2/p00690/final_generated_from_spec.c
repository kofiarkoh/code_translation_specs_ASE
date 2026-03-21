#include <stdio.h>
#include <string.h>

#define MAXE 25
#define MAXPATH 12

typedef struct {
    int u, v;
    int w;
} Edge;

static int n, m;
static Edge edges[MAXE];
static int used[MAXE];

static int curPath[MAXPATH];
static int bestPath[MAXPATH];
static int curLen, bestLen;
static int curCnt, bestCnt;

/* Depth-first exploration of edge-simple trails.
   curPath[0..curCnt-1] is the current vertex sequence. */
static void dfs(int at) {
    int i;

    /* Update best only on strictly larger total length (tie-breaking rule). */
    if (curLen > bestLen) {
        bestLen = curLen;
        bestCnt = curCnt;
        memcpy(bestPath, curPath, sizeof(int) * MAXPATH);
    }

    for (i = 0; i < m; i++) {
        if (used[i]) continue;

        if (edges[i].u == at || edges[i].v == at) {
            int nxt = (edges[i].u == at) ? edges[i].v : edges[i].u;

            used[i] = 1;
            curLen += edges[i].w;

            /* Fixed internal storage: if exceeded, behavior is unspecified.
               We keep writing only while within bounds to remain compilable. */
            if (curCnt < MAXPATH) {
                curPath[curCnt] = nxt;
            }
            curCnt++;

            dfs(nxt);

            curCnt--;
            curLen -= edges[i].w;
            used[i] = 0;
        }
    }
}

int main(void) {
    while (scanf("%d %d", &n, &m) == 2) {
        int i;

        if (n == 0) {
            /* m is read but not otherwise used */
            break;
        }

        for (i = 0; i < m; i++) {
            scanf("%d %d %d", &edges[i].u, &edges[i].v, &edges[i].w);
        }

        memset(used, 0, sizeof(used));
        bestLen = 0;
        bestCnt = 0;
        curLen = 0;
        curCnt = 0;

        /* Explore all possible edge-simple routes starting from each edge
           in both directions. */
        for (i = 0; i < m; i++) {
            /* Start with direction u -> v */
            memset(used, 0, sizeof(used));
            used[i] = 1;
            curLen = edges[i].w;
            curCnt = 2;
            curPath[0] = edges[i].u;
            if (1 < MAXPATH) curPath[1] = edges[i].v;
            dfs(edges[i].v);

            /* Start with direction v -> u */
            memset(used, 0, sizeof(used));
            used[i] = 1;
            curLen = edges[i].w;
            curCnt = 2;
            curPath[0] = edges[i].v;
            if (1 < MAXPATH) curPath[1] = edges[i].u;
            dfs(edges[i].u);
        }

        printf("%d\n", bestLen);

        /* If no route with at least one edge is found, bestCnt may be 0.
           The specification states the program still attempts to print a
           starting vertex label; we print "1" in that case. */
        if (bestCnt <= 0) {
            printf("1\n");
        } else {
            for (i = 0; i < bestCnt; i++) {
                if (i) putchar(' ');
                printf("%d", bestPath[i]);
            }
            putchar('\n');
        }
    }

    return 0;
}
