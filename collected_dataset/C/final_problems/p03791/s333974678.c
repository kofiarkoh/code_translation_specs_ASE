#include <stdio.h>

#define Mod 1000000007

int main()
{
	int i, N, x[100001];
	scanf("%d", &N);
	for (i = 1, x[0] = -1; i <= N; i++) {
		scanf("%d", &(x[i]));
		if (x[i] > x[i-1] + 2) x[i] = x[i-1] + 2;
	}
	
	int k;
	long long ans = 1;
	for (i = 1; i < N && x[i] == x[i-1] + 2; i++);
	for (k = 0; k < N; k++) {
		ans = ans * (i - k) % Mod;
		if (i < N) {
			for (x[i] = x[i-1], i++; i < N && x[i] >= x[i-1] + 2; i++) x[i] = x[i-1] + 2;
		}
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}
