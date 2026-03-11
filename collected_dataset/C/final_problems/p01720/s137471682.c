// AOJ 2608: Minus One
// 2017.11.30 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 300000
typedef struct { int t, node; } QUE;
QUE que[MAX]; int qsize;

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
//	if (qsize == 0) return 0;
//	*n = que[0].node, *t = que[0].t, *c = que[0].c;
	que[0] = que[--qsize];
	min_heapify(0);
//	return 1;
}

void enq(int n, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].node = n, que[i].t = t;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

typedef struct { int *to; } TBL;
TBL tbl[100001]; int len[100001];
int x[300000], y[300000];
int smin[100001], tmin[100001];
int scnt[100001], tcnt[100001];

void dijkstra(int *node, int start)
{
	int i, s, e, d;

	qsize = 0;
	node[start] = 0;
	enq(start, 0);
	while (qsize) {
		s = que[0].node; deq();
		for (i = 0; i < len[s]; i++) {
			e = tbl[s].to[i];
			d = node[s] + 1;
			if (d < node[e]) node[e] = d, enq(e, d);
		}
	}
}

char buf[40], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
	int n, m, s, t, i;
	long long ans;

	fgets(p=buf, 40, stdin);
	n = getint(), p++, m = getint(), p++, s = getint()-1, p++, t = getint()-1;
	for (i = 0; i < m; i++) {
		fgets(p=buf, 20, stdin);
		x[i] = getint()-1, p++, y[i] = getint()-1;
		len[x[i]]++, len[y[i]]++;
	}
	for (i = 0; i < n; i++) tbl[i].to = calloc(len[i], sizeof(int));
	memset(len, 0, sizeof(len));
	for (i = 0; i < m; i++) {
		tbl[x[i]].to[len[x[i]]++] = y[i];
		tbl[y[i]].to[len[y[i]]++] = x[i];
	}

	memset(smin, 0x33, sizeof(smin));
	dijkstra(smin, s);
	memset(tmin, 0x33, sizeof(tmin));
	dijkstra(tmin, t);

	for (i = 0; i < n; i++) {
		if (smin[i] < n) scnt[smin[i]]++;
		if (tmin[i] < n) tcnt[tmin[i]]++;
	}

	ans = 0, s = smin[t]-1;
	for (i = 0; i < s; i++) ans += (long long)scnt[i] * tcnt[s-1-i];
	printf("%lld\n", ans);
	return 0;
}