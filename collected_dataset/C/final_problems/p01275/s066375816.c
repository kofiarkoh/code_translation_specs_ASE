// AOJ 2157: Dial Lock
// 2017.11.9 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HASHSIZ 20011
long long hash[HASHSIZ+2], *hashend = hash+HASHSIZ;

int insert(long long n)
{
	long long *p = hash + n % HASHSIZ;
	while (*p) {
		if (*p == n) return 0;
		if (++p == hashend) p = hash;
	}
	*p = n;
	return 1;
}

#define QSIZE 15000
typedef struct { long long s; int k; } Q;
Q q[QSIZE+2]; int top, end;

long long s2x(char *s)
{
	long long x = 0;
	while (*s) x = (x << 4) | (*s++ & 0xf);
	return x;
}

int main()
{
	int n, i, k, d, ans;
	char ss[11], tt[11];
	long long s, t, x, mask;

	while (scanf("%d", &n) && n > 0) {
		memset(hash, 0, sizeof(hash));
		scanf("%s%s", ss, tt);
		s = s2x(ss), t = s2x(tt);
		q[0].s = s, q[0].k = 0; top = 0, end = 1; insert(s);
		ans = -1;
		while (top < end) {
			s = q[top].s, k = q[top++].k;
			if (s == t) { ans = k; break; }
			for (mask = 0xfLL << 4*(n-1), i = 0; (s&mask) == (t&mask); i++, mask >>= 4);
			d = (int)(((t&mask) - (s&mask)) >> 4*(n-i-1));
			if (d < 0) d += 10;
			for ( ; i < n; i++, mask >>= 4) {
				x = (s >> 4*(n-i-1)) & 0xf;
				x += d; if (x >= 10) x -= 10;
				s = (s & ~mask) | (x << 4*(n-i-1));
				if (insert(s)) q[end].s = s, q[end++].k = k+1;
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}