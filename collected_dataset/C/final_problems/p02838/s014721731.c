#include <stdio.h>

int main()
{
	int i, N;
	long long A[300001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%lld", &(A[i]));
	
	int j;
	long long n, ans = 0, count[61][2] = {};
	for (i = 0, n = 1; i < 61; i++, n *= 2) {
		for (j = 1; j <= N; j++) {
			count[i][A[j]%2]++;
			A[j] /= 2;
		}
		ans = (ans + n * count[i][0] * count[i][1]) % 1000000007;
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}