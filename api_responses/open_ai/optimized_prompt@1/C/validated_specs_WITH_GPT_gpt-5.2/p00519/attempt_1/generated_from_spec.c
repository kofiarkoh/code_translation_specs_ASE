#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAXN 5000
#define MAXL 5000

/* We model states (town t, remaining allowance r) with Dijkstra over:
   - zero-cost edges: (t,r)->(t',r-1) for each neighbor t' if r>0
   - positive-cost edge: (t,r)->(t,L[t]) with +F[t]
   Initial: (0,0) cost 0. Goal: any (N-1, r), take minimum.
*/

typedef struct {
    int to;
    int next;
} Edge;

typedef struct {
    int state; /* encoded as t*(MAXL+1)+r */
    int dist;
} HeapNode;

typedef struct {
    HeapNode *a;
    int size;
    int cap;
} MinHeap;

static void heap_init(MinHeap *h, int cap) {
    h->a = (HeapNode *)malloc((size_t)cap * sizeof(HeapNode));
    h->size = 0;
    h->cap = cap;
}

static void heap_swap(HeapNode *x, HeapNode *y) {
    HeapNode tmp = *x;
    *x = *y;
    *y = tmp;
}

static void heap_push(MinHeap *h, int state, int dist) {
    /* Behavior undefined if allocation fails; cap chosen large enough for typical use.
       If cap is exceeded, we grow (still undefined on malloc failure). */
    if (h->size >= h->cap) {
        int newcap = h->cap * 2 + 1;
        HeapNode *na = (HeapNode *)realloc(h->a, (size_t)newcap * sizeof(HeapNode));
        h->a = na;
        h->cap = newcap;
    }
    int i = h->size++;
    h->a[i].state = state;
    h->a[i].dist = dist;
    while (i > 0) {
        int p = (i - 1) / 2;
        if (h->a[p].dist <= h->a[i].dist) break;
        heap_swap(&h->a[p], &h->a[i]);
        i = p;
    }
}

static int heap_empty(MinHeap *h) {
    return h->size == 0;
}

static HeapNode heap_pop(MinHeap *h) {
    HeapNode ret = h->a[0];
    h->size--;
    if (h->size > 0) {
        h->a[0] = h->a[h->size];
        int i = 0;
        for (;;) {
            int l = i * 2 + 1;
            int r = l + 1;
            int m = i;
            if (l < h->size && h->a[l].dist < h->a[m].dist) m = l;
            if (r < h->size && h->a[r].dist < h->a[m].dist) m = r;
            if (m == i) break;
            heap_swap(&h->a[i], &h->a[m]);
            i = m;
        }
    }
    return ret;
}

int main(void) {
    int N, M;
    if (scanf("%d %d", &N, &M) != 2) {
        return 1; /* no cost output */
    }

    /* Preconditions say N<=5000; if violated behavior is undefined. */
    int *fare = (int *)malloc((size_t)N * sizeof(int));
    int *lim  = (int *)malloc((size_t)N * sizeof(int));

    for (int i = 0; i < N; i++) {
        if (scanf("%d %d", &fare[i], &lim[i]) != 2) {
            return 1; /* no cost output */
        }
        /* Preconditions say nonnegative and lim within 0..5000; if violated undefined. */
    }

    int *head = (int *)malloc((size_t)N * sizeof(int));
    for (int i = 0; i < N; i++) head[i] = -1;

    /* Undirected: store 2*M edges. */
    Edge *edges = (Edge *)malloc((size_t)2 * (size_t)M * sizeof(Edge));
    int ec = 0;

    for (int i = 0; i < M; i++) {
        int u, v;
        if (scanf("%d %d", &u, &v) != 2) {
            return 1; /* no cost output */
        }
        /* Preconditions say endpoints valid; if violated undefined. */
        edges[ec].to = v;
        edges[ec].next = head[u];
        head[u] = ec++;
        edges[ec].to = u;
        edges[ec].next = head[v];
        head[v] = ec++;
    }

    const int L1 = MAXL + 1;
    const int total_states = N * L1;

    int *dist = (int *)malloc((size_t)total_states * sizeof(int));
    for (int i = 0; i < total_states; i++) dist[i] = INT_MAX;

    MinHeap pq;
    heap_init(&pq, 1024);

    int start = 0 * L1 + 0;
    dist[start] = 0;
    heap_push(&pq, start, 0);

    int best_goal = INT_MAX;

    while (!heap_empty(&pq)) {
        HeapNode cur = heap_pop(&pq);
        int s = cur.state;
        int d = cur.dist;
        if (d != dist[s]) continue; /* stale */

        int t = s / L1;
        int r = s % L1;

        if (t == N - 1) {
            /* Dijkstra property: first time we pop a goal state is globally minimal. */
            best_goal = d;
            break;
        }

        /* Purchase taxi in current town: (t, L[t]) with +fare[t] */
        {
            int nr = lim[t];
            int ns = t * L1 + nr;
            /* Preconditions guarantee total fare fits in signed int; use int arithmetic. */
            int nd = d + fare[t];
            if (nd < dist[ns]) {
                dist[ns] = nd;
                heap_push(&pq, ns, nd);
            }
        }

        /* Free traversal if r>0: to each neighbor with r-1 at same cost */
        if (r > 0) {
            int nr = r - 1;
            for (int e = head[t]; e != -1; e = edges[e].next) {
                int tt = edges[e].to;
                int ns = tt * L1 + nr;
                if (d < dist[ns]) {
                    dist[ns] = d;
                    heap_push(&pq, ns, d);
                }
            }
        }
    }

    if (best_goal != INT_MAX) {
        printf("%d\n", best_goal);
    }
    /* If unreachable: terminate normally without printing a cost. */

    return 0;
}
