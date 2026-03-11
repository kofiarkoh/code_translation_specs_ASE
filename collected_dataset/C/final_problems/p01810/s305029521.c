// AOJ 2707: Jail
// 2017.10.9 bal4u@uu

#include <stdio.h>

int main()
{
	int n, k, k1, i;
	long long x;

	scanf("%d%d", &n, &k); k1 = k-1;
	for (x = 0, i = 1; i < n; i++)
		x = (x/k1)*k + x%k1 + 1;
	printf("%lld\n", x);
	return 0;
}