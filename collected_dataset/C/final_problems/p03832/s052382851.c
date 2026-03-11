#include <stdio.h>

const long long Mod = 1000000007;

long long div_mod(long long x, long long y, long long z)
{
	if (x % y == 0) return x / y;
	else return (div_mod((1 + x / y) * y - x, (z % y), y) * z + x) / y;
}

int main()
{
	int N, A, B, C, D;
	scanf("%d %d %d %d %d", &N, &A, &B, &C, &D);
	
	int i, j, k;
	long long dp[2][1001] = {{1}, {1}}, fact[1001], fact_inv[1001], tmp;
	for (i = 1, fact[0] = 1; i <= N; i++) fact[i] = fact[i-1] * i % Mod;
	for (i = N - 1, fact_inv[N] = div_mod(1, fact[N], Mod); i >= 0; i--) fact_inv[i] = fact_inv[i+1] * (i + 1) % Mod;
	for (i = A; i <= B; i++) {
		for (k = 1; k <= N; k++) dp[i%2][k] = dp[1-i%2][k];
		if (i * C > N) break;
		for (j = 1, tmp = 1; j < C; j++) tmp = tmp * fact[i*j] % Mod * fact_inv[i*(j-1)] % Mod * fact_inv[i] % Mod;
		for (; j <= D && i * j <= N; j++) {
			tmp = tmp * fact[i*j] % Mod * fact_inv[i*j-i] % Mod * fact_inv[i] % Mod;
			for (k = i * j; k <= N; k++) dp[i%2][k] = (dp[i%2][k] + dp[1-i%2][k-i*j] * fact[k] % Mod * fact_inv[k-i*j] % Mod * fact_inv[i*j] % Mod * tmp % Mod * fact_inv[j]) % Mod;
		}
	}
	
	printf("%lld\n", dp[1-i%2][N]);
	fflush(stdout);
	return 0;
}