// AOJ 2494 Sliding GCD
// 2018.3.18 bal4u

#include <stdio.h>

//#define putchar_unlocked(x)  putchar(x)

void out(int n)
{
	int rev, c;
    if (!n) { putchar_unlocked('0'); return ; }
	c = 0, rev = n;
	while (!(rev % 10)) c++, rev /= 10;
	rev = 0;
	while (n) rev = 10*rev + n % 10, n /= 10;
	while (rev) putchar_unlocked(rev % 10 + '0'), rev /= 10;
    while (c--) putchar_unlocked('0');
}

#define MAX   100000
#define DMAX     130	// max 126

int   t[MAX+1][DMAX+1]; short w[MAX+1];
short c[MAX+1];

void divisor_tble(int N)
{
	int i, k, n = (N>>1)+1;
	for (i = 2; i <= n; i++) {
		for (k = i<<1; k <= N; k += i) t[k][w[k]++] = i;
	}
}

int main()
{
	int  N, W, nw;
	int i, k, lo, hi, ans;

	scanf("%d%d", &N, &W), nw = N-W+1;
	if (W == 1) {
		putchar_unlocked('1');
		for (k = 2; k <= nw; k++) putchar_unlocked(' '), putchar_unlocked('1');
		putchar_unlocked('\n');
		return 0;
	}

	divisor_tble(N);

	out(W);
	ans = 1;
	for (k = 1; k <= W; k++) {
		c[k] += 2; ans++;
		for (i = 0; i < w[k]; i++) if (++c[t[k][i]] == 2) ans++;
	}

	lo = 1, hi = W;
	for (k = 2; k <= nw; k++) {
		for (i = 0; i < w[lo]; i++) if (--c[t[lo][i]] == 1) ans--;
		c[lo] -= 2; if (c[lo] < 2) ans--;
		lo++, hi++;
		c[hi] += 2; if (c[hi] == 2) ans++;
		for (i = 0; i < w[hi]; i++) if (++c[t[hi][i]] == 2) ans++;
		putchar_unlocked(' '), out(ans);
	}
	putchar_unlocked('\n');
	return 0;
}
