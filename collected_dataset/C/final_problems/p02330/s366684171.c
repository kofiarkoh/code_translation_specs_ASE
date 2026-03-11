// AOJ DPL_4_B Coin Combination Problem II
// 2018.5.10 bal4u
 
#include <stdio.h>
#include <stdlib.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
 
int in()
{
    int n = 0, c = gc();
    do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
    return n;
}

long long inLL()
{
    long long n = 0, c = gc();
    do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
    return n;
}

#define MAX  ((1<<20)+5)

long long a[42]; int w;
long long s1[42][MAX]; int w1[42];
long long s2[42][MAX]; int w2[42];

int binarysearch_upper(long long *a, long long x, int r)
{
	int m, l = 0;

    while (l < r) {
        m = (l+r) >> 1;
		if (a[m] <= x) l = m + 1; else r = m;
    }
	return l;
}

int binarysearch_lower(long long *a, long long x, int r)
{
	int m, l = 0;

    while (l < r) {
        m = (l+r) >> 1;
        if (a[m] < x) l = m + 1; else r = m;
    }
	return l;
}

int cmp(long long *a, long long *b)
{
	if (*a > *b) return -1;
	return (*a != *b);
}

int main()
{
	int N, K, i, j, m1, m2, n, lim;
	long long L, R, t, ans;

	N = in(), K = in(), L = inLL(), R = inLL();

	w = 0; for (i = 0; i < N; i++) {
		t = inLL();
		if (t + K-1 <= R) a[w++] = t;
	}

	m1 = w/2, lim = 1 << m1;
	for (i = 0; i < lim; i++) {
		t = 0, n = 0;
		for (j = 0; j < m1; j++) if ((i >> j) & 1) {
			t += a[j], n++;
		}
		s1[n][w1[n]++] = t;
	}

	m2 = w - m1;
	lim = 1 << m2;
		for (i = 0; i < lim; i++) {
		t = 0, n = 0;
		for (j = 0; j < m2; j++) if ((i >> j) & 1) {
			t += a[j+m1], n++;
		}
		s2[n][w2[n]++] = t;
	}

	ans = 0;
	i = K - m1; if (i < 0) i = 0;
	lim = w - m1; if (lim > K) lim = K;
	while (i <= lim) {
		for (j = 0; j < w2[i]; j++) {
			long long x1 = R-s2[i][j], x2 = L-s2[i][j];
			if (x1 >= 0 && x2 >= 0) {
				ans += (long long)binarysearch_upper(s1[K-i], x1, w1[K-i]) -
					              binarysearch_lower(s1[K-i], x2, w1[K-i]);
			}
		}
		i++;
	}
	printf("%lld\n", ans);
	return 0;
}
