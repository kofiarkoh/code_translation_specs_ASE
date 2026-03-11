// AOJ 1563: Array Update
// 2017.10.24 bal4u@uu

#include <stdio.h>

#define HASHSIZ 101
typedef struct { int id, v; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;

int lookup(int id)
{
	HASH *p = hash + id % HASHSIZ;
	while (p->id) {
		if (p->id == id) return p->v;
		if (++p == hashend) p = hash;
	}
	return -1;
}

void insert(int id, int v)
{
	HASH *p = hash + id % HASHSIZ;
	while (p->id) {
		if (p->id == id) { p->v = v; return; }
		if (++p == hashend) p = hash;
	}
	p->id = id, p->v = v;
}

int n, a, d;

int get(int id)
{
	int v;
	if ((v = lookup(id)) < 0) v = a + (id-1) * d;
	return v;
}

int main() 
{
	int m, x, y, z, k;
	int vy, vz;

	scanf("%d%d%d%d", &n, &a, &d, &m);
	while (m-- > 0) {
		scanf("%d%d%d", &x, &y, &z);
		if (x == 0) {
			vy = get(y), vz = get(z);
			insert(y, vz), insert(z, vy);
		} else {
			vz = get(z);
			insert(y, vz);
		}
	}
	scanf("%d", &k);
	printf("%d\n", get(k));
	return 0;
}