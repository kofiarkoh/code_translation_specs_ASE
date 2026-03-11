// AOJ 1243 Weather Forecast
// 2018/1/21

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int n, ans;
int date[367];
int cd[4];
char mk[367][10][8][8][8][8];
int pos[12] = {0,1,2,3,0,4,5,6,0,7,8,9};
int mv[9][2] = {{0,0},{0,1},{0,-1},{1,0},{-1,0},{0,2},{0,-2},{2,0},{-2,0}};

char check(int k, int x, int y, int *cd)
{
	int a, b, c, d;

	a = ((x - 1) << 2) + y;
	b = (x << 2) + y;
	c = ((x - 1) << 2) + y + 1;
	d = (x << 2) + y + 1;
	if (x < 1 || x > 3 || y < 1 || y > 3) return 0;
	if (cd[0] >= 7 || cd[1] >= 7 || cd[2] >= 7 || cd[3] >= 7) return 0;
	if (((1<<a) & date[k]) || ((1<<b) & date[k]) || ((1<<c) & date[k]) || ((1<<d) & date[k])) return 0;
	if (mk[k][pos[a]][cd[0]][cd[1]][cd[2]][cd[3]]) return 0;
    return 1;
}

void dfs(int k, int x, int y, int *cd)
{
	int i, j, t, x2, y2;
	int a[5];

	if (k == n) { ans = 1; return; }
	for (i = 0; i < 9; i++) {
		if (!k && i) break;
		t = k + 1;
		x2 = x + mv[i][0];
		y2 = y + mv[i][1];
		for (j = 0; j < 4; j++) a[j] = cd[j] + 1;
		if (x2 == 1 && y2 == 1) a[0] = 0;
		if (x2 == 1 && y2 == 3) a[1] = 0;
		if (x2 == 3 && y2 == 1) a[2] = 0;
		if (x2 == 3 && y2 == 3) a[3] = 0;
		if (check(t, x2, y2, a)) {
			mk[t][pos[((x2-1)<<2)+y2]][a[0]][a[1]][a[2]][a[3]] = 1;
			dfs(t, x2, y2, a);
		}
	}
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
			}
		}

		memset(mk, 0, sizeof(mk));
		memset(cd, 0, sizeof(cd));
		ans = 0;
		dfs(0, 2, 2, cd);
		putchar('0'+ans), putchar('\n');
	}
	return 0;
}
