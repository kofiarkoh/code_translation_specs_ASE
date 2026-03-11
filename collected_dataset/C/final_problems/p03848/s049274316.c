#include <stdio.h>
#include <stdlib.h>

int main()
{
	int i, N, A[100000];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[i]));
	
	int count[100001] = {}, p = (N + 1) % 2;
	for (i = 1; i <= N; i++) count[A[i]]++;
	for (i = 1; i <= N / 2; i++) if (count[i*2-p] != 2) break;
	if (i <= N / 2 || (p == 0 && count[0] != 1)) {
		printf("0\n");
		fflush(stdout);
		return 0;
	}

	long long ans = 1;
	for (i = 1; i <= N / 2; i++) ans = ans * 2 % 1000000007;
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}