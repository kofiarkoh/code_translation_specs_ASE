// AOJ 2040 Sort the Panels
// 2018.3.13 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define QMAX 300000
typedef struct { int s; char p; char t; } QUE;
QUE que[QMAX]; int qsize, qmax;

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

void enq(int s, int t, char p)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].s = s, que[i].t = t, que[i].p = p;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define ABS(a)  ((a)>=0?(a):-(a))
char dist[1<<16][16];

int dijkstra(int n, int nb, int start, int goal)
{
	int i, j, k, m, s, t, p, ns, nt, nw;
	int pi, ij;

	memset(dist, 75, sizeof(dist));
	qsize = 0;
	t = -1, nw = n - nb;
	for (p = 0; p < n; p++) dist[start][p] = 0, enq(start, 0, p);
	while (qsize) {
		s = que[0].s, t = que[0].t, p = que[0].p, deq();
		if (s == goal) break;

		if (dist[s][p] < t) continue;

		for (k = i = 0; k < nb; i++) if (s & (1<<i)) { k++, pi = ABS(p-i);
			for (m = j = 0; m < nw; j++) if (!(s & (1<<j))) { m++;
				ij = ABS(j-i);
				ns = s ^ (1<<i) ^ (1<<j);
				nt = t + pi+ij;
				if (dist[ns][j] > nt) dist[ns][j] = nt, enq(ns, nt, j);
				nt = t + ABS(p-j)+ij;
				if (dist[ns][i] > nt) dist[ns][i] = nt, enq(ns, nt, i);
			}
		}
	}
	return t;
}

int calc(char *s) { int res=0; while (*s > ' ') res = (res<<1)|(*s++ == 'B'); return res; }

int main()
{
	int n, nb, start, goal;
	char buf[20], *p;

	while (fgets(buf, 10, stdin) && *buf != '0') {
		n = atoi(buf);
		fgets(p=buf, 20, stdin); nb = 0; while (*p > ' ') if (*p++ == 'B') nb++;
		start = calc(buf);
		fgets(buf, 20, stdin), goal = calc(buf);
		if (nb > n-nb) {
			nb = n-nb;
			start = ~start & ((1<<n)-1), goal = ~goal & ((1<<n)-1);
		}
		printf("%d\n", dijkstra(n, nb, start, goal));
	}
	return 0;
}
