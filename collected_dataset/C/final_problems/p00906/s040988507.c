// AOJ 1327 One-Dimensional Cellular Automaton
// 2017.10.25 bal4u
// 2018.2.9

#include <stdio.h>
#include <string.h>

int s[2][52], d[32][52][52];

#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, m, a, b, c, t;
	int i, j, k, l, k1, k2;

	while (n = in()) {
		m = in(), a = in(), b = in(), c = in(), t = in();
		memset(d, 0, sizeof(d));
		for (i = 1; i <= n; i++) s[0][i] = in();
		for (i = 1; i <= n; i++) d[0][i][i+1] = a, d[0][i][i] = b, d[0][i][i-1] = c;
		for (k = 1; (1<<k) <= t; k++) for (i = 1; i <= n; i++) for (j = 1; j <= n; j++) {
			for (l = 1; l <= n; l++) d[k][i][j] += d[k-1][i][l] * d[k-1][l][j];
			d[k][i][j] %= m;
		}

		for (k1 = 0, k2 = 1, k = 0; (1<<k) <= t; k++) if (t & (1<<k)) {
			memset(s[k2], 0, sizeof(s[k2]));
			for (j = 1; j <= n; j++) {
				for (i = 1; i <= n; i++) s[k2][i] += s[k1][j] * d[k][j][i];
			}
			for (i = 1; i <= n; i++) s[k2][i] %= m;
			k1 = k2, k2 = !k2;
		}
		printf("%d", s[k1][1]);	for (i = 2; i <= n; i++) printf(" %d", s[k1][i]);
		putchar('\n');
	}
	return 0;
}

