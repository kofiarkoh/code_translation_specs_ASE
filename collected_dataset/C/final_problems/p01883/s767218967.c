// AOJ 2783 Parentheses
// 2018.2.2 bal4u

#include <stdio.h>

void pr(long long len, char c) { while (len--) putchar(c); }

int main()
{
	long long a, lo, hi, mi;

	scanf("%lld", &a);
	lo = 0, hi = a+1;
	while (hi - lo > 1) {
		mi = (lo + hi) >> 1;
		if ((mi*(mi+1)>>1) <= a) lo = mi; else hi = mi;
	}
	a -= lo*(lo+1) >> 1;
	if (a) pr(a, ')'), putchar('('), pr(1+lo-a, ')');
	else   pr(lo, ')');
	pr(lo, '('), putchar('\n');
	return 0;
}
