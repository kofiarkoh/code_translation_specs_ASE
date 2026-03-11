// AOJ 1182 Railway Connection
// 2018.2.22 bal4u
 
#include <stdio.h>
#include <string.h>

#define QMAX 10000
typedef struct { int t; int s; } QUE;
QUE que[QMAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].t < que[i].t) min = l; else min = i;
	if (r < qsize && que[r].t < que[min].t) min = r;
	if (min != i) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		min_heapify(min);
	}
}

void deq()
{
	que[0] = que[--qsize];
	min_heapify(0);
}

void enq(int s, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].s = s, que[i].t = t;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define INF 0x10101010
#define MAX 20001

int d[22][102][102];
int p[22], q[22][52], r[22][52];
int hi[102], to[102][204], w[102][204];
int dist[102], fare[22][20010];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	while (c <= ' ') c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int dijkstra(int s, int g)
{
	int i, e, d;

	memset(dist, INF, sizeof(dist));
	qsize = 0;
	enq(s, 0), dist[s] = 0;
	while (qsize) {
		s = que[0].s, deq();
		if (s == g) return dist[g];
		for (i = 0; i < hi[s]; i++) {
			e = to[s][i], d = dist[s] + w[s][i];
			if (d < dist[e]) dist[e] = d, enq(e, d);
		}
	}
	return -1;
}

#if 0
int fare(int c, int dist)
{
	int i, pre, ans;

	ans = 0, pre = 0;
	for(i = 0; q[c][i] <= dist; i++) {
		ans += r[c][i] * (q[c][i] - pre);
		pre = q[c][i];
	}
	return ans + r[c][i] * (dist - pre);
}
#endif

int main()
{
	int N, M, C, S, G, i, j, k;
	int x, y, c, t;

	while (N = in()) {
		M = in(), C = in(), S = in()-1, G = in()-1;

		memset(d, INF, sizeof(d));
		for (c = 0; c < C; c++) for (i = 0; i < N; i++) d[c][i][i] = 0;

		for (i = 0; i < M; i++) {
			x = in()-1, y = in()-1, t = in(), c = in()-1;
			if (t < d[c][x][y]) d[c][x][y] = d[c][y][x] = t;
		}

		for (c = 0; c < C; c++) {
			for (k = 0; k < N; k++) for (i = 0; i < N; i++) for (j = 0; j < N; j++) {
				t = d[c][i][k] + d[c][k][j];
				if (t < d[c][i][j]) d[c][i][j] = t;
			}
		}

		for (c = 0; c < C; c++) p[c] = in();
		for (c = 0; c < C; c++) {
			for (i = 1; i < p[c]; i++) q[c][i] = in();
			q[c][p[c]] = MAX-1;
			for (i = 1; i <= p[c]; i++) r[c][i] = in();
		}

		if (!M) { puts(S == G? "0": "-1"); continue; }

		for (c = 0; c < C; c++) {
			for (i = 1; i <= p[c]; i++) for (j = q[c][i-1]+1; j <= q[c][i]; j++)
				fare[c][j] = fare[c][j-1] + r[c][i];
		}

		memset(hi, 0, sizeof(hi));
		for (int i = 0; i < N; i++) for (int j = 0; j < N; j++) {
			t = INF;
			for (c = 0; c < C; c++) if (d[c][i][j] < MAX) {
				if (t > (k=fare[c][d[c][i][j]])) t = k;
			}

			if (t < INF) {
				k = hi[i]++, to[i][k] = j, w[i][k] = t;
				k = hi[j]++, to[j][k] = i, w[j][k] = t;
			}
		}
		printf("%d\n", dijkstra(S, G));
	}
	return 0;
}

