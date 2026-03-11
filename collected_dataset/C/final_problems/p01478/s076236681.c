// AOJ 2361: Sort
// 2017.12.18 bal4u@uu

#include <stdio.h>
#include <string.h>

#define QSIZE 4000000

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

typedef struct { char s[8]; int t; } QUE;
QUE que[QSIZE+5];
int qsize;

int N, N1, N2, N3;

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

#define HSIZ 100019
typedef struct { char s[8]; int t; } HASH;
HASH hash[HSIZ+2], *hashend = hash + HSIZ;

int lookup(char *s, int t)
{
	HASH *tp;

	tp = hash + ((s[0] << N3) + (s[N>>1] << N2) + (s[N-1])*101 + N1) % HSIZ;

	while (tp->t) {
		if (!memcmp(tp->s, s, N)) {
			if (tp->t > t) { tp->t = t; return 1; }
			return 0;
		}
		if (++tp == hashend) tp = hash;
	}
	memcpy(tp->s, s, N), tp->t = t;
	return 1;
}

int c[8][8];
char start[8] = {0,1,2,3,4,5,6,7};

int main()
{
	int i, j, t, cost, ans;
	char s[10], a;

	scanf("%d", &N); N3 = N << 3, N2 = (N>>1) << 3, N1 = N*103;
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

	ans = 0;
	for (i = 0; i < HSIZ; i++) if (hash[i].t > ans) ans = hash[i].t;
	printf("%d\n", ans);
	return 0;
}