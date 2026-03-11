// AOJ 2137 Time Trial
// 2019.4.4 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned char uchar;
typedef struct { uchar pos[7]; int step; } Q;
Q que[1200000]; int top, end;
char encode[128];

char map[18][18]; int H, W;
int mv[4][2] = { {-1,0}, {0,1}, {1,0}, {0,-1} };

// 数値のハッシュ関数
#define HASHSIZ 2000081
unsigned hash[HASHSIZ + 5], *hashend = hash + HASHSIZ;

int insert(uchar *pos)
{
	int i;
	unsigned x, *p;

	x = 0; for (i = 0; i < 4; i++) x = (x << 8) | pos[i];
	p = hash + (int)(x % HASHSIZ);
	while (*p) {
		if (*p == x) return 0;
		if (++p == hashend) p = hash;
	}
	*p = x;
	return 1;
}

void sort(uchar *p)
{
	uchar t;
	if (p[0] > p[1]) t = p[0], p[0] = p[1], p[1] = t;
	if (p[1] > p[2]) {
		t = p[1], p[1] = p[2], p[2] = t;
		if (p[0] > p[1]) t = p[0], p[0] = p[1], p[1] = t;
	}
}

int bfs(uchar *goal, uchar *init)
{
	int i, r, c, nr, nc, ic, step;
	uchar pos[4], npos[4], np;

	sort(goal), sort(init + 1);
	top = 0, end = 1;
	memcpy(que[0].pos, init, 4), que[0].step = 0;
	while (top != end) {
		memcpy(pos, que[top].pos, 4), step = que[top].step, top++;

		if (goal[0] == pos[1] && goal[1] == pos[2] && goal[2] == pos[3]) return step;

		if (!insert(pos)) continue;   // 同一状態がすでにあった

		r = pos[0] >> 4, c = pos[0] & 0xf;
		for (i = 0; i < 4; i++) {
			memcpy(npos, pos, 4);
			nr = r + mv[i][0], nc = c + mv[i][1], np = (nr << 4) | nc;
			if (map[nr][nc] == '#') continue;
			for (ic = 1; ic < 4; ic++) if (pos[ic] == np) break;
			if (ic < 4) {  // 移動先がRock
				uchar cr = nr + mv[i][0], cc = nc + mv[i][1], cp = (cr << 4) | cc;
				if (map[cr][cc] == '#') continue;    // コンテナの先が壁か別のコンテナ？
				if (pos[1] == cp || pos[2] == cp || pos[3] == cp) continue;
				npos[ic] = cp;
			}
			npos[0] = np, que[end].step = step + 1;
			sort(npos + 1);
			memcpy(que[end++].pos, npos, 4);
		}
	}
	return -1;
}

int main()
{
	int k, r, c, nc, np;
	uchar goal[3], pos[4], p;

	memset(encode, -1, sizeof(encode));
	encode['@'] = 0, encode['*'] = 1, encode['_'] = 4;
	while (scanf("%d%d", &W, &H) && W) {
		memset(hash, 0, sizeof(hash));
		np = nc = 0;
		for (r = 0; r < H; r++) {
			scanf("%s", map[r]);
			for (c = 0; c < W; c++) {
				if ((k = encode[map[r][c]]) < 0) continue;
				p = (r << 4) | c;
				if (k == 0) pos[k] = p;              // Hero 
				else if (k < 4) pos[k + np++] = p;   // Rock
				else goal[nc++] = p;                 // Marked Square
				map[r][c] = '.';
			}
		}
		printf("%d\n", bfs(goal, pos));
	}
	return 0;
}
