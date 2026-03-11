#include <stdio.h>

int main(void)
{
	long long int N, X;
	scanf("%lld%lld", &N, &X);

	if (N % X == 0 && X >= 2 && N % 2 == 0)
		printf("%lld\n", N / 2 * 3);
	else
		printf("%lld\n", (N - 1) * 3);

	return 0;
}