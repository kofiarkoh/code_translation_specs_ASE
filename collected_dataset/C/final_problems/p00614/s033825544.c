// AOJ 1028: ICPC: Ideal Coin Payment and Change
// 2017.10.14

#include <stdio.h>

int n[6];
int c[6] = { 500, 100, 50, 10, 5, 1 };
int k[1001];

int pay(int p)
{
	int i, k, cnt;

	for (cnt = 0, i = 0; i < 6; i++) if (c[i] <= p) {
		k = p/c[i]; if (k > n[i]) k = n[i];
		cnt += k;
		p -= c[i]*k;
	}
	if (p > 0) return -1;
	return cnt;
}

int main()
{
	int p, i, j, q, cnt, ans;

	for (p = 1; p <= 1000; p++) {
		for (cnt = 0, q = p, j = 0; j < 6; j++) if (c[j] <= q) {
			cnt += q/c[j];
			q %= c[j];
		}
		k[p] = cnt;
	}

	while (scanf("%d", &p) && p > 0) {
		for (j = 5; j >= 0; j--) scanf("%d", n+j);
		ans = 0x7fffffff;
		for (i = p; i <= p+1000; i++) {
			if ((j = pay(i)) < 0) continue;
			if ((cnt = j + k[i-p]) < ans) ans = cnt;
		}
		printf("%d\n", ans);
	}
	return 0;
}