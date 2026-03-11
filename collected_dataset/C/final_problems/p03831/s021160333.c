#include <stdio.h>
typedef long long ll;
int main() {
	int n, a, b;
	int x[100000];
	ll ans = 0;
	scanf("%d%d%d", &n, &a, &b);
	for (int i = 0; i < n; i++) {
		scanf("%d", &x[i]);
	}
	for (int i = 0; i < n - 1; i++) {
		if (a * (x[i + 1] - x[i]) >= b) {
			ans += b;
		} else {
			ans += a * (x[i + 1] - x[i]);
		}
	}
	printf("%lld\n", ans);
	return 0;
}