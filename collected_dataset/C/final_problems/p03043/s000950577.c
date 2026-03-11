#include <stdio.h>

int main(void) {
	int N, K;
	scanf("%d%d", &N, &K);

	int i, j, cnt = 0, memo = 0;
	long double ans1, ans2 = 0;
	for (i = 1; i <= N; i++) {
		cnt = 0;
		memo = i;
		ans1 = 1.0;
		for (j = 1; memo < K; j++) {
			memo *= 2;
			cnt++;
		}
		for (j = 0; j < cnt; j++) {
			ans1 *= 0.5;
		}
		ans2 += ans1 * (1.0 / N);
	}
	printf("%.12lf\n", ans2);

	return 0;
}