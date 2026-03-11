// AOJ DPL_2_A Traveling Salesman Problem
// 2018.5.5 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif

int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

#define INF 0x03030303

#define MAX_V  16
int dp[1<<MAX_V][MAX_V];
int d[MAX_V][MAX_V];

int traveling_salesman(int V)
{
	int s, u, v, x;

	memset(dp, INF, sizeof(dp));
	dp[(1<<V)-1][0] = 0;
	for (s = (1<<V)-2; s >= 0; s--) {
		for (v = 0; v < V; v++) {
			for (u = 0; u < V; u++) {
				if ((s>>u) & 1) continue;
				x = dp[s | (1<<u)][u] + d[v][u];
				if (x < dp[s][v]) dp[s][v] = x;
			}
		}
	}
	return dp[0][0];
}

int main()
{
	int V, E, ans;

	memset(d, INF, sizeof(d));
	V = in(), E = in();
	while (E--) {
		int s = in(), t = in();
		d[s][t] = in();
	}
	ans = traveling_salesman(V);
	if (ans >= INF) puts("-1");
	else printf("%d\n", ans);
	return 0;
}
