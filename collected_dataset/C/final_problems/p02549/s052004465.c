#include <stdio.h>
#include <math.h>
#include <string.h>

typedef long long ll;

int n, k;
int in_s[200005];
ll ans[200005];
ll mod = 998244353;

ll getans(int x)
{
	if (x <= 0)
	{
		return 0;
	}
	if (ans[x] != -1)
	{
		return ans[x];
	}
	ll s = 0;
	for (int i = 0; i < n; i++)
	{
		if (in_s[i] == 0)
		{
			continue;
		}
		s += getans(x - i);
		s %= mod;
	}
	ans[x] = s;
	return s;
	
}

int main() {
	scanf("%d %d", &n, &k);
	for (int i = 0; i < k; i++)
	{
		int l, r;
		scanf("%d %d", &l, &r);
		for (int i = l; i <= r; i++)
		{
			in_s[i] = 1;
		}
		
	}
	for (int i = 0; i <= n; i++)
	{
		ans[i] = -1;
	}
	ans[1] = 1;
	printf("%lld", getans(n));
	
	
	
	return 0;
}
