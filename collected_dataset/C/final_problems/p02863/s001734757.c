#include <stdio.h>

typedef struct {
	int key, id;
} data;

void merge_sort(data x[], int n)
{
	static data y[3001] = {};
	if (n == 1) return;
	merge_sort(&(x[0]), n/2);
	merge_sort(&(x[n/2]), (n+1)/2);
	
	int i, p, q;
	for (i = 0, p = 0, q = n/2; i < n; i++) {
		if (p >= n/2) y[i] = x[q++];
		else if (q >= n) y[i] = x[p++];
		else y[i] = (x[p].key < x[q].key)? x[p++]: x[q++];
	}
	for (i = 0; i < n; i++) x[i] = y[i];
}

int main()
{
	int i, N, T, A[3001], B[3001];
	scanf("%d %d", &N, &T);
	for (i = 1; i <= N; i++) scanf("%d %d", &(A[i]), &(B[i]));
	
	data d[3001];
	for (i = 0; i < N; i++) {
		d[i].key = B[i+1];
		d[i].id = i + 1;
	}
	merge_sort(d, N);
	
	int j, dp[2][3001][2] = {};
	for (i = N - 1; i >= 0; i--) {
		for (j = 0; j < T; j++) dp[i%2][j][1] = (dp[1-i%2][j][1] > dp[1-i%2][j][0] + B[d[i].id])? dp[1-i%2][j][1]: dp[1-i%2][j][0] + B[d[i].id];
		for (j = 0; j < T - A[d[i].id]; j++) {
			dp[i%2][j+A[d[i].id]][0] = dp[1-i%2][j][0] + B[d[i].id];
			if (dp[i%2][j+A[d[i].id]][1] < dp[1-i%2][j][1] + B[d[i].id]) dp[i%2][j+A[d[i].id]][1] = dp[1-i%2][j][1] + B[d[i].id];
		}
	}
	
	printf("%d\n", dp[0][T-1][1]);
	fflush(stdout);
	return 0;
}