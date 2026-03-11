// AOJ 1243 Weather Forecast
// 2018/1/21

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int n;
int date[367];
int cd[4];
char mk[367][10][8][8][8][8];
int pos[12] = {0,1,2,3,0,4,5,6,0,7,8,9};
int mv[9][2] = {{0,0},{0,1},{0,-1},{1,0},{-1,0},{0,2},{0,-2},{2,0},{-2,0}};

char check(int k, int x, int y, int *cd)
{
	int a = ((x - 1) << 2) + y;
	if ((date[k] & (1<< a  ))  || (date[k] & (1<<(a+4))) ||
		(date[k] & (1<<(a+1))) || (date[k] & (1<<(a+5)))) return 0;
	if (mk[k][pos[a]][cd[0]][cd[1]][cd[2]][cd[3]]) return 0;
    return 1;
}

int dfs(int k, int x, int y, int *cd)
{
	int i, j, m, x2, y2;
	int a[5];

	if (k == n) return 1;
	m = 9; if (!k++) m = 1;
	for (i = 0; i < m; i++) {
		x2 = x + mv[i][0], y2 = y + mv[i][1];
		if (x2 < 1 || x2 > 3 || y2 < 1 || y2 > 3) continue;
		for (j = 0; j < 4; j++) a[j] = cd[j] + 1;
		if (x2 == 1) {
			if      (y2 == 1) a[0] = 0;
			else if (y2 == 3) a[1] = 0;
		} else if (x2 == 3) {
			if      (y2 == 1) a[2] = 0;
			else if (y2 == 3) a[3] = 0;
		}
		if (a[0] >= 7 || a[1] >= 7 || a[2] >= 7 || a[3] >= 7) continue;

		if (check(k, x2, y2, a)) {
			mk[k][pos[((x2-1)<<2)+y2]][a[0]][a[1]][a[2]][a[3]] = 1;
			if (dfs(k, x2, y2, a)) return 1;
		}
	}
	return 0;
}

int main ()
{
	int i, j;
	char buf[40], *p;

	while (fgets(buf, 10, stdin) && *buf != '0') {
		n = atoi(buf);

		memset(date, 0, sizeof(date));
		for (i = 1; i <= n; i++) {
			fgets(p=buf, 40, stdin);
			for (j = 1; j <= 16; j++) {
				while (*p < '0') p++;
				date[i] |= (*p++ & 0xf) << j;
//				p += 2;
			}
		}

		memset(mk, 0, sizeof(mk));
		memset(cd, 0, sizeof(cd));
		putchar('0' + dfs(0, 2, 2, cd));
		putchar('\n');
	}
	return 0;
}
