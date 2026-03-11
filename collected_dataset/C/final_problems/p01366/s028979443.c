// AOJ 2249: Road Construction
// 2017.11.15 bal4u@uu
// 2017.12.9

#include <stdio.h>
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
		qt = que[i]; que[i] = que[min]; que[min] = qt;
		min_heapify(min);
	}
}

void deq()
{
	que[0] = que[--qsize];
	min_heapify(0);
}

void enq(int n, int t, int c)
{
	int i, min;
	QUE qt;
	i = qsize++;
	que[i].node = n, que[i].t = t, que[i].c = c;
	while (i > 0 && (que[min = PARENT(i)].t > que[i].t ||
		   que[min].t == que[i].t && que[min].c > que[i].c)) {
		qt = que[i]; que[i] = que[min]; que[min] = qt;
		i = min;
	}
}

typedef struct { int to[100], d[100], c[100]; } TBL;
TBL tbl[10003];
int len[10003];
char visited[10003];

int dijkstra(int start)
{
	int i, s, e, c, d, ans = 0;
	TBL *tp;

	qsize = 0;
	memset(visited, 0, sizeof(visited));
	enq(start, 0, 0);
	while(qsize) {
		s = que[0].node, d = que[0].t, c = que[0].c, deq();
		if (visited[s]) continue;
		visited[s] = 1;
		ans += c;
		for (tp = tbl+s, i = 0; i < len[s]; i++) {
			e = tp->to[i];
			if (visited[e]) continue;
			enq(e, d + tp->d[i], tp->c[i]);
		}
	}
	return ans;
}

char buf[40], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++;
	return n;
}

int main()
{
	int n, m, i, k, u, v, d, c;
	TBL *tp;

	while (fgets(p=buf, 40, stdin) && *p != '0') {
		n = getint(), m = getint();
		memset(len, 0, sizeof(len));
		while (m--) {
			fgets(p=buf, 40, stdin);
			u = getint()-1, v = getint()-1, d = getint(), c = getint();
			tp = tbl+u,	k = len[u]++;
			tp->to[k] = v, tp->d[k] = d, tp->c[k] = c;
			tp = tbl+v, k = len[v]++;
			tp->to[k] = u, tp->d[k] = d, tp->c[k] = c;
		}
		printf("%d\n", dijkstra(0));
	}
	return 0;
}