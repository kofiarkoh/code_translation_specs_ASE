#include <stdio.h>

#define N	300000
#define MD	1000000007

int min(int a, int b) { return a < b ? a : b; }

long long power(int a, int k, int md) {
	long long b = a, p = 1;

	while (k) {
		if (k & 1)
			p = p * b % md;
		b = b * b % md;
		k >>= 1;
	}
	return p;
}

int ff[N * 2 + 1], gg[N * 2 + 1], p3[N * 2 + 1];

void init() {
	int i, f, p;

	f = 1;
	for (i = 0; i <= N * 2; i++) {
		gg[i] = power(ff[i] = f, MD - 2, MD);
		f = (long long) f * (i + 1) % MD;
	}
	p = 1;
	for (i = 0; i <= N * 2; i++) {
		p3[i] = p;
		p = (long long) p * 3 % MD;
	}
}

long long choose(int n, int k) {
	return k < 0 || k > n ? 0 : (long long) ff[n] * gg[k] % MD * gg[n - k] % MD;
}

int main() {
	int n, m, k, i, ans, sum;

	init();
	scanf("%d%d%d", &n, &m, &k);
	ans = 0, sum = 1;
	for (i = 0; i <= m + k; i++) {
		ans = (ans + sum * choose(n - 1 + i, n - 1) % MD * p3[m + k - i]) % MD;
		sum = (sum * 2 - choose(i, i - k) - choose(i, min(i, m))) % MD;
		if (i + 1 <= m)
			sum = (sum + 1) % MD;
	}
	if (ans < 0)
		ans += MD;
	printf("%d\n", ans);
	return 0;
}
