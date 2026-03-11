#include <stdio.h>

int main()
{
	int i, N, M, X[100001];
	scanf("%d %d", &N, &M);
	for (i = 1; i <= N; i++) scanf("%d", &(X[i]));

	int tmp, ans = 0, count[2][100001] = {};
	for (i = 1; i <= N; i++) {
		count[0][X[i]]++;
		count[1][X[i]%M]++;
	}
	ans = count[1][0] / 2;
	count[1][0] %= 2;
	if (M % 2 == 0) {
		ans += count[1][M/2] / 2;
		count[1][M/2] %= 2;
	}
	for (i = 1; i < (M + 1) / 2; i++) {
		tmp = (count[1][i] < count[1][M-i])? count[1][i]: count[1][M-i];
		ans += tmp;
		count[1][i] -= tmp;
		count[1][M-i] -= tmp;
	}
	for (i = 1; i <= 100000; i++) {
		if (count[0][i] >= 2 && count[1][i%M] >= 2) {
			tmp = (count[0][i] < count[1][i%M])? count[0][i]: count[1][i%M];
			ans += tmp / 2;
			count[1][i%M] -= tmp / 2 * 2;
		}
	}

	printf("%d\n", ans);
	fflush(stdout);
	return 0;
}