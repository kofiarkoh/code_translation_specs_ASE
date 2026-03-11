// AOJ 1087: Dimensional Analysis
// 2017.12.13 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define HASHSIZ 101
typedef struct { long long k; char *s; } HASH;
HASH hash1[HASHSIZ+2], *hashend1 = hash1+HASHSIZ;			// Hash table
HASH hash2[HASHSIZ+2], *hashend2 = hash2+HASHSIZ;

char *getStr(long long k)
{
	int i = (int)(k % HASHSIZ);
	HASH *tp = hash1 + i;

	while (tp->k) {
		if (tp->k == k) return tp->s;
		if (++tp == hashend1) tp = hash1;
	}
	return NULL;
}

void regStr(long long k, char *s)
{
	int i = (int)(k % HASHSIZ);
	HASH *tp = hash1 + i;

	while (tp->k) {
		if (tp->k == k) tp->s = s;
		if (++tp == hashend1) tp = hash1;
	}
	tp->k = k, tp->s = s;
}

long long getK(char *s)
{
	int w = strlen(s);
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash2 + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->k;
		if (++tp == hashend2) tp = hash2;
	}
	return -1;
}

void regK(char *s, long long k)
{
	int w = strlen(s);
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash2 + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return;
		if (++tp == hashend2) tp = hash2;
	}
	tp->s = s, tp->k = k;
}

#define N1   5
#define N2   31
#define BASE 16

int n;
char str[30][25]; int sz;
char formula[101], *p;


long long factor()
{
	long long x;
	long long expr();
	char *q, c;

	if (*p == '(') { p++; if ((x = expr()) < 0) return -1LL; p++; }
	else {
		q = p; while (isalpha(*p)) p++;
		c = *p; *p = 0;
		x = getK(q);
		*p = c;
	}
	return x;
}

long long calc(long long x, long long y, int op)
{
	int i, a[5], b[5];

	for (i = 0; i < n; i++) {
		a[i] = (int)(x & N2);
		b[i] = (int)(y & N2);
		x >>= N1, y >>= N1;
	}
	for (i = 0; i < n; i++) {
		if (op) a[i] += b[i] - BASE;
		else    a[i] += BASE - b[i];
		if (a[i] <= 0 || a[i] >= N2) return -1LL;
	}
	for (x = 0, i = n-1; i >= 0; i--) x = (x << N1) + a[i];
	return x;
}

long long term()
{
	int op;
	long long x, y;

	if ((x = factor()) < 0) return -1LL;
	while (1) {
		if (*p == '*') op = 1;
		else if (*p == '/') op = 0;
		else break;
		p++;
		if ((y = factor()) < 0) return -1LL;
		x = calc(x, y, op);
		if (x < 0) return -1LL;
	}
	return x;
}

long long expr()
{
	long long x, y;
	
	if ((x = term()) < 0) return -1LL;
	while (1) {
		if (*p == '+' || *p == '-');
		else break;
		p++;
		if ((y = term()) < 0) return -1LL;
		if (x != y) return -1LL;
	}
	return x;
}

int main()  
{  
	int M, P, d, i;
	long long k;  
	char buf[25];

    while (scanf("%d%d%d", &n, &M, &P) && n > 0) {
		sz = 0;
		memset(hash1, 0, sizeof(hash1));
		memset(hash2, 0, sizeof(hash2));
		while (M--) {
			scanf("%s", str[sz++]);
			for (k = 0, i = 0; i < n; i++) {
				scanf("%d", &d);
				k = (k << N1) + (d + BASE);
			}
			regStr(k, str[sz-1]);
			regK(str[sz-1], k);
		}

		scanf("%s", formula);

		while (P--) {
			scanf("%s%s", str[sz++], buf);
			regK(str[sz-1], getK(buf));
		}

		p = formula;
		k = expr();
		if (k < 0) puts("error");
		else if ((p = getStr(k)) == NULL) puts("undefined");
		else puts(p);
    }
    return 0;
}