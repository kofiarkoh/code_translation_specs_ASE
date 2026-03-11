// AOJ 2021: Princess in Danger
// 2017.12.11 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 10000
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
	que[0] = que[--qsize];
	min_heapify(0);
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

#define INF 0x07070707

typedef struct { int to[70], t[70]; } TBL;
TBL tbl[10200];
int len[10200];
char f[101];
int dist[10200];

void dijkstra(int start)
{
	int i, s, t, e, d;

	memset(dist, INF, sizeof(dist));
	qsize = 0;
	dist[start] = 0;
	enq(start, 0);
	while (qsize) {
		s = que[0].node, t = que[0].t; deq();
		if (dist[s] < t) continue;
		for (i = 0; i < len[s]; i++) {
			e = tbl[s].to[i], d = dist[s] + tbl[s].t[i];
			if (d < dist[e]) dist[e] = d, enq(e, d);
		}
	}
}

char buf[400], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf);
	p++;
	return n;
}


int main()
{
	int n, m, l, k, a, h, i, j, ans;
	int x, y, t, u, v, u2, v2, m1;

	while (fgets(p=buf, 60, stdin) && *p != '0') {
		n = getint(), m = getint(), l = getint();
		k = getint(), a = getint(), h = getint();
		m1 = m+1;

		memset(f, 0, sizeof(f));
		f[a] = f[h] = 1;
		if (l > 0) {
			fgets(p=buf, sizeof(buf), stdin);
			while (l--) f[getint()] = 1;
		}

		memset(len, 0, sizeof(len));
		while (k--) {
			fgets(p=buf, 20, stdin);
			x = getint()*m1, y = getint()*m1, t = getint();
			for (j = t; j <= m; j++) {
				u  = x + j, v  = y + j;
				u2 = u - t, v2 = v - t;
				i = len[u]++, tbl[u].to[i] = v2, tbl[u].t[i] = t;
				i = len[v]++, tbl[v].to[i] = u2, tbl[v].t[i] = t;
			}
		}
		for (u = 0, i = 0; i < n; i++, u += m1) {
			if (!f[i]) continue;
			for (j = 0; j < m; j++) {
				v = u+j, v2 = v+1;
				t = len[v]++, tbl[v].to[t] = v2, tbl[v].t[t] = 1;
			}
		}

		dijkstra(a*m1+m);

		ans = INF; m1 *= h;
		for (i = 0; i <= m; i++) if (dist[m1+i] < ans) ans = dist[m1+i];
		if (ans == INF) puts("Help!");
		else printf("%d\n", ans);
	}
	return 0;
}