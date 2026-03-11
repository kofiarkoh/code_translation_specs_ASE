// AOJ 2021: Princess in Danger
// 2017.12.11 bal4u@uu

#include <stdio.h>
#include <string.h>

#define INF 0x07070707

int n;
char f[101];
int dist[101][101];

void warshallFloyd()
{
	int i, j, k, t;

    for (k = 0; k < n; k++)	for (i = 0; i < n; i++) for (j = 0; j < n; j++) {
		t = dist[i][k] + dist[k][j];
		if (t < dist[i][j]) dist[i][j] = t;
	}
}

char buf[400], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf);
	p++;
	return n;
}

int main()
{
	int m, l, k, a, h, i, j, x, y, t, ans;

	while (fgets(p=buf, 60, stdin) && *p != '0') {
		n = getint(), m = getint(), l = getint();
		k = getint(), a = getint(), h = getint();

		memset(f, 0, sizeof(f));
		f[a] = f[h] = 1;
		if (l > 0) {
			fgets(p=buf, sizeof(buf), stdin);
			while (l--) f[getint()] = 1;
		}

		memset(dist, 0x07, sizeof(dist));
		for (i = 0; i < n; i++) dist[i][i] = 0;
		while (k--) {
			fgets(p=buf, 20, stdin);
			x = getint(), y = getint(), t = getint();
			dist[x][y] = dist[y][x] = t;
		}

		warshallFloyd();
		for (i = 0; i < n; i++) for (j = i+1; j < n; j++) {
			if (!f[i] || !f[j] || dist[i][j] > m)
				dist[i][j] = dist[j][i] = INF;
		}
		warshallFloyd();

		ans = dist[a][h];
		if (ans == INF) puts("Help!");
		else {
			if (ans > m) ans = (ans << 1) - m;
			printf("%d\n", ans);
		}
	}
	return 0;
}