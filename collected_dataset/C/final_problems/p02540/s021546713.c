#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#if 1
#define MAX 212345
#define BLOCK_SIZE 450
#else
#define MAX 100
#define BLOCK_SIZE 2
#endif

#define BLOCK_MAX ((MAX + BLOCK_SIZE - 1) / BLOCK_SIZE)

struct mamegu {
	int x, y;
	int idx;
};

int mikakosi(const void* x, const void* y) {
	struct mamegu a = *(const struct mamegu*)x, b = *(const struct mamegu*)y;
	if (a.x != b.x) return a.x < b.x ? -1 : a.x > b.x;
	return a.y < b.y ? -1 : a.y > b.y;
}

int kiyonon(const void* x, const void* y) {
	struct mamegu a = *(const struct mamegu*)x, b = *(const struct mamegu*)y;
	if (a.y != b.y) return a.y < b.y ? -1 : a.y > b.y;
	return a.x < b.x ? -1 : a.x > b.x;
}

#define KI_MAX (MAX + BLOCK_MAX * BLOCK_MAX)

int ki[KI_MAX];

void ki_init(void) {
	int i;
	for (i = 0; i < KI_MAX; i++) {
		ki[i] = -1;
	}
}

int ki_root(int n) {
	if (ki[n] < 0) return n;
	return ki[n] = ki_root(ki[n]);
}

static int ki_size(int n) {
	return -ki[ki_root(n)];
}

static void ki_merge(int a, int b) {
	int ar = ki_root(a);
	int br = ki_root(b);
	if (ar != br) {
		int as = ki_size(ar);
		int bs = ki_size(br);
		if (as <= bs) {
			ki[br] += ki[ar];
			ki[ar] = br;
		} else {
			ki[ar] += ki[br];
			ki[br] = ar;
		}
	}
}

int N;
struct mamegu mati_x[MAX], mati_y[MAX];

int blockNum;
char chukei_exists[BLOCK_MAX][BLOCK_MAX], chukei_active[BLOCK_MAX][BLOCK_MAX];
int chukei_next_x[BLOCK_MAX][BLOCK_MAX], chukei_next_y[BLOCK_MAX][BLOCK_MAX];
struct horieyui {
	int x, y;
} chukei_right_down[BLOCK_MAX][BLOCK_MAX + 1];

int chukei_delta[KI_MAX];

int main(void) {
	int i, j, k;
	int processed_row;
	ki_init();
	if (scanf("%d", &N) != 1) return 1;
	blockNum = (N + BLOCK_SIZE - 1) / BLOCK_SIZE;
	for (i = 0; i < N; i++) {
		if (scanf("%d%d", &mati_x[i].x, &mati_x[i].y) != 2) return 1;
		mati_x[i].x--;
		mati_x[i].y--;
		mati_x[i].idx = i;
		chukei_exists[mati_x[i].y / BLOCK_SIZE][mati_x[i].x / BLOCK_SIZE] = 1;
	}
	memcpy(mati_y, mati_x, sizeof(*mati_x) * N);
	qsort(mati_x, N, sizeof(*mati_x), mikakosi);
	qsort(mati_y, N, sizeof(*mati_y), kiyonon);
	ki_init();

	/* 縦方向の短冊を処理する */
	for (i = 0; i < N; i += BLOCK_SIZE) {
		for (j = 0; j < BLOCK_SIZE && i + j < N; j++) {
			assert(mati_x[i + j].x == i + j);
			for (k = j + 1; k < BLOCK_SIZE && i + k < N; k++) {
				if (mati_x[i + j].y < mati_x[i + k].y) {
					ki_merge(mati_x[i + j].idx, mati_x[i + k].idx);
				}
			}
		}
	}

	/* 横方向の短冊を処理する */
	for (i = 0; i < N; i += BLOCK_SIZE) {
		for (j = 0; j < BLOCK_SIZE && i + j < N; j++) {
			assert(mati_y[i + j].y == i + j);
			for (k = j + 1; k < BLOCK_SIZE && i + k < N; k++) {
				if (mati_y[i + j].x < mati_y[i + k].x) {
					ki_merge(mati_y[i + j].idx, mati_y[i + k].idx);
				}
			}
		}
	}

	/* 次に存在する位置を判定する */
	for (i = 0; i < blockNum; i++) {
		chukei_next_x[i][blockNum - 1] = blockNum;
		chukei_next_y[blockNum - 1][i] = blockNum;
		for (j = blockNum - 2; j >= 0; j--) {
			chukei_next_x[i][j] = chukei_exists[i][j + 1] ? j + 1 : chukei_next_x[i][j + 1];
			chukei_next_y[j][i] = chukei_exists[j + 1][i] ? j + 1 : chukei_next_y[j + 1][i];
		}
	}
	for (i = blockNum - 1; i >= 0; i--) {
		chukei_right_down[i][blockNum].x = -1;
		chukei_right_down[i][blockNum].y = -1;
		for (j = blockNum - 1; j >= 0; j--) {
			chukei_right_down[i][j] = chukei_right_down[i][j + 1];
			if (chukei_exists[i][j] && j > chukei_right_down[i][j].x) {
				chukei_right_down[i][j].y = i;
				chukei_right_down[i][j].x = j;
			}
			if (i < blockNum - 1 && chukei_right_down[i + 1][j].y > chukei_right_down[i][j].y) {
				chukei_right_down[i][j] = chukei_right_down[i + 1][j];
			}
		}
	}

	/* 中継ノードを接続する */
	processed_row = blockNum;
	for (j = 0; j < blockNum; j++) {
		for (i = chukei_exists[0][j] ? 0 : chukei_next_y[0][j]; i < blockNum; i = chukei_next_y[i][j]) {
			assert(chukei_exists[i][j]);
			if(i + 1 >= processed_row) {
				/* すでに接続処理がされている */
				if (i + 1 < blockNum) {
					struct horieyui yukati = chukei_right_down[i + 1][j + 1];
					if (yukati.y >= 0) {
						ki_merge(N + i * blockNum + j, N + yukati.y * blockNum + yukati.x);
						chukei_active[i][j] = 1;
						chukei_active[yukati.y][yukati.x] = 1;
					}
				}
			} else {
				/* 接続処理を走らせる */
				int exists = 0;
				for (k = i + 1; k < processed_row; k++) {
					int l;
					for (l = chukei_next_x[k][j]; l < blockNum; l = chukei_next_x[k][l]) {
						assert(chukei_exists[k][l]);
						ki_merge(N + i * blockNum + j, N + k * blockNum + l);
						chukei_active[k][l] = 1;
						exists = 1;
					}
				}
				if (processed_row < blockNum) {
					struct horieyui yukati = chukei_right_down[processed_row][j + 1];
					if (yukati.y >= 0) {
						ki_merge(N + i * blockNum + j, N + yukati.y * blockNum + yukati.x);
						chukei_active[yukati.y][yukati.x] = 1;
						exists = 1;
					}
				}
				if (exists) chukei_active[i][j] = 1;
				processed_row = i + 1;
			}
		}
	}

	/* 接続があった中継ノードと街を接続する */
	for (i = 0; i < N; i++) {
		int by = mati_x[i].y / BLOCK_SIZE, bx = mati_x[i].x / BLOCK_SIZE;
		if (chukei_active[by][bx]) {
			ki_merge(mati_x[i].idx, N + by * blockNum + bx);
		}
	}

	/* 中継ノードを数から外す準備をする */
	for (i = 0; i < blockNum; i++) {
		for (j = 0; j < blockNum; j++) {
			if (chukei_active[i][j]) {
				chukei_delta[ki_root(N + i * blockNum + j)]++;
			}
		}
	}

	/* 答えを求める */
	for (i = 0; i < N; i++) {
		int size = ki_size(i);
		int root = ki_root(i);
		printf("%d\n", size - chukei_delta[root]);
	}

	return 0;
}

/*

左に行く → yも上に行く
より上のやつがある → 上位互換

*/

/*

↓こういうので死にそう

....*.....
...*......
..*.......
.*........
*.........
.........*
........*.
.......*..
......*...
.....*....

10
5 1
4 2
3 3
2 4
1 5
10 6
9 7
8 8
7 9
6 10

*/
