// AOJ 0394: Pilling Blocks
// 2019.2.23 bal4u

#include <stdio.h>
#include <stdlib.h>

int main()
{
	int N, K, ans, sum, len;

	scanf("%d%d", &N, &K);
	ans = sum = K + 1;
	if (N <= sum) {
		printf("%d\n", N);
		return 0;
	}

	len = 2; while (1) {
		while (sum > len*K) len++;
		sum += len;
		if (sum > N) break;
		ans++;
	}
	printf("%d\n", ans);
	return 0;
}
