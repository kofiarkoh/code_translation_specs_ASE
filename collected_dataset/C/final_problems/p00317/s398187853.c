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

#define M     1000000007
#define BASE   200
#define HEAD_S 1
#define TAIL_S 2
#define QMARK  4

int n;
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
	char c;

	k1 = k = 0;
	while (*s) {
		c = *s++ - ('a'-1);
		k1 = (k1 << 5) + c;
		k = ((k  << 5) + c) % M;
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

int check1(char *s1, int w1, char *s2, int w2, int eq)
{
	int i, ans = 0;
	char *p;

	for (i = 0; i < n; i++) {
		if (eq) { if (wlen[i] != w1+w2+1) continue; }
		else    { if (wlen[i] <= w1+w2)   continue; }
		memcpy(p=word[i]+BASE-w1, s1, w1);
		Zalgo(z, p, wlen[i]+w1);
		if (z[w1] < w1) continue;

		memcpy(p=word[i]+BASE-w2, s2, w2);
		Zalgo(z, p, wlen[i]+w2);
		if (z[wlen[i]] >= w2) ans++;
	}
	return ans;
}

int check2(char *s, int w, int pos)
{
	int i, ans = 0;
	char *p;

	for (i = 0; i < n; i++) {
		if (wlen[i] < w) continue;
		memcpy(p=word[i]+BASE-w, s, w);
		Zalgo(z, p, wlen[i]+w);
		if (z[pos? wlen[i]: w] >= w) ans++;
	}
	return ans;
}

int main()
{
	int m, i, j, w, w1, f, ans;
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
		f = 0;
		fgets(p=buf, 202, stdin);

		if (*p == '*') p++, f |= HEAD_S;
		while (*p >= ' ') {
			if      (*p == '?') f |= QMARK, p2 = p, w1 = p2-buf;
			else if (*p == '*') f |= TAIL_S;
			p++;
		}
		*p = 0;
		if (f & QMARK) *p2++ = 0, w = p-p2; 
		else w = p-buf;

		ans = 0;
		if (!f) {
			k1 = rollingHash(&k2, buf);
			if (lookup(k1, k2)) ans++;
		} else if (f & QMARK) {
			if (f & HEAD_S) {
				w1--;
				for (j = 0; j < n; j++) {
					if (wlen[j] <= w1+w) continue;
					memcpy(p=word[j]+BASE-w1, buf+1, w1);
					Zalgo(z, p, wlen[j]+w1);
					memcpy(p=word[j]+BASE-w, p2, w);
					Zalgo(z2, p, wlen[j]+w);
					if (z[wlen[j]-w1-1] >= w1 && z2[wlen[j]] >= w) ans++;
				}
			} else if (f & TAIL_S) ans = check1(buf, w1, p2, w-1, 0);
			else                   ans = check1(buf, w1, p2, w,   1);
		} else {
			if      (f & HEAD_S) ans = check2(buf+1, w-1, 1);
			else if (f & TAIL_S) ans = check2(buf  , w-1, 0);
		}
		printf("%d\n", ans);
	}
	return 0;
}

