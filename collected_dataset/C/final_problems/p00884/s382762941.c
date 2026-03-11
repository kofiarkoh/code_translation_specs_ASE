// AOJ 1305: Membership Management
// 2017.10.3 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define HASHSIZ 2003
typedef struct { char *s; int id; } HASH;
HASH hash[HASHSIZ+2];		// Hash table

char name[1002][16]; int sz;
int info[1002];
char got[1002];

typedef struct { int id, lock, n, m[10]; } T;
T group[102]; int n;

int lookup(char *s, int id)
{
	int w = strlen(s);
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp;

	tp = hash + i;
	if (tp->s != NULL) {
		int inc = *s + (*(s + 1) << 1);
		while (1) {
			if (!memcmp(tp->s, s, w)) return tp->id;
			i += inc;
			if (i >= HASHSIZ) i -= HASHSIZ;
			tp = hash + i;
			if (tp->s == NULL) break;
		}
	}
	tp->s = s, tp->id = id;
	return -1;
}

int count(int m)
{
	int i, id, s;

	if (group[m].lock) return 0;
	group[m].lock = 1;
	for (s = 0, i = 0; i < group[m].n; i++) {
		id = group[m].m[i];
		if (info[id] >= 0) s += count(info[id]);
		else if (!got[id]) got[id] = 1, s++;
	}
	group[m].lock = 0;
	return s;
}

int main()
{
	int i, j, id;
	char buf[200], *p, *q;

	while (scanf("%d", &n) && n > 0) {
		sz = 0; memset(hash, 0, sizeof(hash));

		for (i = 0; i < n; i++) {
			scanf("%s", buf);
			p = buf; while (isalpha(*p)) p++;
//			if (*p != ':') while (1); // panic
			*p++ = 0;
			strcpy(name[sz], buf);
			if ((id = lookup(name[sz], sz)) < 0) id = sz++;
			group[i].id = id;  // group
			j = 0; while (*p) {
				q = p; while (isalpha(*p)) p++;
				*p++ = 0;
				strcpy(name[sz], q);
				if ((id = lookup(name[sz], sz)) < 0) id = sz++;
				group[i].m[j++] = id;
			}
			group[i].n = j, group[i].lock = 0;
		}
		memset(info, -1, sizeof(int)*sz);
		for (i = 0; i < n; i++) info[group[i].id] = i;
		memset(got, 0, sizeof(char)*sz);
		printf("%d\n", count(0));
	}
	return 0;
}