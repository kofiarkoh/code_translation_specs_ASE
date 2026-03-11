// AOJ 2134: Deadly Dice Game
// 2017.11.9 bal4u@uu

#include <stdio.h>
#include <string.h>

double a[2][2002];

char buf[2002], *bp;
int getint()
{
	int n = 0;
	while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);
	return n;
}

int main()
{
	int n, t, i, j, k, red;
	double p, ans;
	
	while (fgets(bp=buf, 20, stdin) && *bp != '0') {
		n = getint(), bp++, t = getint();
		memset(a, 0, sizeof(a));
		fgets(bp=buf, sizeof(buf), stdin);
		for (i = 0; i < n; i++) a[0][i] = (*bp++ == 'R');
		for (red = 0, i = 0; i < t; i++, red = !red) {
			for (j = 0; j < n; j++) {
				for (p = 0, k = 1; k <= 6; k++) {
					int jj = j + k;
					if (jj >= n) jj -= n;
					p += a[red][jj]/6;
				}
				a[!red][j] = p;
			}
		}
		for (ans = 0, i = 0; i < n; i++) if (a[red][i] > ans) ans = a[red][i];
		printf("%.8f\n", ans);
	}
	return 0;
}