#include <stdio.h>
#include <stdlib.h>

#define INIT_CAP 10
#define INF 1000000000

#define min(a, b) (((a) < (b)) ? (a) : (b))

const int di[4] = {-1, 0, 1, 0};
const int dj[4] = {0, 1, 0, -1};

typedef struct {
	int to, w;
} Edge;

typedef struct {
	Edge *elem;
	int cap, size;
} VecEdge;

typedef struct {
	int num, d;
} PNumDis;

VecEdge adj[30 * 30 * 4];
PNumDis heap[30 * 30 * 4 * 4];
int hsize;
int d[30 * 30 * 4];

int w, h;
int field[30][30];

void init(int n) {
	while (n--)
		adj[n] = (VecEdge){malloc(sizeof(Edge) * INIT_CAP), INIT_CAP, 0};
}

inline void add(int from, int to, int w) {
	if (adj[from].size == adj[from].cap) {
		adj[from].cap <<= 1;
		adj[from].elem = realloc(adj[from].elem, sizeof(Edge) * adj[from].cap);
	}
	adj[from].elem[adj[from].size++] = (Edge){to, w};
}

void freeAll(int n) {
	while (n--)
		free(adj[n].elem);
}

inline void insert(int num, int d) {
	int i;
	for (i = hsize++; i > 0 && d < heap[i - 1 >> 1].d; i = i - 1 >> 1)
		heap[i] = heap[i - 1 >> 1];
	heap[i] = (PNumDis){num, d};
}

inline int calcNextIdx(int i) {
	return i * 2 + 1 + (i * 2 + 2 < hsize && heap[i * 2 + 1].d > heap[i * 2 + 2].d);
}

inline PNumDis delete(void) {
	int i;
	int nextIdx;
	const PNumDis root = heap[0], last = heap[--hsize];

	for (i = 0; i * 2 + 1 < hsize && last.d > heap[nextIdx = calcNextIdx(i)].d; i = nextIdx)
		heap[i] = heap[nextIdx];
	heap[i] = last;

	return root;
}

void dijkstra(int n, int s) {
	int i;

	for (i = 0; i < n; i++)
		d[i] = INF;

	d[s] = 0;
	insert(s, 0);
	while (hsize > 0) {
		const PNumDis fromP = delete();

		if (fromP.d > d[fromP.num])
			continue;

		for (i = 0; i < adj[fromP.num].size; i++) {
			const Edge e = adj[fromP.num].elem[i];
			if (fromP.d + e.w < d[e.to]) {
				d[e.to] = fromP.d + e.w;
				insert(e.to, d[e.to]);
			}
		}
	}
}

int main(void) {
	int i, j, k, l;

	while (scanf("%d %d", &w, &h), w) {
		int c[4];

		for (i = 0; i < h; i++)
			for (j = 0; j < w; j++)
				scanf("%d", field[i] + j);

		for (i = 0; i < 4; i++)
			scanf("%d", c + i);

		init(h * w * 4);
		for (i = 0; i < h; i++)
			for (j = 0; j < w; j++)
				for (k = 0; k < 4; k++)
					for (l = 0; l < 4; l++) {
						int nd = k + l & 3;
						int ni = i + di[nd], nj = j + dj[nd];
						if (ni >= 0 && ni < h && nj >= 0 && nj < w)
							add(i * w * 4 + j * 4 + k, ni * w * 4 + nj * 4 + nd, l == field[i][j] ? 0 : c[l]);
					}

		dijkstra(h * w * 4, 1);

		printf("%d\n", min(d[(h - 1) * w * 4 + (w - 1) * 4 + 1], d[(h - 1) * w * 4 + (w - 1) * 4 + 2]));

		freeAll(h * w * 4);
	}

	return 0;
}