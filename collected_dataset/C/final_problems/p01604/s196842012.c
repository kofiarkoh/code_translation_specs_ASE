// AOJ 2492: goto busters
// 2018.1.8 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HASHSIZ 199
typedef struct { char *s; int n; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;			// Hash table

int lookup(char *s, int w)
{
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->n;
		if (++tp == hashend) tp = hash;
	}
	return -1;
}

void insert(char *s, int w, int n)
{
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->n = n;
}

#define QMAX 5000
typedef struct { int t, s; } QUE;
QUE que[QMAX]; int qsize;

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

char text[101][20], f[101];
char to[101][101], d[101][101], hi[101];
char used[101];

int dijkstra(int start, int goal)
{
	int i, s, t, e;

	t = 0, qsize = 0;
	enq(start, 0);
	while (qsize) {
		s = que[0].s, t = que[0].t, deq();
		if (s == goal) break;
		if (used[s]) continue;
		used[s] = 1;
		for (i = 0; i < hi[s]; i++) {
			e = to[s][i];
			if (used[e]) continue;
			enq(e, t+d[s][i]);
		}
	}
	return t;
}

int main()
{
	int n, i, k;
	char *p, *q;

	fgets(p=text[0], 10, stdin);
	n = atoi(p);
	for (i = 0; i < n; i++) {
		fgets(p=text[i], 20, stdin);
		if (memcmp(p, "goto ", 5) == 0) { f[i] = 1; continue; }
		q = p; while (*p != ':') p++;
		*p = 0, insert(q, p-q, i);
	}
	for (i = 0; i < n; i++) {
		if (!f[i]) to[i][hi[i]++] = i+1;
		else {
			p = q = text[i] + 5;
			while (*p != ';') p++;
			*p = 0, k = lookup(q, p-q);
			to[i][hi[i]++] = k;
			if (k != i+1) to[i][hi[i]] = i+1, d[i][hi[i]++] = 1;
		}
	}
	if (!f[n-1]) to[n-1][hi[n-1]++] = n;

	printf("%d\n", dijkstra(0, n));
	return 0;
}
