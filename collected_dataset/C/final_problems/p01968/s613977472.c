// AOJ 2868 Hierarchical Calculator
// 2018.3.29 bal4u

#include <stdio.h>

int a[63];
long long x[63];
int s[63], sz;

int main()
{
	int N, i, j, smax;
	long long max;

	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", a+i);
	max = x[0] = 1, smax = 0;
	for (i = 1; i <= N; i++) {
		x[i] = a[i]*x[i-1];
		if (x[i] > max) max = x[i], smax = i;
		if (x[i] == 0) break;
	}

	if (smax == 0) { puts("0"); return 0; }
	sz = 0, s[sz++] = smax;
	for (i = smax-1; i >= 1; i--) {
		for (j = 1; ; j++) {
			if (j >= i) { s[sz++] = i; break; }
			if (x[j] == x[i]) { s[sz++] = j; i = j; break; }
		}

	}
	printf("%d\n", sz);
	for (i = sz-1; i >= 0; i--) printf("%d\n", s[i]);
	return 0;
}
