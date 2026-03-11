#include <stdio.h>
#include <string.h>

#define N 10

int main() {
	static char s[N + 1];
	int n, n_, i, j;
	long long ans, x;

	scanf("%s", s);
	n = strlen(s);
	ans = 0;
	for (i = 0; i < n; i++) {
		x = 0;
		for (j = i; j < n; j++) {
			x = x * 10 + s[j] - '0';
			n_ = n - 1 - (j - i);
			if (i > 0)
				n_--;
			if (j + 1 < n)
				n_--;
			ans += x << n_;
		}
	}
	printf("%lld\n", ans);
	return 0;
}
