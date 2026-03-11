// AOJ 3030 Light
// 2018.4.4 bal4u
 
#include <stdio.h>
//#include <stdlib.h>
#include <string.h>
 
#define QMAX 150000
typedef struct { int t, s; } QUE;
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
#define ABS(a)   ((a)>=0?(a):-(a))
int x[102], y[102]; int N;
int dist[102];

int dijkstra(int sr, int sc, int gr, int gc)
{
	int s, e, r, nr;

	memset(dist, INF, (N+1) << 2);
	qsize = 0;
	for (s = 0; s <= N; s++) enq(s, x[s]+y[s]);
	while (qsize) {
		s = que[0].s, r = que[0].t, deq();
		if (s == N) break;
		if (dist[s] < r) continue;
		dist[s] = r;
		for (e = 0; e <= N; e++) if (e != s) {
			nr = ABS(x[e]-x[s])+ABS(y[e]-y[s])-r;
			if (nr < 0) nr = 0;
			if (dist[e] < r+nr) continue;
			enq(e, r+nr);
		}
	}
	return r;
}

int main()
{
	int W, H, i;

	W = in(), H = in(), N = in();
	for (i = 0; i < N; i++) x[i] = in()-1, y[i] = in()-1;
	x[N] = W-1, y[N] = H-1;
	printf("%d\n", dijkstra(0, 0, H-1, W-1));
	return 0;
}
