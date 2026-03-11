#include <stdio.h>
typedef long long ll;
int main() {
	int k;
	scanf("%d", &k);
	if (k % 2 == 0 || k % 5 == 0) {
		printf("-1\n");
		return 0;
	}
	ll n = 7;
	int ans = 1;
	while (n % k != 0) {
		n = n * 10 + 7;
		n %= k;
		ans++;
	}
	printf("%d\n", ans);
	return 0;
}
