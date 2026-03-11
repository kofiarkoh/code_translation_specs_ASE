// AOJ 2376 DisconnectedGame
// 2018.2.24 bal4u

#include <stdio.h>
#include <stdlib.h>

int  n;
char map[1003][1003];
char mk[1003];
int  x, f[2];

void dfs(int r)
{
	int c;

	if (mk[r]) return;
	mk[r] = 1;
	x++;
	for (c = 0; c < n; c++) if (map[r][c] & 1) dfs(c);
}

int main()
{
	int r, c, cnt;
	char buf[10], *p;

	fgets(buf, 10, stdin), n = atoi(buf);
	cnt = (n*(n-1)) >> 1;
	for (r = 0; r < n; r++) {
		fgets(p=map[r], 1003, stdin);
		for (c = 0; c < r; c++) cnt -= (*p++ == 'Y');
	}

	for (r = 0; r < n; r++) {
		if (mk[r]) continue;
		x = 0, dfs(r), f[x & 1]++;
	}

	if ((f[0] == 1 && f[1] == 2) || (cnt & 1) == (f[0] || f[1] != 2)) puts("Taro");
	else puts("Hanako");
	return 0;
}
