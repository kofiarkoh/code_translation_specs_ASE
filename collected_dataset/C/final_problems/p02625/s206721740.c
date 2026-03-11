#include <stdio.h>

const int Mod = 1000000007;

long long div_mod(long long x, long long y, long long z)
{
	if (x % y == 0) return x / y;
	else return (div_mod((1 + x / y) * y - x, (z % y), y) * z + x) / y;
}

int main()
{
	int N, M;
	scanf("%d %d", &N, &M);
	
	int i;
	long long fact[500001], fact_inv[500001];
	for (i = 1, fact[0] = 1; i <= M; i++) fact[i] = fact[i-1] * i % Mod;
	for (i = M - 1, fact_inv[M] = div_mod(1, fact[M], Mod); i >= 0; i--) fact_inv[i] = fact_inv[i+1] * (i + 1) % Mod;
	
	long long ans = fact[M] * fact_inv[M-N] % Mod;
	for (i = 1; i <= N; i++) {
		if (i % 2 == 0) ans = (ans + fact[N] * fact_inv[i] % Mod * fact_inv[N-i] % Mod * fact[M-i] % Mod * fact_inv[M-N] % Mod + Mod) % Mod;
		else ans = (ans - fact[N] * fact_inv[i] % Mod * fact_inv[N-i] % Mod * fact[M-i] % Mod * fact_inv[M-N] % Mod + Mod) % Mod;
	}
	
	printf("%lld\n", ans * fact[M] % Mod * fact_inv[M-N] % Mod);
	fflush(stdout);
	return 0;
}