#include <stdio.h>

#define INF 1000000000

#define min(a, b) (((a) < (b)) ? (a) : (b))

const int di[2][9] = {{-2, -1, -1, 0, 0, 0, 1, 1, 2}, {-2, -1, -1, 0, 0, 0, 1, 1, 2}};
const int dj[2][9] = {{1, 1, 2, 1, 2, 3, 1, 2, 1}, {-1, -2, -1, -3, -2, -1, -2, -1, -1}};

typedef struct {
	int i, j, f, d;
} PairCoordDis;

PairCoordDis heap[32400];
int hsize;

inline void insert(int i, int j, int f, int d) {
	int ii;
	for (ii = hsize++; ii > 0 && d < heap[ii - 1 >> 1].d; ii = ii - 1 >> 1)
		heap[ii] = heap[ii - 1 >> 1];
	heap[ii] = (PairCoordDis){i, j, f, d};
}

inline int calcNextIdx(int i) {
	return i * 2 + 1 + (i * 2 + 2 < hsize && heap[i * 2 + 1].d > heap[i * 2 + 2].d);
}

inline PairCoordDis delete(void) {
	int i;
	int nextIdx;
	const PairCoordDis root = heap[0], last = heap[--hsize];

	for (i = 0; i * 2 + 1 < hsize && last.d > heap[nextIdx = calcNextIdx(i)].d; i = nextIdx)
		heap[i] = heap[nextIdx];
	heap[i] = last;

	return root;
}

char field[60][30];
int cost[60][30];
int d[60][30][2];

void dijkstra(int h, int w, int si, int sj, int sf) {
	int i, j;

	for (i = 0; i < h; i++)
		for (j = 0; j < w; j++)
			d[i][j][0] = d[i][j][1] = INF;

	d[si][sj][sf] = 0;
	insert(si, sj, sf, 0);
	while (hsize > 0) {
		const PairCoordDis fromP = delete();
		const int nf = fromP.f ^ 1;

		if (fromP.d > d[fromP.i][fromP.j][fromP.f])
			continue;

		for (i = 0; i < 9; i++) {
			const int ni = fromP.i + di[fromP.f][i], nj = fromP.j + dj[fromP.f][i];
			if (ni >= 0 && ni < h && nj >= 0 && nj < w)
				if (fromP.d + cost[ni][nj] < d[ni][nj][nf]) {
					d[ni][nj][nf] = fromP.d + cost[ni][nj];
					insert(ni, nj, nf, d[ni][nj][nf]);
				}
		}
	}
}

int main(void) {
	int i, j, k;
	int w, h;

	while (scanf("%d %d", &w, &h), w) {
		int min;

		for (i = 0; i < h; i++)
			for (j = 0; j < w; j++) {
				scanf(" %c", field[i] + j);
				cost[i][j] = field[i][j] == 'X' ? INF : field[i][j] <= '9' ? field[i][j] - '0' : 0;
			}

		min = INF;
		for (i = 0; i < w; i++)
			if (field[h - 1][i] == 'S')
				for (j = 0; j < 2; j++) {
					dijkstra(h, w, h - 1, i, j);
					for (k = 0; k < w; k++)
						if (field[0][k] == 'T') {
							min = min(min, d[0][k][0]);
							min = min(min, d[0][k][1]);
						}
				}

		printf("%d\n", min == INF ? -1 : min);
	}

	return 0;
}