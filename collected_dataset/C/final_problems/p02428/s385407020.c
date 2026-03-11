// AOJ ITP2_11_B Enumeration of Subsets II
// 2019.3.9 bal4u

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

int main()
{
	int i, j, k, n, max, mask;

	n = in(), max = 1 << n;
	k = in(), mask = 0;
	while (k--) mask |= 1 << in();
	for (i = 0; i < max; i++) {
		if ((i & mask) != mask) continue;
		out(i), pc(':');
		k = i; for (j = 0; k; j++) {
			if (k & 1) pc(' '), out(j);
			k >>= 1;
		}
		pc('\n');
	}
	return 0;
}
