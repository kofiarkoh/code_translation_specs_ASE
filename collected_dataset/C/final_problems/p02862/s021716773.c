#include <stdio.h>
typedef long long ll;
#define MOD 1000000007
ll modpow(ll a, ll n) {
	if (n == 0)
		return 1;
	if (n % 2 == 1)
		return (a * modpow(a, n - 1) % MOD);
	ll t = modpow(a, n / 2);
	return (t * t % MOD);
}
ll comb(ll n, ll r) {
	if (n < r || n < 0 || r < 0)
		return 0;
	ll x = 1, y = 1;
	for (ll i = n; i > n - r; i--)
		x = x * i % MOD;
	for (ll i = 2; i <= r; i++)
		y = y * i % MOD;
	return (x * modpow(y, MOD - 2) % MOD);
}
int main() {
	int x, y;
	scanf("%d%d", &x, &y);
	if ((x + y) % 3 != 0) {
		printf("0\n");
		return 0;
	}
	int n = (x + y) / 3;
	int r = n * 2 - x;
	// printf("%d %d\n", n, r);
	printf("%lld\n", comb(n, r));
	return 0;
}