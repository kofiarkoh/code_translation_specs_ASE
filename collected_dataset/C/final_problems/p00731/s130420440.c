#include <stdio.h>
#include <stdlib.h>

#define INIT_CAP 10
#define NUM_NODE 20000
#define INF 1000000000

#define min(a, b) (((a) < (b)) ? (a) : (b))

typedef struct {
	int to, w;
} Edge;

typedef struct {
	Edge *elem;
	int cap, size;
} VecEdge;

typedef struct {
	int num;
	int d;
} PairNumDis;

typedef struct {
	int d;
	int idx;
} Node;

VecEdge adj[NUM_NODE];
PairNumDis heap[NUM_NODE];
int hsize;
Node node[NUM_NODE];

void init(int n) {
	while (n--)
		adj[n] = (VecEdge){malloc(sizeof(Edge) * INIT_CAP), INIT_CAP, 0};
}

void add(int from, int to, int w) {
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

int calcNextIdx(int i) {
	return i * 2 + 1 + (i * 2 + 2 < hsize && heap[i * 2 + 1].d > heap[i * 2 + 2].d);
}

PairNumDis delete(void) {
	int i;
	int nextIdx;
	const PairNumDis root = heap[0], last = heap[--hsize];

	for (i = 0; i * 2 + 1 < hsize && last.d > heap[nextIdx = calcNextIdx(i)].d; i = nextIdx) {
		heap[i] = heap[nextIdx];
		node[heap[i].num].idx = i;
	}
	heap[i] = last;
	node[last.num].idx = i;

	return root;
}

void decreaseKey(int num, int d) {
	int i;

	for (i = node[num].idx; i > 0 && d < heap[i - 1 >> 1].d; i = i - 1 >> 1) {
		heap[i] = heap[i - 1 >> 1];
		node[heap[i].num].idx = i;
	}
	heap[i] = (PairNumDis){num, d};
	node[num].idx = i;
}

void dijkstra(int n, int s) {
	int i;

	for (i = 0; i < n; i++) {
		heap[i] = (PairNumDis){i, INF};
		node[i] = (Node){INF, i};
	}
	hsize = n;

	node[s].d = 0;
	decreaseKey(s, 0);
	while (hsize > 0) {
		PairNumDis fromPair = delete();
		for (i = 0; i < adj[fromPair.num].size; i++) {
			Edge e = adj[fromPair.num].elem[i];
			if (fromPair.d + e.w < node[e.to].d) {
				node[e.to].d = fromPair.d + e.w;
				decreaseKey(e.to, node[e.to].d);
			}
		}
	}
}

char field[60][30];

int main(void) {
	int i, j, k, l, m;
	int w, h;
	int min;

	while (scanf("%d %d", &w, &h), w) {
		for (i = 0; i < h; i++)
			for (j = 0; j < w; j++)
				scanf(" %c", field[i] + j);

		init(h * w * 2);
		for (i = 0; i < h; i++)
			for (j = 0; j < w; j++)
				for (k = 0; k < 2; k++)
					for (l = i - 2; l <= i + 2; l++)
						if (l >= 0 && l < h)
							for (m = (k ? j - 3 : j + 1); m <= (k ? j - 1: j + 3); m++)
								if (m >= 0 &&  m < w)
									if (abs(i - l) + abs(j - m) <= 3)
										if (field[l][m] == 'S' || field[l][m] == 'T')
											add(i * w + j + k * h * w, l * w + m + (k ^ 1) * h * w, 0);
										else if (field[l][m] != 'X')
											add(i * w + j + k * h * w, l * w + m + (k ^ 1) * h * w, field[l][m] - '0');

		min = INF;
		for (i = 0; i < w; i++)
			if (field[h - 1][i] == 'S')
				for (j = 0; j < 2; j++) {
					dijkstra(h * w * 2, (h - 1) * w + i + j * h * w);
					for (k = 0; k < w; k++)
						if (field[0][k] == 'T') {
							min = min(min, node[k].d);
							min = min(min, node[k + h * w].d);
						}
				}

		printf("%d\n", min == INF ? -1 : min);

		freeAll(h * w * 2);
	}

	return 0;
}