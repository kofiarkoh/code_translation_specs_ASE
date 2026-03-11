// AOJ 0395: A Round Table for Sages
// 2019.2.23 bal4u

#include <stdio.h>
#include <stdlib.h>

#define gc() getchar()
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int a[12];

int main()
{
	int N, w, s;
	int i, lmin, rmin;

	N = in();
	s = 0; for (i = 0; i < N; i++) a[i] = in(), s += a[i];
	if (s == 0 || s == N) {
		puts("0");
		return 0;
	}
	lmin = rmin = 1001;
	for (i = 0; i < N; i++) {
		w = in();
		if (a[i]) {
			if (w < lmin) lmin = w;
		}
		else {
			if (w < rmin) rmin = w;
		}
	}
	printf("%d\n", lmin + rmin);
	return 0;
}
