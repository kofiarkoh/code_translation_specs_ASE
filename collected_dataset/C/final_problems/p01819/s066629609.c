#include <stdio.h>

int n, m;
char d[128][12345];

int eSumLeft[12345], wSumRight[12345];

int main(void) {
	int i, j;
	int score = 999999999;
	int answer = 0;
	if (scanf("%d%d", &n, &m) != 2) return 1;
	for (i = 0; i < m; i++) {
		if (scanf("%12344s", d[i] + 1) != 1) return 1;
	}
	for (i = 1; i <= n; i++) {
		for (j = 0; j < m; j++) {
			if (d[j][i] == 'E') eSumLeft[i]++;
			if (d[j][i] == 'W') wSumRight[i]++;
		}
	}
	for (i = 1; i <= n; i++) eSumLeft[i] += eSumLeft[i - 1];
	for (i = n - 1; i >= 0; i--) wSumRight[i] += wSumRight[i + 1];
	for (i = 1; i <= n + 1; i++) {
		int candidate = eSumLeft[i - 1] + wSumRight[i];
		if (candidate < score) {
			score = candidate;
			answer = i;
		}
	}
	printf("%d %d\n", answer - 1, answer);
	return 0;
}

