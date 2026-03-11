// AOJ 2249: Road Construction
// 2017.11.15 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 16010
typedef struct { int t, node, c; } QUE;
QUE que[MAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && (que[l].t < que[i].t ||
		que[l].t == que[i].t && que[l].c < que[i].c)) min = l; else min = i;
	if (r < qsize && (que[r].t < que[min].t ||
		que[r].t == que[min].t && que[r].c < que[min].c)) min = r;
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

void enq(int n, int t, int c)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].node = n, que[i].t = t, que[i].c = c;
	while (i > 0 && (que[min = PARENT(i)].t > que[i].t ||
		   que[min].t == que[i].t && que[min].c > que[i].c)) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define INF 0x55555555
typedef struct { int to, d, c; } EG;
typedef struct { int len; EG e[100]; } TBL;
TBL *tbl;
char visited[10003]; int size;

int dijkstra(int start)
{
	int s, d, c, ans = 0;
	EG *e;

	qsize = 0;
	memset(visited, 0, sizeof(visited));
	enq(start, 0, 0);
	while(qsize) {
		s = que[0].node, d = que[0].t, c = que[0].c; deq();
		if (visited[s]) continue;
		visited[s] = 1;
		ans += c;
		for (e = tbl[s].e; e->to >= 0; e++) {
			if (visited[e->to]) continue;
			enq(e->to, d + e->d, e->c);
		}
	}
	return ans;
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
	int n, m, i, u, v, d, c;
	EG  *e;

	while (fgets(p=buf, 40, stdin) && *p != '0') {
		size = n = getint(), p++, m = getint();

		tbl = calloc(size+2, sizeof(TBL));
		memset(tbl, 0, sizeof(TBL)*size);
		for (i = 0; i < m; i++) {
			fgets(p=buf, 40, stdin);
			u = getint()-1, p++, v = getint()-1, p++, d = getint(), p++, c = getint();

			e = tbl[u].e + tbl[u].len;
			e->to = v, e->d = d, e->c = c, tbl[u].len++;

			e = tbl[v].e + tbl[v].len;
			e->to = u, e->d = d, e->c = c, tbl[v].len++;
		}
		for (i = 0; i < n; i++) tbl[i].e[tbl[i].len].to = -1;

		printf("%d\n", dijkstra(0));
		free(tbl);
	}
	return 0;
}