#include <stdio.h>

const int Mod = 1000000007;

long long div_mod(long long x, long long y, long long z)
{
	if (x % y == 0) return x / y;
	else return (div_mod((1 + x / y) * y - x, (z % y), y) * z + x) / y;
}

int main()
{
	int i, N, A[100001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[i]));
	
	long long fact = 1, sum[100001], ans = 0;
	for (i = 1; i <= N; i++) fact = fact * i % Mod;
	for (i = 2, sum[1] = 0; i <= N; i++) sum[i] = (sum[i-1] + div_mod(fact, i, Mod)) % Mod;
	for (i = 1; i <= N; i++) ans = (ans + (sum[i] + sum[N-i+1] + fact) * A[i]) % Mod;
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}