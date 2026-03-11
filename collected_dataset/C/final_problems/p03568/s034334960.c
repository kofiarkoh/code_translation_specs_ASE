#include <stdio.h>
#include <stdlib.h>

int main()
{
	int i, N, A[11];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[i]));
	
	int j;
	long long ans = 0, x;
	for (i = 1; i <= N; i++) {
		x = 1;
		for (j = 1; j < i; j++) x *= 2 - (A[j] % 2);
		for (j = i + 1; j <= N; j++) x *= 3;
		ans += x * (A[i] % 2 + 1);
	}
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}