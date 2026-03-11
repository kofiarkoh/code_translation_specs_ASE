// AOJ 1528: Warping Girl
// 2019.2.21 bal4u

#include <stdio.h>
#include <stdlib.h>

// Hash Functions
#define HASHSIZ 4999
typedef struct { int key; int id; } HASH;
HASH hash[HASHSIZ + 5], *hashend = hash + HASHSIZ;

int lookup(int key)
{
	HASH *p = hash + ++key % HASHSIZ;
	while (p->key) {
		if (p->key == key) return p->id;
		if (++p == hashend) p = hash;
	}
	return -1;
}

void insert(int key, int id)
{
	HASH *p = hash + ++key % HASHSIZ;
	while (p->key) {
		if (p->key == key) return;
		if (++p == hashend) p = hash;
	}
	p->key = key, p->id = id;
}

#define gc() getchar()
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

typedef struct { int P, D, T; int tm;  char f; } T;
T tbl[2005];

int cmp(const void *a, const void *b)
{
	return ((T *)a)->P - ((T *)b)->P;
}

int main()
{
	int i, j, ii, L, n;
	int id, now, tim, min;

	L = in(), n = in();
	for (i = 0; n--; ) {
		tbl[i].P = in(), tbl[i].D = in(), tbl[i].T = in();
		if (tbl[i].D == 0) continue;
		tbl[i].f = 1, tbl[i].tm = L;
		tbl[i + 1].P = tbl[i].P + tbl[i].D, tbl[i + 1].tm = L;
		i += 2;
	}
	n = i;
	qsort(tbl, n, sizeof(T), cmp);
	for (i = 0; i < n; i++) {
		if (tbl[i].f == 0) insert(tbl[i].P, i);
	}

	now = tim = 0;
	for (i = 0; i < n; ) {
		min = tbl[i].tm;
		for (j = i+1; tbl[j].P == tbl[i].P; j++) {
			if (tbl[j].tm < min) min = tbl[j].tm;
		}
		if (min > tbl[i].P - now + tim) min = tbl[i].P - now + tim;
		now = tbl[i].P, tim = min;
		ii = j;

		for (j = i; j < ii; j++) {
			if (tbl[j].f) {
				id = lookup(tbl[j].P + tbl[j].D);
				if (tbl[id].tm > tim + tbl[j].T) tbl[id].tm = tim + tbl[j].T;
			}
		}
		i = ii;
	}
	printf("%d\n", L - now + tim);
	return 0;
}
