#include <stdio.h>

int main()
{
	int i, N, x[100001], y[100001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d %d", &(x[i]), &(y[i]));

	int n[2] = {}, count[2][100001] = {};
	for (i = 1; i <= N; i++) {
		if (count[0][x[i]] == 0) n[0]++;
		if (count[1][y[i]] == 0) n[1]++;
		count[0][x[i]]++;
		count[1][y[i]]++;
	}
	
	long long ans = (long long)n[0] * n[1] - N;
	for (i = 1; i <= N; i++) {
		if (count[0][x[i]] == 1 && count[1][y[i]] == 1) {
			ans -= n[0] + n[1] - 2;
			n[0]--;
			n[1]--;
		}
	}	
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}