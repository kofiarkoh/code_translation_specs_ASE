// AOJ DSL_3_C The Number of Windows
// 2018.5.6 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in() {
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

long long ll() {
	long long n = 0; int c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

#define INF   0x10101010101010LL
#define MAX_N 100005
long long s[MAX_N];

// バイナリサーチ
// x より大きい最初の要素のインデックスを返す。つまり、 > x
int binarySearch(long long x, int N)
{
	int m, l = 1, r = N;

    while (l < r) {
        m = (l+r) >> 1;
        if (s[m] < x) l = m+1; else r = m;
    }
	if (s[l] == x) l++;
	return l;
}

int main()
{
	int N, Q, i, j;
	long long base, x, ans;

	N = in(), Q = in();
	base = (long long)N*(N+1) >> 1;

	for (i = 1; i <= N; i++) s[i] = s[i-1] + in();
	s[++N] = INF;		// caution: N -> N+1

	while (Q--) {
		x = ll();
//		j = 1; while (s[j] <= x) j++;
		j = binarySearch(x, N);
		ans = 0, i = 0;
		while (j < N) {
			while (s[j]-s[i] >  x) i++, ans += N-j;
			while (s[j]-s[i] <= x) j++;
		}
		printf("%lld\n", base-ans);
	}
	return 0;
}
