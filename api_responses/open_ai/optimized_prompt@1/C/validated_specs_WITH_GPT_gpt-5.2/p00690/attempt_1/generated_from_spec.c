#include <stdio.h>

#define MAXV 12
#define MAXE 25

typedef struct {
    int u, v;
    long long w;
    int used;
} Edge;

static int nV, nE;
static Edge edges[MAXE];

static long long bestLen;
static int bestPath[MAXV];
static int bestPathLen;

static int curPath[MAXV];
static int curPathLen;

static void consider_best(long long curLen) {
    if (curLen > bestLen) {
        bestLen = curLen;
        bestPathLen = curPathLen;
        for (int i = 0; i < curPathLen; i++) bestPath[i] = curPath[i];
    }
}

/* Depth-first exhaustive exploration of all edge-simple trails.
   The current trail is represented by curPath[0..curPathLen-1] and curLen.
   We extend the trail by choosing any unused edge incident to the current end vertex. */
static void dfs_from_end(int endVertex, long long curLen) {
    consider_best(curLen);

    /* Try to extend by any unused incident edge */
    for (int i = 0; i < nE; i++) {
        if (edges[i].used) continue;

        int next = -1;
        if (edges[i].u == endVertex) next = edges[i].v;
        else if (edges[i].v == endVertex) next = edges[i].u;
        else continue;

        /* Route storage capacity is fixed; behavior is unspecified if exceeded.
           We conservatively avoid writing out of bounds. */
        if (curPathLen >= MAXV) continue;

        edges[i].used = 1;
        curPath[curPathLen++] = next;
        dfs_from_end(next, curLen + edges[i].w);
        curPathLen--;
        edges[i].used = 0;
    }
}

int main(void) {
    while (1) {
        if (scanf("%d", &nV) != 1) return 0;
        if (nV == 0) return 0;
        if (scanf("%d", &nE) != 1) return 0;

        for (int i = 0; i < nE; i++) {
            int a, b;
            long long w;
            scanf("%d %d %lld", &a, &b, &w);
            edges[i].u = a;
            edges[i].v = b;
            edges[i].w = w;
            edges[i].used = 0;
        }

        bestLen = 0;
        bestPathLen = 0;

        /* Explore all trails starting from each vertex label in range.
           Vertex labels are assumed to be within the declared range by precondition. */
        for (int start = 0; start < nV; start++) {
            for (int i = 0; i < nE; i++) edges[i].used = 0;

            curPathLen = 1;
            curPath[0] = start;
            dfs_from_end(start, 0);
        }

        /* Output format: maximum length then one corresponding vertex sequence.
           If there are no edges, the sequence is not well-defined by the spec;
           we print a single vertex 0 if possible, otherwise print nothing after length. */
        printf("%lld", bestLen);
        if (bestPathLen > 0) {
            for (int i = 0; i < bestPathLen; i++) {
                printf(" %d", bestPath[i]);
            }
        } else {
            if (nV > 0) printf(" %d", 0);
        }
        printf("\n");
    }
}
