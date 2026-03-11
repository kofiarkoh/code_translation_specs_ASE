#include<stdio.h>
#include<string.h>
long long int modpow(long long int n, long long int a, long long int p)
{
	if (a < 0)
		return 0;
	long long int res = 1;
	while (a > 0)
	{
		if (a % 2 == 1)
			res = (res * n) % p;
		n = (n * n) % p;
		a /= 2;
	}
	return res;
}
int main()
{
	char s[100005];
	scanf("%s", s);
	long long int i, n = strlen(s);
	long long int p = 1000000007;
	long long int counta[1005], countaa[1005], countc[1005], countcc[1005];
	counta[0] = countaa[0] = 0;
	for (i = 0; i < n; i++)
	{
		counta[i + 1] = counta[i];
		countaa[i + 1] = countaa[i];
		if (s[i] == 'A')counta[i + 1]++;
		if (s[i] == '?')countaa[i + 1]++;
	}
	countc[n] = countcc[n] = 0;
	for (i = n - 1; i >= 0; i--)
	{
		countc[i] = countc[i + 1];
		countcc[i] = countcc[i + 1];
		if (s[i] == 'C')countc[i]++;
		if (s[i] == '?')countcc[i]++;
	}
	long long int ans = 0;
	for (i = 1; i < n - 1; i++)
		if (s[i] == 'B' || s[i] == '?')
			ans = (ans + (counta[i] * modpow(3, countaa[i], p) + countaa[i] * modpow(3, countaa[i] - 1, p)) * (countc[i + 1] * modpow(3, countcc[i + 1], p) + countcc[i + 1] * modpow(3, countcc[i + 1] - 1, p))) % p;
	printf("%lld\n", ans);
	return 0;
}