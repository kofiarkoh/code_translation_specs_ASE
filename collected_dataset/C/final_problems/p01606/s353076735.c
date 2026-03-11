// AOJ 2494 Sliding GCD
// 2018.3.18 bal4u

#include <stdio.h>

#define MAX   100000
#define DMAX     130	// max 126

int t[MAX+1][DMAX+1], w[MAX+1];
int c[MAX+1];

void divisor_tble(int N)
{
	int i, k, n;
	n = (N>>1)+1;
	for (i = 2; i <= n; i++) {
		for (k = i<<1; k <= N; k += i) t[k][w[k]++] = i;
	}
}

int main()
{
	int  N, W, nw, i, k, lo, hi, one, two;

	scanf("%d%d", &N, &W), nw = N-W+1;
	if (W == 1) {
		putchar('1');
		for (k = 2; k <= nw; k++) putchar(' '), putchar('1');
		putchar('\n');
		return 0;
	}

	divisor_tble(N);

	printf("%d", W);
	one = W, two = 0;
	for (k = 1; k <= W; k++) {
		for (i = 0; i < w[k]; i++) {
			if (++c[t[k][i]] == 2) { two++;	if (t[k][i] <= W) one--; }
		}
	}

	lo = 1, hi = W;
	for (k = 2; k <= nw; k++) {
		for (i = 0; i < w[lo]; i++) {
			if (--c[t[lo][i]] == 1) { two--; if (t[lo][i] > lo && t[lo][i] <= hi) one++; }
		}
		if (c[lo] < 2) one--;
		lo++, hi++;
		for (i = 0; i < w[hi]; i++) {
			if (++c[t[hi][i]] == 2) { two++; if (t[hi][i] >= lo && t[hi][i] <= hi) one--; }
		}
		if (c[hi] < 2) one++;
		printf(" %d", one+two+1);
	}
	putchar('\n');
	return 0;
}
