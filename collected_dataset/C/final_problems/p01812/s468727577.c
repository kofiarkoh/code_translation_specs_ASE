// AOJ 2709 Dark Room
// 2018.3.31 bal4u

#include <stdio.h>
#include <string.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

#define INF 0x10101010
#define QMAX 100000
int q[QMAX]; int top, end;
int d[18], r[102];
int v[102][102];
int dist[1<<16];

int main()
{
	int N, M, K, i, j, k, s, e, t;

	N = in(), M = in(), K = in();
	memset(r, -1, sizeof(r));
	for (i = 0; i < M; i++) d[i] = in()-1, r[d[i]] = 1<<i;
	for (i = 0; i < N; i++) for (j = 0; j < K; j++) v[i][j] = in()-1;

	memset(dist, INF, sizeof(dist));
	q[0] = s =(1<<M)-1, dist[s] = 0;
	top = 0, end = 1;
	while (top != end) {
		s = q[top++];

		if ((t=dist[s]) == INF) continue;
		for (i = 0; i < K; i++) {
			e = 0;
			for (j = 0; j < M; j++) if ((s>>j)&1) {
				k = v[d[j]][i];
				if (r[k] >= 0) e |= r[k];
			}
			if (dist[e] > t+1) dist[e] = t+1, q[end++] = e;
		}
	}
	printf("%d\n", dist[0]);
	return 0;
}
