#include <stdio.h>
#include <math.h>
int dp[31];
void solve() {
	int i;
	for(i = 0; i < 31; ++ i) dp[i] = 0;
	dp[0] = 1;
	for(i = 0; i < 31; ++ i) {
		if(i + 1 < 31) dp[i + 1] += dp[i];
		if(i + 2 < 31) dp[i + 2] += dp[i];
		if(i + 3 < 31) dp[i + 3] += dp[i];
	}
}
int main() {
	int N;
	solve();
	while(1) {
		scanf("%d", &N);
		if(N == 0) break;
		printf("%d\n", (int) ceil((double) dp[N] / 3650));
	}
}