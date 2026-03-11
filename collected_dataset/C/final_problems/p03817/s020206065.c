#include <stdio.h>

int main(void) {
	long long int x, ans;
	int rest;
	scanf("%lld", &x);

	ans = (x / 11) * 2;
	rest = x % 11;
	if (rest > 6) {
		ans += 2;
	}
	else {
		ans++;
	}

	printf("%lld", ans);

	return 0;
}