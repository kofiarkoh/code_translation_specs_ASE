// AOJ 2709 Dark Room
// 2018.3.31 bal4u

#include <stdio.h>
#include <string.h>

#define QMAX 40000
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

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

#define INF 0x10101010
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
	qsize = 0;
	enq(s=(1<<M)-1, 0), dist[s] = 0;
	while (qsize) {
		s = que[0].s, t = que[0].t, deq();
	
		if (dist[s] == INF) continue;
		for (i = 0; i < K; i++) {
			e = 0;
			for (j = 0; j < M; j++) if ((s>>j)&1) {
				k = v[d[j]][i];
				if (r[k] >= 0) e |= r[k];
			}
			if (dist[e] > t+1) dist[e] = t+1, enq(e, t+1);
		}
	}
	printf("%d\n", dist[0]);
	return 0;
}
