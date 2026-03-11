// AOJ 3030 Light
// 2018.4.14 bal4u
 
#include <stdio.h>
//#include <stdlib.h>
#include <string.h>
 
#define QMAX 100000
typedef struct { int t, s, c; } QUE;
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

void enq(int s, int t, int c)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].s = s, que[i].t = t, que[i].c = c;
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
int x[102], y[102];
int dist[102][1002];

int dijkstra(int n)
{
	int s, e, t, nt, c;

	memset(dist, INF, sizeof(dist));
	qsize = 0;
	for (s = 0; s < n; s++) c = x[s]+y[s], enq(s, c, c), dist[s][c] = c;
	while (qsize) {
		s = que[0].s, t = que[0].t, c = que[0].c, deq();
//printf("s %d, t %d, c %d, N %d, qsize %d\n", s, t, c, n, qsize);
		if (s == n) break; 
		if (dist[s][c] < t) continue;
		for (e = 0; e < n; e++) if (e != s) {
			nt = ABS(x[e]-x[s])+ABS(y[e]-y[s])-c-1;
			if (nt < 0) nt = 0;
			if (dist[e][nt] > t+nt) dist[e][nt] = t+nt, enq(e, t+nt, nt);
		}
		nt = ABS(x[n]-x[s])+ABS(y[n]-y[s])-c;
		if (nt < 0) nt = 0;
		if (dist[n][nt] > t+nt) dist[n][nt] = t+nt, enq(n, t+nt, nt);
	}
	return t;
}

int main()
{
	int W, H, N, i;

	W = in(), H = in(), N = in();
	for (i = 0; i < N; i++) x[i] = in()-1, y[i] = in()-1;
	x[N] = W-1, y[N] = H-1;
	printf("%d\n", dijkstra(N));
	return 0;
}
