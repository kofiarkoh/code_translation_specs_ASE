// AOJ 2345: Network Reliability
// 2017.10.24 bal4u@uu

#include <stdio.h>

int u[102], v[102];
double dp[16500];

int main()
{
	int n, m, i, j, k, l, lim;
	double p, t;

	scanf("%d%d%lf", &n, &m, &p); p /= 100;
	for (i = 0; i < m; i++) scanf("%d%d", u+i, v+i), u[i]--, v[i]--;
	lim = 1 << n;
	for (i = 1; i < lim; i++) {
		for (l = 1; !(l & i); l<<=1);
		dp[i] = 1;
		for (k = i; k; k = (k - 1) & i) {
			if (k == i || !(k & l)) continue;
			t = dp[k];
			for (j = 0; j < m; j++) {
				if ((i & (1<<u[j])) && (i & (1<<v[j])) && ((k>>u[j]) & 1) != ((k>>v[j]) & 1))
					t *= p;
			}
			dp[i] -= t;
		}
	}
	printf("%.12f\n",dp[lim - 1]);
	return 0;
}