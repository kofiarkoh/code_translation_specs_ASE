// AOJ DPL_5_A Balls and Boxes 1
// 2019.3.10 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007

int myPow(int x, int n)  // x^n
{
	long long ans = 1;
	long long xx = x;

	while (n > 0) {
		if (n & 1) ans = (ans * xx) % MOD, n--;
		else xx = (xx * xx) % MOD, n >>= 1;
	}
	return (int)ans;
}

int main()
{
	int n, k;

	scanf("%d%d", &n, &k);
	printf("%d\n", myPow(k, n));
	return 0;
}
