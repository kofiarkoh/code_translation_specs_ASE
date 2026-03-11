// AOJ 1251: Pathological Paths
// 2017.10.19 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HASHSIZ 10007
typedef struct { char *s; int k; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;			// Hash table

void insert(char *s, int w, int k)
{
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!memcmp(tp->s, s, w)) return;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->k = k;
}

int lookup(char *s, int w)
{
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!memcmp(tp->s, s, w)) return tp->k;
		if (++tp == hashend) tp = hash;
	}
	return -2;
}

char path[102][102];
char s[2][102];
char t[2][120];

int main()
{
	int n, m, i, k, found[2];
	char *p, *q;

	while (scanf("%d%d", &n, &m) && n > 0) {
		memset(hash, 0, sizeof(hash));
		for (i = 0; i < n; i++) {
			scanf("%s", path[i]);
			p = path[i];
			while (*p) {
				p++;
				while (*p && *p != '/') p++;
				k = (*p == 0) ? i : -1;
				insert(path[i], p-path[i], k);
			}
		}

		while (m-- > 0) {
			memset(s, 0, sizeof(s)), memset(t, 0, sizeof(t));
			for (i = 0; i < 2; i++) scanf("%s", s[i]);
			for (i = 0; i < 2; i++) {
				p = s[i], q = t[i];
				if (*p++ != '/') goto notfound;
				*q++ = '/';
				while (*p) {
					if (*p == '/') {
						if (lookup(t[i], q-t[i]) != -1) goto notfound;
						*q++ = *p++;
					} else if (*p == '.' && *(p-1) == '/' &&
						(*(p+1) == '.' || *(p+1) == '/' || *(p+1) == 0)) {
						p++;
						if (*p == '.') {		// for /../
							if (*(p+1) != '/' && *(p+1) != 0) goto notfound;
							if (--q == t[i]) goto notfound;
							if (--q == t[i]) goto notfound;
							while (*q != '/') q--;
							p++, q++;
						}
						p++;
					} else *q++ = *p++;
				}

				*q = 0;
				if ((found[i] = lookup(t[i], q-t[i])) >= 0) goto next;
				if (*(q-1) == '/') {
					strcpy(q, "index.html");
					if ((found[i] = lookup(t[i], q-t[i]+10)) >= 0) goto next;
				} else {
					strcpy(q, "/index.html");
					if ((found[i] = lookup(t[i], q-t[i]+11)) >= 0) goto next;
				}
				goto notfound;
next:;
			}
			puts(found[0] == found[1] ? "yes" : "no");
			continue;
notfound:	puts("not found");
		}
	}
 	return 0;
}