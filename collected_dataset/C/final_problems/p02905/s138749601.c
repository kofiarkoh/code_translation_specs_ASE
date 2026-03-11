#include <stdio.h>

long long prime[1000] = {}, coeff[1000001] = {}, sum[1000001] = {};

int factorize(long long n, long long p[], int m[])
{
	int i, k;
	for (i = 0, k = 0; prime[i] * prime[i] <= n; i++) {
		if (n % prime[i] != 0) continue;
		p[k] = prime[i];
		for (m[k] = 0; n % p[k] == 0; m[k]++, n /= p[k]);
		k++;
	}
	if (n > 1) {
		p[k] = n;
		m[k++] = 1;
	}
	return k;
}

long long div_mod(long long x, long long y, long long z)
{
	if (x % y == 0) return x / y;
	else return (div_mod((1 + x / y) * y - x, (z % y), y) * z + x) / y;
}

int main()
{
	int i, N;
	long long A[200001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%lld", &(A[i]));
	
	int j, k;
	for (i = 2, k = 0; i <= 1010; i++) {
		for (j = 0; j < k; j++) if (i % prime[j] == 0) break;
		if (j == k) prime[k++] = i;
	}
	
	int h, l[100], m[100];
	long long p[100], n[2];
	for (i = 1; i <= N; i++) {
		sum[0] = (sum[0] + A[i]) % 998244353;
		k = factorize(A[i], p, m);
		for (j = 1, l[0] = 1; j <= k; j++) l[j] = 0;
		n[0] = p[0];
		n[1] = 1 - p[0];
		while (l[k] == 0) {
			coeff[n[0]] = n[1];
			sum[n[0]] = (sum[n[0]] + A[i] / n[0]) % 998244353;
			
			for (j = 0; j < k && l[j] == m[j]; j++);
			if (j < k) {
				n[0] *= p[j];
				if (l[j] == 0) n[1] *= 1 - p[j];
			}
			for (l[j--]++; j >= 0; l[j--] = 0) {
				for (h = 0; h < l[j]; h++) n[0] /= p[j];
				if (l[j] > 0) n[1] /= 1 - p[j];
			}
		}
	}
	
	long long ans = sum[0] * (sum[0] - 1) % 998244353;
	for (i = 2; i <= 1000000; i++) {
		if (coeff[i] != 0) ans = (ans + ((sum[i] * sum[i] % 998244353) * i % 998244353) * coeff[i] % 998244353 + 998244353) % 998244353;
	}
	
	printf("%lld\n", div_mod(ans, 2, 998244353));
	fflush(stdout);
	return 0;
}