// AOJ 1345: Bit String Reordering
// 2017.12.4 bal4u@uu

#include <stdio.h>

#define INF 1000

int b[16];
int zero[16], one[16];

int swap(int *a, int goal, int fr, int to)
{
	int i;

	for (i = fr+1; i < to; i++) {
		if (a[i] == goal) { a[i] = a[fr], a[fr] = goal; return i - fr; }
	}
	return INF;
}

int main()
{
	int n, m, p, i, j, bit, cnt0, cnt1;

	scanf("%d%d", &n, &m);
	for (i = 0; i < n; i++) scanf("%d", b+i);
	for (bit = 0, j = i = 0; i < m; i++, bit = !bit) {
		scanf("%d", &p);
		while (p--) zero[j] = bit, one[j++] = !bit;
	}

	for (cnt0 = cnt1 = 0, i = 0; i < n; i++) {
		if (zero[i] != b[i]) cnt0 += swap(zero, b[i], i, n);
		if ( one[i] != b[i]) cnt1 += swap( one, b[i], i, n);
	}
	if (cnt0 > cnt1) cnt0 = cnt1;
	printf("%d\n", cnt0);
	return 0;
}