// AOJ 1533: Caterpillar
// 2019.2.28 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

// 数値のハッシュ関数
#define HASHSIZ 99991  // 499979
long long hash[HASHSIZ + 5], *hashend = hash + HASHSIZ;

// 数値をハッシュテーブルに登録。成功からリターン値が1。
int insert(long long n)
{
	long long *p = hash + (int)(n % HASHSIZ);
	while (*p) {
		if (*p == n) return 0;
		if (++p == hashend) p = hash;
	}
	*p = n;
	return 1;
}

#define QMAX 10000
typedef struct { long long p; int s, lv; } Q;
Q q[QMAX + 2]; int top, end;
int mv[4][2] = { {-1,0},{1,0},{0,-1},{0,1} };

int H, W, N;
char map[15][15];

long long next(int nr, int nc, long long p)
{
	int i, j, r[6], c[6];
	r[0] = nr, c[0] = nc;                                    // Sの位置
	r[1] = ((p >> 40) >> 4) & 0xf, c[1] = (p >> 40) & 0xf;   // aの位置
	r[2] = ((p >> 32) >> 4) & 0xf, c[2] = (p >> 32) & 0xf;   // bの位置
	r[3] = ((p >> 24) >> 4) & 0xf, c[3] = (p >> 24) & 0xf;   // cの位置
	r[4] = ((p >> 16) >> 4) & 0xf, c[4] = (p >> 16) & 0xf;   // dの位置
	r[5] = ((p >>  8) >> 4) & 0xf, c[5] = (p >>  8) & 0xf;   // eの位置

	for (i = 0; i < 5; i++) for (j = i + 1; j < 6; j++) {
		if (r[i] == r[j] && c[i] == c[j]) return 0;
	}

	return ((long long)((r[0] << 4) | c[0]) << 40) |
		   ((long long)((r[1] << 4) | c[1]) << 32) |
		   ((long long)((r[2] << 4) | c[2]) << 24) |
		   ((long long)((r[3] << 4) | c[3]) << 16) |
		   ((long long)((r[4] << 4) | c[4]) << 8 ) |
		   ((long long)((r[5] << 4) | c[5]));
}

int bfs(long long p)
{
	int i, r, c, rr, cc, s, lv;
	long long pp;

	q[0].p = p, q[0].s = 0, q[0].lv = 1;
	top = 0, end = 1;
	while (top != end) {
		p = q[top].p, s = q[top].s, lv = q[top].lv;
		if (++top == QMAX) top = 0;

		r = ((p >> 40) >> 4) & 0xf, c = (p >> 40) & 0xf;   // 頭の位置
//		printf("(%d,%d) s=%d, level=%d\n", r, c, s, lv);
		if (map[r][c] == lv+'0') lv++;
		if (lv > N) return s;
		if (!insert(p | ((long long)lv << 48))) continue;

		s++;
		for (i = 0; i < 4; i++) {
			rr = r + mv[i][0], cc = c + mv[i][1];
			if (rr < 0 || rr >= H || cc < 0 || cc >= W || map[rr][cc] == '#') continue;
			if (!(pp = next(rr, cc, p))) continue;
			q[end].p = pp, q[end].s = s, q[end].lv = lv;
			if (++end == QMAX) end = 0;
		}
	}
	return -1;
}

int main()
{
	int r, c;
	long long p;

	scanf("%d%d%d", &H, &W, &N);
	p = 0; for (r = 0; r < H; r++) {
		scanf("%s", map[r]);
		for (c = 0; c < W; c++) {
			switch (map[r][c]) {
			case 'S': p |= ((long long)((r << 4) | c) << 40); break;
			case 'a': p |= ((long long)((r << 4) | c) << 32); break;
			case 'b': p |= ((long long)((r << 4) | c) << 24); break;
			case 'c': p |= ((long long)((r << 4) | c) << 16); break;
			case 'd': p |= ((long long)((r << 4) | c) <<  8); break;
			case 'e': p |= ((long long)((r << 4) | c)      ); break;
			}
			if (isalpha(map[r][c])) map[r][c] = '.';
		}
	}
	printf("%d\n", bfs(p));
	return 0;
}
