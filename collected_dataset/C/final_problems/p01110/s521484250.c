// AOJ 1625 Origami, or the art of folding paper
// 2019.3.25 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//// 入力処理
#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()  // 非負整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int H, W;
int map[2][70][70];

int main()
{
	int i, t, p, r, c, id;

	while (W = in()) {
		H = in(), t = in(), p = in();
		memset(map, 0, sizeof(map));
		for (r = 0; r < H; r++) for (c = 0; c < W; c++) map[0][r][c] = 1;
		for (id = 0, i = 0; i < t; i++, id = !id) {
			int d = in(), e = in();
			if (d == 1) {	// 横方向に折る
				for (r = 0; r < H; r++) for (c = e; c < 2 * e; c++)
					map[id][r][c] += map[id][r][2 * e - c - 1];
				if (W - e > e) W -= e; else W = e;
				memset(map[!id], 0, sizeof(map[0]));
				for (r = 0; r < H; r++) for (c = 0; c < W; c++)
					map[!id][r][c] = map[id][r][c + e];
			}
			else {        // 縦に（上下に）折る
				for (r = e; r < 2 * e; r++) for (c = 0; c < W; c++)
					map[id][r][c] += map[id][2 * e - r - 1][c];
				if (H - e > e) H -= e; else H = e;
				memset(map[!id], 0, sizeof(map[0]));
				for (r = 0; r < H; r++) for (c = 0; c < W; c++)
					map[!id][r][c] = map[id][r + e][c];
			}
		}
		for (i = 0; i < p; i++) {
			c = in(), r = in();
			printf("%d\n", map[id][r][c]);
		}
	}
	return 0;
}
