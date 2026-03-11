#include <stdio.h>

int main(void)
{
	int N, K, X, Y;

	scanf("%d%d%d%d", &N, &K, &X, &Y);
	if (K < N) {
		printf("%d\n", K * X + (N - K) * Y);
	} else {
		printf("%d\n", N * X);
	}

	return 0;
}