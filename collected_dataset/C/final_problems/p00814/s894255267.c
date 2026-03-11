// AOJ 1235: Life Line
// 2017.10.22 bal4u@uu

#include <stdio.h>
#include <string.h>

int que[100][2], top;
int map[10][10]; int n;
int mk[10][10];
int zero[46][2], sz;
int mv[6][2] = {{-1,-1},{-1,0},{0,-1},{0,1},{1,0},{1,1}};

int calc(int r0, int c0, int num)
{
	int i, r, c, r2, c2, v;

	top = 0;
	memset(mk, 0, sizeof(mk));
	for (i = 0; i < sz; i++) {
		r = zero[i][0], c = zero[i][1];
		if (map[r][c] == 0) {
			mk[r][c] = 1, que[top][0] = r, que[top++][1] = c;
		}
	}
	while (top) {
		r = que[--top][0], c = que[top][1];
		for (i = 0; i < 6; i++) {
			r2 = r + mv[i][0], c2 = c + mv[i][1];
			if (r2 < 1 || r2 > n || c2 < 1 || c2 > r2 || mk[r2][c2]) continue;
			if (map[r][c] == 0 || map[r2][c2] == map[r][c]) {
				mk[r2][c2] = 1;
				que[top][0] = r2, que[top++][1] = c2;
			}
		}
	}
	for (v = 0, r = 1; r <= n; r++) for (c = 1; c <= r; c++) if (!mk[r][c]) {
		if (map[r][c] == num) v--; else v++;
	}
	return v;
}

int main()
{
	int c, i, j, k, v, ans;

	while (scanf("%d%d", &n, &c) && n > 0) {
		for (sz = 0, i = 1; i <= n; i++) for (j = 1; j <= i; j++) {
			scanf("%d", &v), map[i][j] = v;
			if (!v) zero[sz][0] = i, zero[sz++][1] = j;
		}
		for (ans = -0x7ffffff, k = 0; k < sz; k++) {
			i = zero[k][0], j = zero[k][1];
			map[i][j] = c;
			if ((v = calc(i, j, c)) > ans) ans = v;
			map[i][j] = 0;
		}
		printf("%d\n", ans);
	}
	return 0;
}