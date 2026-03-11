#include <stdio.h>

#define N	300000
#define MD	1000000007

int min(int a, int b) { return a < b ? a : b; }

int vv[N * 3 + 1], ff[N * 3 + 1], gg[N * 3 + 1], p3[N * 2 + 1];

void init() {
	int i;

	ff[0] = gg[0] = 1;
	for (i = 1; i <= N * 3; i++) {
		vv[i] = i == 1 ? 1 : (long long) vv[i - MD % i] * (MD / i + 1) % MD;
		ff[i] = (long long) ff[i - 1] * i % MD;
		gg[i] = (long long) gg[i - 1] * vv[i] % MD;
	}
	p3[0] = 1;
	for (i = 1; i <= N * 2; i++)
		p3[i] = p3[i - 1] * 3LL % MD;
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
		sum = (sum * 2 - choose(i, i - k) - choose(i, m)) % MD;
	}
	if (ans < 0)
		ans += MD;
	printf("%d\n", ans);
	return 0;
}
