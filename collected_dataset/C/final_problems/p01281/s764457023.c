// AOJ 2163 Tatami
// 2019.3.10 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int h, w, ans;
int map[22][22];
int tbl[200], id;

int ok(int r, int c)
{
	if (r == 0 || c == 0) return 1;
	tbl[map[r][c]] = ++id;
	if (tbl[map[r][c - 1]] == id) return 1;
	tbl[map[r][c - 1]] = id;
	if (tbl[map[r - 1][c]] == id) return 1;
	tbl[map[r - 1][c]] = id;
	return tbl[map[r - 1][c - 1]] == id;
}

void check(int k, int r, int c)
{
	if (c == w) {
		if (r == h - 1) ans++;
		else check(k, r + 1, 0);
		return;
	}
	if (map[r][c]) check(k, r, c + 1);
	else {
		if (r + 1 < h && !map[r + 1][c]) {
			map[r][c] = map[r + 1][c] = k;
			if (ok(r, c)) check(k + 1, r, c + 1);
			map[r][c] = map[r + 1][c] = 0;
		}
		if (c + 1 < w && !map[r][c + 1]) {
			map[r][c] = map[r][c + 1] = k;
			if (ok(r, c)) check(k + 1, r, c + 2);
			map[r][c] = map[r][c + 1] = 0;
		}
	}
}

int main()
{
	while (scanf("%d%d", &h, &w) && (h | w)) {
		if (h == w) putchar(h & 1 ? '0' : '2');
		else {
			ans = id = 0;
			memset(map, 0, sizeof(map));
			memset(tbl, 0, sizeof(tbl));
			check(1, 0, 0);
			printf("%d", ans);
		}
		putchar('\n');
	}
	return 0;
}
