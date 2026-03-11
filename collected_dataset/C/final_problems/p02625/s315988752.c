#include<stdio.h>
long long int modpow(long long int a, long long int n, long long int p)
{
	long long int res = 1;
	while (n > 0)
	{
		if (n % 2 > 0)
			res = res * a % p;
		a = a * a % p;
		n /= 2;
	}
	return res;
}
long long int fact[500005];
long long int comb(long long int n, long long int r, long long int p)
{
	long long int res = fact[n];
	res = res * modpow(fact[r], p - 2, p) % p;
	res = res * modpow(fact[n - r], p - 2, p) % p;
	return res;
}
int main()
{
	long long int n, m;
	scanf("%lld %lld", &n, &m);
	long long int p = 1000000007;
	long long int i;
	fact[0] = 1;
	for (i = 1; i <= m; i++)
		fact[i] = fact[i - 1] * i % p;
	long long int ans = fact[m] * modpow(fact[m - n], p - 2, p) % p;
	long long int s;
	for (i = 0; i < n; i++)
	{
		s = comb(n, i + 1, p);
		s = s * fact[m - i - 1] % p * modpow(fact[m - n], p - 2, p) % p;
		if (i % 2 > 0)
			ans = (ans + s) % p;
		else
			ans = (ans - s + p) % p;
	}
	s = fact[m] * modpow(fact[m - n], p - 2, p) % p;
	ans = ans * s % p;
	printf("%lld\n", ans);
	return 0;
}