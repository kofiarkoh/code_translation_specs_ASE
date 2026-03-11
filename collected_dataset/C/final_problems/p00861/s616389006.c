// AOJ 1282: Bug Hunt
// 2017.10.22 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define HASHSIZ 2003
typedef struct { char x; int id, v; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;

int lookup(char x, int id)
{
	long long t;
	HASH *hp;
	
	t = x; t = ((t << 32LL) + id) % HASHSIZ;
	hp = hash + (int)t;
	while (hp->x) {
		if (hp->x == x && hp->id == id) return hp->v;
		if (++hp == hashend) hp = hash;
	}
	return -1;
}

void insert(char x, int id, int v)
{
	long long t;
	HASH *hp;
	
	t = x; t = ((t << 32LL) + id) % HASHSIZ;
	hp = hash + (int)t;
	while (hp->x) {
		if (hp->x == x && hp->id == id) { hp->v = v; break; }
		if (++hp == hashend) hp = hash;
	}
	hp->x = x, hp->id = id, hp->v = v;
}

int range['z'+1];
char buf[130], *p;

char *parse(int *val, char *p)
{
	char x;
	int v, w;

	if (isdigit(*p)) {
		v = 0; while (isdigit(*p)) v = 10*v + (*p++ - '0');
		*val = v;
		return p;
	}
	x = *p, p += 2;   // [
	if ((p = parse(&v, p)) == NULL) return NULL;
	if (range[x] < 0) { range[x] = v; return p; }
	
	if (v >= range[x]) return NULL;
	p++;			  // ]
	if (*p == ']' || *p < ' ') {
		if ((*val = lookup(x, v)) < 0) return NULL;
		return p;
	}
	p++;			  // =
	if ((p = parse(&w, p)) == NULL) return NULL;
	insert(x, v, w);
	return p;
}

int main() 
{
	int i, err, v;

	while (1) {
		memset(hash, 0, sizeof(hash)), memset(range, -1, sizeof(range));
		for (err = 0, i = 1; fgets(p=buf, 128, stdin) && *p != '.'; i++) {
			if (!err && parse(&v, p) == NULL) err = i;
		}
		if (i == 1) break;
		printf("%d\n", err);
	}
	return 0; 
}