// AOJ 0322: Slates
// 2018.1.1 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef unsigned long long ull;
#define HASHSIZ 100019

typedef struct { ull k1; int k2; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;

int lookup(ull k1, int k2)
{
	HASH *p = hash + k2 % HASHSIZ;
	while (p->k1) {
		if (p->k1 == k1 && p->k2 == k2) return 1;
		if (++p == hashend) p = hash;
	}
	return 0;
}

void insert(ull k1, int k2)
{
	HASH *p = hash + k2 % HASHSIZ;
	while (p->k1) {
		if (p->k1 == k1 && p->k2 == k2) return;
		if (++p == hashend) p = hash;
	}
	p->k1 = k1, p->k2 = k2;
}

#define M2     1000000007
#define M1     101
#define BASE   200
#define HEAD_S 1
#define TAIL_S 2
#define QMARK  4

char word[50002][402]; int wlen[50002];
char buf[202], *p;
int z[402], z2[402];

int in()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf);
	p++;
	return n;
}

ull rollingHash(int *k2, char *s)
{
	ull k, k1;

	k1 = k = 0;
	while (*s) {
		k1 = *s + k1 * M1;
		k = (*s + k * M1) % M2;
		s++;
	}
	*k2 = (int)k;
	return k1;
}

void Zalgo(int *z, char *s, int n)
{
	int i, j, k;

	z[0] = n;
    i = 1, j = 0;
    while (i < n) {
		while (i+j < n && s[j] == s[i+j]) ++j;
        z[i] = j;
        if (j == 0) { ++i; continue;}
        k = 1;
        while (i+k < n && k+z[k] < j) z[i+k] = z[k], ++k;
        i += k; j -= k;
    }
}

int main()
{
	int n, m, i, j, k, w, w1, f, ans;
	ull k1; int k2;
	char *p2;
	
	fgets(p=buf, 20, stdin);
	n = in(), m = in();
	for (i = 0; i < n; i++) {
		fgets(p = word[i]+BASE, 202, stdin);
		w = strlen(p)-1;
		if (*(p+w) < ' ') *(p+w) = 0; else w++;
		wlen[i] = w;

		k1 = rollingHash(&k2, p);
		insert(k1, k2);
	}

	for (i = 0; i < m; i++) {
		ans = 0, f = 0;
		fgets(p=buf, 202, stdin);
		if (*p == '*') p++, f |= HEAD_S;
		while (*p >= ' ') {
			if (*p == '?') f |= QMARK, p2 = p, w1 = p2-buf;
			else if (*p == '*') { p++, f |= TAIL_S; break; }
			p++;
		}
		*p = 0;
		if (f & QMARK) *p2++ = 0, w = p-p2; 
		else w = p-buf;

		if (!f) {
			k1 = rollingHash(&k2, buf);
			if (lookup(k1, k2)) ans++;
		} else if (f & QMARK) {
			if (f & HEAD_S) {
				w1--;
				for (j = 0; j < n; j++) {
					if (wlen[j] <= w1+w) continue;
					if (w1 > 0) {
						memcpy(p=word[j]+BASE-w1, buf+1, w1);
						Zalgo(z, p, wlen[j]+w1);
					}
					if (w > 0) {
						memcpy(p=word[j]+BASE-w, p2, w);
						Zalgo(z2, p, wlen[j]+w);
					}

					for (k = 0; k < wlen[j]; k++) {
						if (w1 > 0 && w > 0) {
							if (z[k+w1] >= w1 && z2[k+w1+w1+1] >= w) { ans++; break; }
						} else if (w1 > 0) {
							if (z[k+w1] >= w1) { ans++; break; }
						} else if (w > 0) {
							if (z2[k+w1+w1+1] >= w) { ans++; break; }
						}
					}
				}
			} else if (f & TAIL_S) {
				w--;
				for (j = 0; j < n; j++) {
					if (wlen[j] <= w1+w) continue;
					if (w1 > 0) {
						memcpy(p=word[j]+BASE-w1, buf, w1);
						Zalgo(z, p, wlen[j]+w1);
						if (z[w1] < w1) continue;
					}
					if (w > 0) {
						memcpy(p=word[j]+BASE-w, p2, w);
						Zalgo(z, p, wlen[j]+w);
						if (z[w1+w1+1] >= w) ans++;
					} else ans++;
				}
			} else {
				for (j = 0; j < n; j++) {
					if (wlen[j] != w1+w+1) continue;
					if (w1 > 0) {
						memcpy(p=word[j]+BASE-w1, buf, w1);
						Zalgo(z, p, wlen[j]+w1);
						if (z[w1] < w1) continue;
					}
					if (w > 0) {
						memcpy(p=word[j]+BASE-w, p2, w);
						Zalgo(z, p, wlen[j]+w);
						if (z[w1+w1+1] >= w) ans++;
					} else ans++;
				}
			}
		} else {
			if (f & HEAD_S) {
				w--;
				for (j = 0; j < n; j++) {
					memcpy(p=word[j]+BASE-w, buf+1, w);
					Zalgo(z, p, wlen[j]+w);
					for (k = 0; k < wlen[j]; k++) if (z[k+w] >= w) { ans++; break; }
				}
			} else if (f & TAIL_S) {
				w--;
				for (j = 0; j < n; j++) {
					memcpy(p=word[j]+BASE-w, buf, w);
					Zalgo(z, p, wlen[j]+w);
					if (z[w] >= w) ans++;
				}
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}

