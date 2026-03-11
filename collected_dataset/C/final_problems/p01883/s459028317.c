// AOJ 2783 Parentheses
// 2018.2.2 bal4u

#include <stdio.h>
#include <string.h>

char s[50000];

int main()
{
	long long A, lo, hi, mi;

	scanf("%lld", &A);
	lo = 0, hi = A+1;
	while (hi - lo > 1) {
		mi = (lo + hi) >> 1;
		if ((mi*(mi+1)>>1) <= A) lo = mi; else hi = mi;
	}
	A -= lo*(lo+1) >> 1;

	if (A) {
		memset(s, ')', lo+2), s[lo+2] = 0;
		if (A < lo+2) s[A] = '(', A = -1;
		else A -= lo+2;
		printf(s);

		memset(s, '(', lo), s[lo] = 0;
		if (A >= 0) s[A] = '(';
		puts(s);
	} else {
		memset(s, ')', lo), s[lo] = 0; printf(s);
		memset(s, '(', lo); puts(s);
	}
	return 0;
}
