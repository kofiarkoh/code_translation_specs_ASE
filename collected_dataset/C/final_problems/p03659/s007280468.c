#include <stdio.h>
#include <stdlib.h>

int main()
{
	int i, N;
	long long a[200001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%lld", &(a[i]));
	
	long long sum[200001] = {}, min;
	for (i = 1; i <= N; i++) sum[i] = sum[i-1] + a[i];
	for (i = 2, min = llabs(sum[N] - sum[1] * 2); i < N; i++) {
		if (llabs(sum[N] - sum[i] * 2) < min) min = llabs(sum[N] - sum[i] * 2);
	}
	
	printf("%lld\n", min);
	fflush(stdout);
	return 0;
}