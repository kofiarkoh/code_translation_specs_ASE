// AOJ 2021: Princess in Danger
// 2017.12.11 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 20000
typedef struct { int t, node, s; } QUE;
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

void enq(int n, int t, int s)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].node = n, que[i].t = t, que[i].s = s;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define INF 0x7fffff

typedef struct { int to[101], t[101]; } TBL;
TBL tbl[101];
int len[101];
char f[101];
int dist[101][101];

void dijkstra(int start, int m)
{
	int i, j, s, t, k, s2, t2, x;

	memset(dist, 0x33, sizeof(dist));
	qsize = 0;
	enq(start, 0, m);
	while (qsize) {
		s = que[0].node, t = que[0].t, k = que[0].s; deq();
		if (dist[s][k] < t) continue;
		for (i = 0; i < len[s]; i++) {
			s2 = tbl[s].to[i], t2 = tbl[s].t[i];
			if (f[s]) {
				for (j = 0; j <= m; j++) {
					if (k+j > m) break;
					if (k+j < t2) continue;
					if ((x = t+j+t2) < dist[s2][k+j-t2]) {
						dist[s2][k+j-t2] = x;
						enq(s2, x, k+j-t2);
					}
				}
			} else {
				if (k < t2) continue;
				if ((x = t+t2) < dist[s2][k-t2]) {
					dist[s2][k-t2] = x;
					enq(s2, x, k-t2);
				}
			}
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
	int n, m, l, k, a, h, i, x, y, t, ans;

	while (fgets(p=buf, 60, stdin) && *p != '0') {
		n = getint(), m = getint(), l = getint();
		k = getint(), a = getint(), h = getint();

		memset(f, 0, sizeof(f));
		if (l > 0) {
			fgets(p=buf, sizeof(buf), stdin);
			while (l--) f[getint()] = 1;
		}

		memset(len, 0, sizeof(len));
		while (k--) {
			fgets(p=buf, 20, stdin);
			x = getint(), y = getint(), t = getint();
			i = len[x]++, tbl[x].to[i] = y, tbl[x].t[i] = t;
			i = len[y]++, tbl[y].to[i] = x, tbl[y].t[i] = t;
		}

		dijkstra(a, m);
		ans = INF;
		for (i = 0; i <= m; i++) if (dist[h][i] < ans) ans = dist[h][i];
		if (ans == INF) puts("Help!");
		else printf("%d\n", ans);
	}
	return 0;
}