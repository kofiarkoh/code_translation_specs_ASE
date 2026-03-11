// AOJ 2494 Sliding GCD
// 2018.3.18 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX   100000
#define DMAX     130	// max 126

int t[MAX+1][DMAX+1], w[MAX+1];
int c[MAX+1];

void divisor_tble(int N)
{
	int i, k;
	for (i = 2; i <= N; i++) {
		for (k = i<<1; k <= N; k += i) t[k][w[k]++] = i;
	}
}

int main()
{
	int  N, W, nw, i, j, k, lo, hi, ans;

	scanf("%d%d", &N, &W), nw = N-W+1;
	if (W == 1) {
		putchar('1');
		for (k = 2; k <= nw; k++) putchar(' '), putchar('1');
		putchar('\n');
		return 0;
	}

	divisor_tble(N);

	printf("%d", W);
	ans = 0;
	for (i = 1; i <= W; i++) {
		for (j = 0; j < w[i]; j++) {
			if (++c[t[i][j]] == 2) ans++;
		}
	}

	lo = 1, hi = W+1;
	for (k = 2; k <= nw; k++) {
		for (i = 0; i < w[lo]; i++) {
			if (--c[t[lo][i]] == 1) ans--;
		}
		lo++;
		for (i = 0; i < w[hi]; i++) {
			if (++c[t[hi][i]] == 2) ans++;
		}
		hi++;
		j = 0; for (i = lo; i <= hi; i++) if (c[i] < 2) j++;
//printf("\nk %d, lo %d, hi %d, ans %d, j %d\n", k, lo, hi, ans, j);
		printf(" %d", ans+j);
	}
	putchar('\n');
	return 0;
}
