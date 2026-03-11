#include <stdio.h>
int a[7], b[7];
int dp[1000];
void solve(int N) {
	int i, j, k;
	for(i = 0; i < 1000; ++ i) dp[i] = 0;
	dp[0] = 1;
	for(i = 0; i < N; ++ i) {
		for(j = 999; j > -1 ; -- j) {
			for(k = 1; k <= b[i]; ++ k) {
				if(j - a[i] * k > -1) {
					dp[j] += dp[j - a[i] * k];
				}
			}
		}
	}
}
int main() {
	int N, M, I, g;
	while(1) {
		scanf("%d", &N);
		if(N == 0) break;
		for(I = 0; I < N; ++ I) scanf("%d %d", &a[I], &b[I]);
		solve(N);
		scanf("%d", &M);
		for(I = 0; I < M; ++ I) {
			scanf("%d", &g);
			printf("%d\n", dp[g]);
		}
	}
	return 0;
}