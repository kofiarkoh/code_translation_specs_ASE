// AOJ 2151: Brave Princess Revisited
// 2017.11.11 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef struct { int t, n, m; } QUE;
QUE que[20001]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].t < que[i].t) min = l; else min = i;
	if (r < qsize && que[r].t < que[min].t) min = r;
	if (min != i) {
		QUE t = que[i]; que[i] = que[min]; que[min] = t;
		min_heapify(min);
	}
}

int deq(int *n, int *m)
{
	if (qsize == 0) return 0;
	*n = que[0].n, *m = que[0].m;
	que[0] = que[--qsize];
	min_heapify(0);
	return 1;
}

void enq(int t, int n, int m)
{
	int i, min;

	i = qsize++;
	que[i].t = t, que[i].n = n, que[i].m = m;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		QUE tt = que[i]; que[i] = que[min]; que[min] = tt;
		i = min;
	}
}

typedef struct { int len, to[101], d[101], e[101]; } TBL;
TBL tbl[101];
int node[101][101]; int size;

int search(int start, int goal, int l)
{
	int s, m, i, k, e, p, ans;
	TBL *tp;

	qsize = 0;
	memset(node, 0x55, sizeof(node));
	node[start][l] = 0; enq(0, start, l);
	ans = 0x7ffffff;
	while(deq(&s, &m)) {
		p = node[s][m];
		if (s == goal) {
			if (p < ans) ans = p;
			continue;
		}

		for (tp = tbl + s, i = 0; i < tp->len; i++) {
			e = tp->to[i];

			k = p + tp->e[i];
			if (k < node[e][m]) node[e][m] = k, enq(node[e][m], e, m);

			k = m - tp->d[i];
			if (tp->d[i] <= m && p < node[e][k]) node[e][k] = p, enq(node[e][k], e, k);
		}
	}
	return ans;
}

int main()
{
	int n, m, l, a, b, d, e, i;

	while (scanf("%d%d%d", &n, &m, &l) && n > 0) {
		memset(tbl, 0, sizeof(tbl));
		while (m--) {
			scanf("%d%d%d%d", &a, &b, &d, &e);
			a--, b--;
			i = tbl[a].len, tbl[a].to[i] = b, tbl[a].d[i] = d, tbl[a].e[i] = e, tbl[a].len++;
			i = tbl[b].len, tbl[b].to[i] = a, tbl[b].d[i] = d, tbl[b].e[i] = e, tbl[b].len++;
		}
		size = n;
		printf("%d\n", search(0, n-1, l));
	}
	return 0;
}