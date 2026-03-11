#include <stdio.h>

int main()
{
	int i, N, A[200001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[i]));
	for (i = 1; i < N; i++) if (A[i] >= A[i+1]) break;
	if (i == N) {
		printf("1\n");
		fflush(stdout);
		return 0;
	}
	
	int j, l = 2, r = N, m, count[51];
	while (l < r) {
		m = (l + r) / 2;
		if (A[1] <= 50) {
			for (i = 1; i <= A[1]; i++) count[i] = 1;
			for (; i <= 50; i++) count[i] = 0;
		} else for (i = 1; i <= 50; i++) count[i] = 1;
		for (i = 2; i <= N; i++) {
			if (A[i] > 50) continue;
			for (j = A[i]; j >= 1 && count[j] == m; j--);
			if (j == 0) break;
			for (count[j++]++; j <= A[i]; count[j++] = 1);
			for (; j <= 50; count[j++] = 0);
		}
		if (i <= N) l = m + 1;
		else r = m;
	}
	printf("%d\n", l);
	fflush(stdout);
	return 0;
}