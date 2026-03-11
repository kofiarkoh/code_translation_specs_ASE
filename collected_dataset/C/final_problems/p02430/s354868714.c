// AOJ ITP2_11_D Enumeration of Combinations
// 2019.3.10 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (isdigit(c));
	return n;
}

void out(int n)
{
	int i;
	char ob[30];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
}

void outs(char *s) { while (*s) pc(*s++); }

void pr(int k)
{
	int j;
	out(k), pc(':');
	for (j = 0; k; j++) {
		if (k & 1) pc(' '), out(j);
		k >>= 1;
	}
	pc('\n');
}

void combi(int lim, int from, int n)
{
	int i, m;

	for (i = from+1; i <= lim; i++) {
		m = n;
		m &= ~(1 << from);
		m |= 1 << i;
		pr(m);
		if (from > 0) combi(i - 1, from - 1, m);
	}
}

int main()
{
	int n, k, b;

	n = in(), k = in(), b = (1 << k)-1;
	pr(b);
	combi(n-1, k-1, b);
	return 0;
}
