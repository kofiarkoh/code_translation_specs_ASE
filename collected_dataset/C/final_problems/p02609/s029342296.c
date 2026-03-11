#include <stdbool.h>
#include <stdio.h>
typedef long long ll;
ll modpow(ll a, ll n, ll MOD) {
	if (n == 0)
		return 1;
	if (n % 2 == 1)
		return (a * modpow(a, n - 1, MOD) % MOD);
	ll t = modpow(a, n / 2, MOD);
	return (t * t % MOD);
}
ll popCnt(ll num) {
	int res = 0;
	for (int i = 0; i < 64; i++) {
		if (num >> i & 1) {
			res++;
		}
	}
	return res;
}
int main() {
	int n;
	int cnt = 0;
	char x[200001];
	scanf("%d%s", &n, x);
	for (int i = 0; i < n; i++) {
		x[i] -= '0';
		if (x[i] == 1) {
			cnt++;
		}
	}
	ll mx = 0, px = 0;
	bool is0 = false;
	for (int i = 0; i < n; i++) {
		px += x[i] * modpow(2, n - i - 1, cnt + 1) % (cnt + 1);
		if (cnt == 1) {
			is0 = true;
		} else {
			mx += x[i] * modpow(2, n - i - 1, cnt - 1) % (cnt - 1);
		}
	}
	for (int i = 0; i < n; i++) {
		ll now = 0;
		if (x[i] == 1) {
			if (is0) {
				printf("0\n");
				continue;
			}
			now = mx;
			now += (cnt - 1);
			now -= modpow(2, n - i - 1, cnt - 1);
			now %= (cnt - 1);
		} else {
			now = px;
			now += modpow(2, n - i - 1, cnt + 1);
			now %= (cnt + 1);
		}
		int ans = 1;
		while (popCnt(now) != 0) {
			// printf("x:%d", num);
			int c = popCnt(now);
			// printf(" cnt:%d\n", c);
			now = now % c;
			ans++;
		}
		printf("%d\n", ans);
	}
	return 0;
}