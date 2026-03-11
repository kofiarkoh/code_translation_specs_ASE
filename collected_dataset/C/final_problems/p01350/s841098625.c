// AOJ 2233 Carrot Tour
// 2018.2.1 bal4u

#include <stdio.h>
#include <string.h>
#include <math.h>

#define EPS 1e-8
#define PI 3.1415926535897932384626433832795

int x[21], y[21];
double d[21][21];
double dp[2][21][21];
int to[21][21][21], hi[21][21];
char f[2][21][21];

#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	if (c == '-') {
		c = getchar_unlocked();
		do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
		while (c >= '0');
		return -n;
	}
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

double angle(int k, int i, int j)
{
	double a, b, c;
	
	a = hypot(x[k]-x[i], y[k]-y[i]);
	b = hypot(x[i]-x[j], y[i]-y[j]);
	c = (double)(x[k]-x[i])*(x[i]-x[j]) + (double)(y[k]-y[i])*(y[i]-y[j]);
	return acos(c/(a*b));
}

int main()
{
	int n, i, j, k, m, k1, k2, ok, ans;
	double r, a, t;
	char buf[30];

	n = in();
	fgets(buf, 30, stdin), sscanf(buf, "%lf%lf", &r, &a);
	a *= PI/180;
	for (i = 0; i < n; i++) x[i] = in(), y[i] = in();

	for (i = 0; i < n; i++) for (j = i+1; j < n; j++)
		d[i][j] = d[j][i] = hypot(x[i]-x[j], y[i]-y[j]);

	for (i = 0; i < n; i++) for (j = 0; j < n; j++) if (j != i) {
		for (k = 0; k < n; k++) if (k != i) {
			if (angle(k, i, j) < a + EPS) to[i][j][hi[i][j]++] = k;
		}
	}

	ok = 0;
	for (j = 1; j < n; j++) if (d[0][j] < r + EPS) {
		ok = 1, dp[0][j][0] = d[0][j], f[0][j][0] = 1;
	}

	ans = 0;
	for (k1 = 0, k2 = 1; ok; k1 = k2, k2 = !k2) {
		ans++;
		ok = 0;
		memset(f[k2], 0, sizeof(f[k2]));
		for (i = 0; i < n; i++) for (j = 0; j < n; j++) {
			if (i == j || !f[k1][i][j]) continue;
			for (m = 0; m < hi[i][j]; m++) {
				k = to[i][j][m];
				if ((t = dp[k1][i][j] + d[i][k]) < r + EPS) {
					if (!f[k2][k][i] || dp[k2][k][i] > t) {
						dp[k2][k][i] = t, f[k2][k][i] = 1, ok = 1;
					}
				}
			}
		}
	}
	printf("%d\n", ans);
	return 0;
}
