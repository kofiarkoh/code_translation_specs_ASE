#include <stdio.h>

#define MOD 1000000007
typedef long long ll;

int main(){
	int n, m;
	scanf("%d%d", &n, &m);

	ll a[112345];
	for(int i = 1; i < m; ++i) a[i] = 0;
	a[0] = a[m] = 1;

	ll ans = 0;
	for(int i = m; i <= n; ++i){
		ans = (ans * 2 + a[i]) % MOD;
		a[i + 1] = (a[i] * 2 - a[i - m] + MOD) % MOD;
	}

	printf("%lld\n", ans);
}

