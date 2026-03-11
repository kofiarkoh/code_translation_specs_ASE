#include <stdio.h>

long long n;
long long h[100010];
long long dp[100010];

long long chmin(long long a, long long b)
{
	if (b > a)
		return a;
	else 
		return b;
}

long long ab(long long x)
{
	if (x < 0)
		return -x;
	return x;
}

int main(int argc, char const *argv[])
{
	scanf("%lld", &n);
	for (int i = 0; i < n; i++)
	{
		dp[i] = 99999999;
		scanf("%lld", &h[i]);
	}
	dp[0] = 0;

	for (int i = 0; i < n; i++)
	{
		if(i+1 < n) dp[i + 1]=chmin(dp[i] + ab(h[i + 1] - h[i]), dp[i + 1]);
		if(i+2 < n) dp[i + 2]=chmin(dp[i] + ab(h[i + 2] - h[i]), dp[i + 2]);
	}

	printf("%lld\n", dp[n - 1]);
	return 0;
}
