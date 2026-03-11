#include <stdio.h>

const int Mod = 1000000007;

long long div_mod(long long x, long long y, long long z)
{
	if (x % y == 0) return x / y;
	else return (div_mod((1 + x / y) * y - x, (z % y), y) * z + x) / y;
}

int main()
{
	int N, A, B, C;
	scanf("%d %d %d %d", &N, &A, &B, &C);
	
	int i;
	long long fact[200001], fact_inv[200001], pow[2][100001] = {{1, div_mod(A, A + B, Mod)}, {1, div_mod(B, A + B, Mod)}};
	for (i = 1, fact[0] = 1; i <= N * 2; i++) fact[i] = fact[i-1] * i % Mod;
	for (i = N * 2 - 1, fact_inv[N*2] = div_mod(1, fact[N*2], Mod); i >= 0; i--) fact_inv[i] = fact_inv[i+1] * (i + 1) % Mod;
	for (i = 2; i <= N; i++) {
		pow[0][i] = pow[0][i-1] * pow[0][1] % Mod;
		pow[1][i] = pow[1][i-1] * pow[1][1] % Mod;
	}
	
	long long prob[2], ans = 0;
	for (i = 0; i < N; i++) {
		prob[0] = pow[0][N] * pow[1][i] % Mod * fact[N+i-1] % Mod * fact_inv[N-1] % Mod * fact_inv[i] % Mod;
		prob[1] = pow[1][N] * pow[0][i] % Mod * fact[N+i-1] % Mod * fact_inv[N-1] % Mod * fact_inv[i] % Mod;
		ans = (ans + (prob[0] + prob[1]) * (N + i)) % Mod;
	}
	printf("%lld\n", div_mod(ans * 100 % Mod, A + B, Mod));
	fflush(stdout);
	return 0;
}