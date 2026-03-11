// AOJ 2361: Sort
// 2018.4.10 bal4u

#include <stdio.h>
#include <string.h>

#define QSIZE 600000

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

typedef struct { int s, t; } QUE;
QUE que[QSIZE+2]; int qsize;

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

#define INF 0x30303030
int c[8][8]; int n;
int three[10][10];
int dist[1<<24];

int main()
{
	int i, j, s, ss, ns, t, nt, a, b, bi, bj, ans;

	for (n = 1; n <= 8; n++) for (i = 0; i < n; i++) three[n][i] = 3*(n-i-1);

	n = in();
	for (i = 0; i < n; i++) for (j = 0; j < n; j++) c[i][j] = in();

	memset(dist, INF, sizeof(dist));
	s = 0; for (i = 0; i < n; i++) s = (s << 3) | i;
	ans = 0, qsize = 0, enq(s, 0);
	while (qsize) {
		s = que[0].s, t = que[0].t, deq();
		if (dist[s] <= t) continue;
		dist[s] = t;
		if (t > ans) ans = t;
		for (i = 0; i < n; i++) {
			bi = three[n][i], a = (s >> bi) & 7;
			ss = s ^ (a << bi);
			for (j = i+1; j < n; j++) {
				bj = three[n][j], b = ((s >> bj) & 7);
				ns = (ss ^ (b << bj)) | (b << bi) | (a << bj);
				nt = t + c[i][j];
				if (dist[ns] <= nt) continue;
				enq(ns, nt);
			}
		}
	}
	printf("%d\n", ans);
	return 0;
}

