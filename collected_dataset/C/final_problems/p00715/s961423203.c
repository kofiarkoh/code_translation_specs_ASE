// AOJ 1134: Name the Crossing
// 2017.11.13 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HASHSIZ 503
typedef struct { char *s; int id; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;			// Hash table

int insert(char *s, int w, int id)
{
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->id;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->id = id;
	return -1;
}

int lookup(char *s, int w)
{
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->id;
		if (++tp == hashend) tp = hash;
	}
	return -1;
}

#define INT 0x20202020
char name[202][36]; int sz;
int rel[202][202];

int main()
{
	int n, m, w, i, j, k;
	char buf[36], *p, *q;

	while (fgets(buf, 10, stdin) && *buf != '0') {
		memset(hash, 0, sizeof(hash));
		memset(rel, INT, sizeof(rel));

		n = atoi(buf), sz = 0;
		while (n--) {
			fgets(name[sz], 35, stdin);
			p = q = name[sz]; while (*p != '-') p++; w = p - q, *p++ = 0;
			if ((i = insert(q, w, sz)) < 0) i = sz++;
			q = p; while (*p > ' ') p++; w = p - q, *p = 0;
			if ((j = insert(q, w, sz)) < 0) j = sz++;
			rel[i][j] = 1;
		}
		printf("%d\n", sz);

		for (i = 0; i < sz; i++) for (j = 0; j < sz; j++) if (j != i) {
			int f = 0;
			for (k = 0; k < sz; k++) {
				if (rel[i][k] & 1 && rel[k][j] & 1 ||
					rel[j][k] & 1 && rel[k][i] & 1) { f = 0; break; }
				if (!f) {
					if (rel[i][k] & 1 && rel[j][k] & 1 ||
						rel[k][i] & 1 && rel[k][j] & 1) f = 1;
				}
			}
			if (f) rel[i][j] = rel[j][i] = 0;
		}

		for (k = 0; k < sz; k++) for (i = 0; i < sz; i++) for (j = 0; j < sz; j++)
			if (rel[i][j] > rel[i][k] + rel[k][j]) rel[i][j] = rel[i][k] + rel[k][j];

		fgets(buf, 10, stdin); m = atoi(buf);
		while (m--) {
			fgets(buf, 35, stdin);
			p = q = buf; while (*p != '-') p++;	w = p - q, *p++ = 0;
			if ((i = lookup(q, w)) < 0) goto no;
			q = p; while (*p > ' ') p++; w = p - q, *p = 0;
			if ((j = lookup(q, w)) < 0) goto no;
			if (rel[i][j] & 1) puts("YES");
			else {
no:				puts("NO");
			}
		}
	}
	return 0;

}