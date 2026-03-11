// AOJ DPL_4_B Coin Combination Problem II
// 2018.5.10 bal4u
 
#include <stdio.h>
 
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
typedef struct { long long s; char n; } QUE;
QUE que[MAX]; int top, end;

long long a[42];
int w;

long long sum[42][MAX]; int hi[42];

int main()
{
	int N, K, m, i, j, k, n, n1, stop, ans;
	long long L, R, v, t;

	N = in(), K = in(), L = inLL(), R = inLL();
	ans = 0;

	if (K == 1) {
		for (i = 0; i < N; i++) {
			t = inLL();
			if (t >= L && t <= R) ans++;
		}
		printf("%d\n", ans);
		return 0;
	}

	w = 0; for (i = 0; i < N; i++) {
		t = inLL();
		if (t + K-1 <= R) a[w++] = t;
	}

	m = w/2;
	que[0].s = 0, que[0].n = 0, top = 0, end = 1;
	for (i = 0; i < m; i++) {
		top = 0, stop = end;
		while (top != stop) {
			v = que[top].s, n = que[top++].n, n1 = n+1;
			t = v + a[i];

			if (n1 == K) {
				if (t >= L && t <= R) ans++;
			} else if (n1 < K) {
				if (t < R) {
					que[end].s = t, que[end++].n = n1;
					sum[n1][hi[n1]++] = t;
				}
			}
		}
	}

	que[0].s = 0, que[0].n = 0, top = 0, end = 1;
	for (i = m; i < w; i++) {
		top = 0, stop = end;
		while (top != stop) {
			v = que[top].s, n = que[top++].n, n1 = n+1;
			t = v + a[i];

			if (n1 == K) {
				if (t >= L && t <= R) ans++;
			} else if (n1 < K) {
				if (t < R) {
					que[end].s = t, que[end++].n = n1;

					k = K-n1;
					if (k >= 0) {
						for (j = 0; j < hi[k]; j++) {
							long long s = t + sum[k][j];
							if (s >= L && s <= R) ans++;
						}
					}
				}
			}
		}
	}
	printf("%d\n", ans);
	return 0;
}
