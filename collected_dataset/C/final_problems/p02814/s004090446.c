/* Coached by rainboy */
#include <stdio.h>

#define N 100000

long long gcd(long long a, long long b) {
	return b == 0 ? a : gcd(b, a % b);
}

long long lcm(long long a, long long b) {
	return a * b / gcd(a, b);
}

int main() {
	static int aa[N];
	int n, m, i;
	long long x;

	scanf("%d%d", &n, &m);
	x = 1;
	for (i = 0; i < n; i++) {
		scanf("%d", &aa[i]);
		x = lcm(x, aa[i] / 2);
		if (x > m) {
			printf("0\n");
			return 0;
		}
	}
	for (i = 0; i < n; i++)
		if (x % aa[i] == 0) {
			printf("0\n");
			return 0;
		}
	printf("%lld\n", (m / x + 1) / 2);
	return 0;
}
