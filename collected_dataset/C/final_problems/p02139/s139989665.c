// AOJ 3041: Round And Round
// 2019.2.22 bal4u

#include <stdio.h>
#include <stdlib.h>

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
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

int main()
{
	int a, N, Q, q, k;

	N = in(), Q = in();
	a = 0;
	while (Q--) {
		q = gc(), gc();
		k = in();
		if (q & 1) {
			a += k;
			if (a >= N) a -= N;
		}
		else out(1 + (a + k - 1) % N);
	}
	return 0;
}
