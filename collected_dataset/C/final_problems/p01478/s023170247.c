// AOJ 2361: Sort
// 2017.12.18 bal4u@uu

#include <stdio.h>
#include <string.h>

#define QSIZE 2000000

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

typedef struct { char s[8]; int t; } QUE;
QUE que[QSIZE+5];
int qsize;
int N;

void max_heapify(int i)
{
	int l, r, max;
	QUE qt;
	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].t > que[i].t) max = l; else max = i;
	if (r < qsize && que[r].t > que[max].t) max = r;
	if (max != i) {
		qt = que[i], que[i] = que[max], que[max] = qt;
		max_heapify(max);
	}
}

void deq()
{
//	if (qsize == 0) return 0;
//	*t = que[0].t;
	que[0] = que[--qsize];
	max_heapify(0);
//	return 1;
}

void enq(int t, char *s)
{
	int i, max;
	QUE qt;

	i = qsize++;
	que[i].t = t, memcpy(que[i].s, s, N);
	while (i > 0 && que[max = PARENT(i)].t < que[i].t) {
		qt = que[i]; que[i] = que[max], que[max] = qt;
		i = max;
	}
}

int c[8][8];
char start[9] = {0,1,2,3,4,5,6,7};
int min[16434830];
int max[9] = {0,1,8,136,1672,18056,181896,1754760,16434824};
int lookup(char *s, int t)
{
	int i, k;

	k = 0, i = N; while (i--) k = (k<<3) | *s++;
	if (!min[k] || min[k] > t) { min[k] = t; return 1; }
	return 0;
}

int main()
{
	int i, j, t, cost, ans;
	char s[10], a;

	scanf("%d", &N);
	for (i = 0; i < N; i++) for (j = 0; j < N; j++) scanf("%d", &c[i][j]);

	ans = 0;
	qsize = 0, enq(0, start);
	while (qsize) {
		t = que[0].t, memcpy(s, que[0].s, N), deq();

		for (i = 0; i < N; i++) for (j = i+1; j < N; j++) {
			a = s[i], s[i] = s[j], s[j] = a;
			cost = t + c[s[i]][s[j]];
			if (lookup(s, cost)) enq(cost, s);
			a = s[i], s[i] = s[j], s[j] = a;
		}
	}

	ans = 0, i = max[N] + 1;
	while (i--) if (min[i] > ans) ans = min[i];
	printf("%d\n", ans);
	return 0;
}