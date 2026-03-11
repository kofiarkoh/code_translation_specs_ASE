// AOJ 1370 Hidden Anagrams
// 2018.2.8 bal4u

#include <stdio.h>
#include <string.h>

#define HASHSIZ		10007
typedef struct { int s; short f[26]; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;			// Hash table

void insert(int s, short *f)
{
	int i;
	unsigned long long k;
	HASH *tp;

	k = 0; for (i = 0; i < 26; i+=2) k = (k << 4) + f[i];
	tp = hash + (int)(((k << 12) + s) % HASHSIZ);
	while (tp->s) {
		if (tp->s == s && !memcmp(tp->f, f, 52)) return;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, memcpy(tp->f, f, 52);
}

int lookup(int s, short *f)
{
	int i;
	unsigned long long k;
	HASH *tp;

	k = 0; for (i = 0; i < 26; i+=2) k = (k << 4) + f[i];
	tp = hash + (int)(((k << 12) + s) % HASHSIZ);
	while (tp->s) {
		if (tp->s == s && !memcmp(tp->f, f, 52)) return 1;
		if (++tp == hashend) tp = hash;
	}
	return 0;
}

#define MAGIC 101
char sa[4004]; int wa;
char sb[4004]; int wb;
short f[26];

int main()
{
	int i, k, w, s;
	char *p, *q;

	fgets(p=sa, 4004, stdin); while (*p > ' ') *p -= 'a', p++; wa = p - sa;
	fgets(p=sb, 4004, stdin); while (*p > ' ') *p -= 'a', p++; wb = p - sb;
	w = MAGIC; if (w > wa) w = wa; if (w > wb) w = wb;

	for (k = w; k > 0; k--) {
		memset(hash, 0, sizeof(hash));

		memset(f, 0, sizeof(f));
		s = 1, p = sa; for (i = 0; i < k; i++) s += *p, f[*p++]++;
		insert(s, f);

		i = wa - k, q = sa; while (i--) {
			s -= *q, f[*q++]--;
			s += *p, f[*p++]++;
			insert(s, f);
		}

		memset(f, 0, sizeof(f));
		s = 1, p = sb; for (i = 0; i < k; i++) s += *p, f[*p++]++;
		if (lookup(s, f)) goto done;

		i = wb - k, q = sb; while (i--) {
			s -= *q, f[*q++]--;
			s += *p, f[*p++]++;
			if (lookup(s, f)) goto done;
		}
    }
done:
	printf("%d\n", k);
	return 0;
}
