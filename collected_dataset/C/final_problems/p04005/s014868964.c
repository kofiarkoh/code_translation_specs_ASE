#include <stdio.h>
typedef long long ll;
ll min(ll x, ll y) {
	return x < y ? x : y;
}
int main() {
	ll a, b, c;
	scanf("%lld%lld%lld", &a, &b, &c);
	if (a % 2 == 0 || b % 2 == 0 || c % 2 == 0) {
		printf("0\n");
	} else {
		printf("%lld\n", min(a * b, min(b * c, c * a)));
	}
	return 0;
}