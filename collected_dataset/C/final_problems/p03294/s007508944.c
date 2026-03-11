#include <stdio.h>

int gcd(long long a, long long b) {
	long long r;
	while ((r = a % b) != 0) {
		a = b;
		b = r;
	}
	return b;
}
int lcm(long long a, long long b) {
	return a * b / gcd(a,b);
}

int main(void) {
	long long i, n, a[3000], ans = 1, max = 0, sum = 0;
	
	scanf("%lld", &n);
	for (i = 0; i < n; i++) {
		scanf("%lld", &a[i]);
		ans = lcm(ans, a[i]);
		sum += a[i];
	}
	
	for (i = 0; i < n; i++) {
		max += (ans - 1) % a[i];
	}
	
	printf("%lld", (max > sum-n ? max : sum-n));
	
	return 0;
}