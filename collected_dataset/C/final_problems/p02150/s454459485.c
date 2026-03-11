// AOJ 3052 Milk
// 2018.3.17 bal4u

#include <stdio.h>
#include <stdlib.h>

#define M 1000000007

int main()
{
	long long a, b, x, c, ans;

	scanf("%lld%lld%lld", &a, &b, &x);
	ans = x;
	if (x > b) {
		c = (x - b) / (a - b);
		ans += ((b%M) * (c%M)) % M;
	}
	printf("%d\n", (int)(ans % M));
	return 0;
}
