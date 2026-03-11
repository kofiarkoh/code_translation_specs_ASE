// AOJ 1553: Manhattan Warp Machine 1
// 2019.2.22 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MAX 1300000
typedef struct { int x, c; } QUE;
QUE que[MAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].c < que[i].c) min = l; else min = i;
	if (r < qsize && que[r].c < que[min].c) min = r;
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

void enq(int x, int c)
{
	int i, min;
	QUE qt;

	i = qsize++, que[i].x = x, que[i].c = c;
	while (i > 0 && que[min = PARENT(i)].c > que[i].c) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

int N;
int D[20], C[20];
char visited[200005];

int dijkstra(int start, int goal)
{
	int i, x, y, c;
	int lim = goal << 1;

	qsize = 0;
	enq(start, 0);
	while (qsize) {
		x = que[0].x, c = que[0].c; deq();
		if (x == goal) return c;
		if (visited[x]) continue;
		visited[x] = 1;
		for (i = 0; i < N; i++) {
			y = x + D[i];
			if (y < 200005 && visited[y] == 0) enq(y, c+C[i]);
			y = x - D[i];
			if (y > 0 && visited[y] == 0) enq(y, c+C[i]);
		}
	}
	return -1;
}

typedef struct { int d, c; } T;
T tbl[20];

int cmp(const void *a, const void *b)
{
	return ((T *)a)->c - ((T *)b)->c;
}

int main()
{
	int i, X;

	scanf("%d%d", &N, &X);
	for (i = 0; i < N; i++) scanf("%d%d", &tbl[i].d, &tbl[i].c);
	qsort(tbl, N, sizeof(T), cmp);
	for (i = 0; i < N; i++) D[i] = tbl[i].d, C[i] = tbl[i].c;
	printf("%d\n", dijkstra(0, X));
	return 0;
}

