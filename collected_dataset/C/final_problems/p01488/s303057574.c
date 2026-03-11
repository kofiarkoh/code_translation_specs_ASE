// AOJ 2371 TransferTrain
// 2018.2.24 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// hash function library
#define HASHSIZ 249989
typedef struct { char *s; int id; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;

int insert(char *s, int id)
{
	unsigned long long i = 0;
	char *p = s;
	HASH *tp;

	while (*p) i = (i<<6) + *p++;
	tp = hash + (int)(i % HASHSIZ);
	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->id;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->id = id;
	return -1;
}

// Priority Queue library
#define QMAX 100000
typedef struct { int s, tm; short tr; } QUE;
QUE que[QMAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && (que[l].tm < que[i].tm ||
		que[l].tm == que[i].tm && que[l].tr < que[i].tr)) min = l; else min = i;
	if (r < qsize && (que[r].tm < que[min].tm ||
		que[r].tm == que[min].tm && que[r].tr < que[min].tr)) min = r;
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

int qmax;

void enq(int s, int tm, int tr)
{
	int i, min;
	QUE qt;

	i = qsize++;		if (qsize > qmax) qmax = qsize;
	que[i].s = s, que[i].tm = tm, que[i].tr = tr;
	while (i > 0 && (que[min = PARENT(i)].tm > que[i].tm ||
		que[min].tm == que[i].tm && que[min].tr > que[i].tr)) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define MAX 150002
char name[MAX][11]; int sz;
int  trSta[MAX];
int lim[MAX], hi[MAX]; int *to[MAX]; short *tm[MAX]; char *tr[MAX];
char mk[MAX];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

void ins(char *s)
{
	while ((*s = getchar_unlocked()) > ' ') s++;
	*s = 0;
}

void getmem(int k, int lim, int mode)
{
	if (mode) {			// realloc
		to[k] = realloc(to[k], lim << 2);	// int
		tm[k] = realloc(tm[k], lim << 1);	// short
		tr[k] = realloc(tr[k], lim);		// char
	} else {
		to[k] = malloc(lim << 2);
		tm[k] = malloc(lim << 1);
		tr[k] = malloc(lim);
		hi[k] = 0;
	}
}

int dijkstra(int *atm, int *atr, int start, int goal)
{
	int i, s, _tm, _tr, e;

	qsize = 0;
	enq(start, 0, 0), mk[start] = 1;
	while (qsize) {
		s = que[0].s, _tm = que[0].tm, _tr = que[0].tr, deq();
		if (s == goal) { *atm = _tm, *atr = _tr; return 1; }

		for (i = 0; i < hi[s]; i++) {
			e = to[s][i];
			if (mk[e]) continue;
			mk[e] = 1;
			enq(e, _tm + tm[s][i], _tr + tr[s][i]);
		}
	}
	return 0;
}

int main()
{
	int N, T, a, t, i, j, k, s, e, atm, atr;
	int *memo;

	N = in(), T = in();

	/* start/goal station */
	ins(name[0]);
	ins(name[1]);

	sz = 2;
	memo = malloc(MAX << 2);
	while (N--) {
		a = in(); for (i = 0; i < a; i++) {
			ins(name[sz]);
			k = insert(name[sz], sz);
			if (k < 0) {
				k = sz++;
				memo[i] = k;
				getmem(k, lim[k] = 2, 0);
			} else {		// if (k >= 0)
				if (!trSta[k]) {
					trSta[k] = s = sz++;
					getmem(k, lim[k] += 2, 1);
					getmem(s, lim[s] = 4, 0);
					j = hi[k]++, to[k][j] = s, tm[k][j] = T, tr[k][j] = 1;
					j = hi[s]++, to[s][j] = k, tm[s][j] = 0, tr[s][j] = 0;
				} else {
					s = trSta[k];
					if (hi[s] >= lim[s]) {
						k = lim[s]; if (k > 10000) k = 10000;
						lim[s] += k;
						getmem(s, lim[s], 1);
					}
				}

				memo[i] = e = sz++;
				getmem(e, lim[e] = 4, 0);
				j = hi[s]++, to[s][j] = e, tm[s][j] = 0, tr[s][j] = 0;
				j = hi[e]++, to[e][j] = s, tm[e][j] = T, tr[e][j] = 1;
			}
		}
		s = memo[0]; for (i = 1; i < a; i++) {
			e = memo[i], t = in();
			k = hi[s]++, to[s][k] = e, tm[s][k] = t, tr[s][k] = 0;
			k = hi[e]++, to[e][k] = s, tm[e][k] = t, tr[e][k] = 0;
			s = e;
		}
	}
	free(memo);

	atm = atr = 0;
	s = insert(name[0], 0);
	if (trSta[s] > 0) s = trSta[s];
	e = insert(name[1], 1);
	if (trSta[e] > 0) e = trSta[e], atm = -T, atr = -1;

	if (dijkstra(&i, &j, s, e)) printf("%d %d\n", atm+i, atr+j);
	else puts("-1");
	return 0;
}
