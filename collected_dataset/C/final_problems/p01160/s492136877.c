// AOJ 2035: It Prefokery Pio
// 2017.10.9 bal4u@uu

#include <stdio.h>
#include <string.h>

char s[2002], t[2010];

char c[2002][2002], z[2002];    // for LCS
void LCS(char *x, int m, char *y, int n)
{
	int i, j, ret, ret2;

	memset(c, 0, sizeof(c));
	for (i = 1; i <= m; i++) for (j = 1; j <= n; j++) {
		if (x[i-1] == y[j-1]) c[i][j] = c[i-1][j-1] + 1;
		else {
			if (c[i-1][j] >= c[i][j-1]) c[i][j] = c[i-1][j];
			else c[i][j] = c[i][j-1];
		}
	}
	ret2 = ret = c[m][n]-1;
	while (ret >= 0) {
		if (c[m][n] == c[m-1][n-1] + 1) z[ret] = x[m-1], ret--, m--, n--;
		else if (c[m][n] == c[m-1][n]) m--; else n--;
	}
	z[ret2+1] = 0;
}

int main()
{
	int n;
	char *p, *q;

	while (scanf("%s", s) == 1) {
		n = strlen(s);
		p = s, q = t+n; *q = 0;
		while (*p) *--q = *p++;
		LCS(s, n, t, n);
		puts(z);
	}
	return 0;
}