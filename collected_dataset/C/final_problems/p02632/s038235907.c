#include<stdio.h>
long long int fact[2000006];
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
long long int comb(long long int n, long long int r, long long int p)
{
	long long int res = fact[n];
	res = res * modpow(fact[r], p - 2, p) % p;
	res = res * modpow(fact[n - r], p - 2, p) % p;
	return res;
}
int main()
{
	long long int k;
	scanf("%lld", &k);
	char s[1000006];
	scanf("%s", s);
	long long int i, n = 0;
	while (s[n] != '\0')
		n++;
	long long int p = 1000000007;
	fact[0] = 1;
	for (i = 1; i < 2000006; i++)
		fact[i] = fact[i - 1] * i % p;
	long long int ans = 0;
	for (i = n; i <= n + k; i++)
		ans = (ans + comb(i - 1, n - 1, p) * modpow(25, i - n, p) % p * modpow(26, n + k - i, p) % p) % p;
	printf("%lld\n", ans);
	return 0;
}