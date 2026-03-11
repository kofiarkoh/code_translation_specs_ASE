// AOJ 1219: Pump up Batteries
// 2017.10.23 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 102
typedef struct { int t, id; } QUE;
QUE que[MAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && (que[l].t < que[i].t ||
		que[l].t == que[i].t && que[l].id < que[i].id)) min = l; else min = i;
	if (r < qsize && (que[r].t < que[min].t ||
		que[r].t == que[min].t && que[r].id < que[min].id)) min = r;
	if (min != i) {
		QUE t = que[i]; que[i] = que[min]; que[min] = t;
		min_heapify(min);
	}
}

int deq(void)
{
	if (qsize == 0) return 0;
//	*t = que[0].t, *id = que[0].id;
	que[0] = que[--qsize];
	min_heapify(0);
	return 1;
}

void enq(int t, int id)
{
	int i, min;

	i = qsize++;
	que[i].t = t, que[i].id = id;
	while (i > 0 && (que[min = PARENT(i)].t > que[i].t ||
		que[min].t == que[i].t && que[min].id > que[i].id)) {
		QUE tt = que[i]; que[i] = que[min]; que[min] = tt;
		i = min;
	}
}

int mque[MAX], top, end, mqsz;
typedef struct { int r, i, n, u[51], c[51]; } T;    // r:remain time
T tbl[MAX], *tp;

int main() 
{
	int n, lim, id, id0, i, t, ans;

	while (scanf("%d%d", &n, &lim) && n > 0) {
		qsize = 0;
		for (tp = tbl, id = 0; id < n; id++, tp++) {
			for (i = 0; scanf("%d", &t) && t > 0; i++) tp->u[i] = t, scanf("%d", &tp->c[i]);
			tp->i = 0, tp->n = i, enq(tp->u[0], id);
		}

		ans = 0, top = end = 0, mqsz = 0;
		for (t = 1; t <= lim; t++) {
			id0 = -1;
			if (mqsz > 0) id0 = mque[top], ans += mqsz - 1;
			while (qsize > 0 && que[0].t == t) {
				id = que[0].id, deq();
				tp = tbl + id, tp->r = tp->c[tp->i];
				mqsz++, mque[end] = id; if (++end == MAX) end = 0;
			}
			if (mqsz == 0) continue;
			if ((id = mque[top]) != id0) continue;
			tp = tbl + id;
			if (--tp->r == 0) {
				if (++tp->i == tp->n) tp->i = 0;
				enq(t + tp->u[tp->i], id);
				mqsz--; if (++top == MAX) top = 0;
			}
		}
		printf("%d\n", ans);
	}
	return 0; 
}