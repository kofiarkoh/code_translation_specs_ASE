// AOJ DPL_2_B Chinese Postman Problem
// 2018.5.12 bal4u
  
#include <stdio.h>
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

#define INF  0x10101010

int V;
int dist[17][17];
int hi[17], to[17][1005], d[17][1005]; 
int dp[1<<17];

int chinesePostman(int sum)
{
	int i, j, k, S, t, s, lim;
	int odd[17], sz;

	sz = 0;
	for (i = 0; i < V; i++) {
		if (hi[i] & 1) odd[sz++] = i;
	}

	for (k = 0; k < V; k++) {
		for (i = 0; i < V; i++) if (dist[i][k] < INF) {
			for (j = 0; j < V; ++j) if (dist[k][j] < INF) {
				t = dist[i][k] + dist[k][j];
				if (t < dist[i][j]) dist[i][j] = t;
			}
		}
	}

	memset(dp, INF, sizeof(dp));
	dp[0] = 0, lim = (1<<sz)-1;
	for (S = 0; S <= lim; S++) {
		for (i = 0; i < sz; i++) if ((~S >> i) & 1) {
			for (j = 0; j < i; j++) if ((~S >> j) & 1) {
				t = dp[S] + dist[odd[i]][odd[j]];
				s = S | (1<<i) | (1<<j);
				if (t < dp[s]) dp[s] = t;
			}
		}
	}
	return sum + dp[lim];
}
 
int h[1402][1402];
char map[1402][1402];
 
int main()
{
	int k, E, s, t, a, sum;
 
	memset(dist, INF, sizeof(dist));
	V = in(), E = in();
	sum = 0;
	while (E--) {
		s = in(), t = in(), a = in();
		sum += a;
		if (a < dist[s][t]) dist[s][t] = dist[t][s] = a;
		k = hi[s]++, to[s][k] = t, d[s][k] = a;
		k = hi[t]++, to[t][k] = s, d[t][k] = a;
	}
	printf("%d\n", chinesePostman(sum));
	return 0;
}

