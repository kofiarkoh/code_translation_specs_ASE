#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p (int)(1e9 + 7)

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

signed main(){
	int N, K, i, j;
	scanf("%lld%lld", &N, &K);
	int *a = (int *)malloc(sizeof(int) * (N + 1));
	for(i = 1; i <= N; i++){
		scanf("%lld", &a[i]);
	}
	int **dp = (int **)malloc(sizeof(int *) * (N + 1));
	int **s = (int **)malloc(sizeof(int *) * (N + 1));
	for(i = 0; i <= N; i++){
		dp[i] = (int *)malloc(sizeof(int) * (K + 1));
		s[i] = (int *)malloc(sizeof(int) * (K + 1));
		for(j = 0; j <= K; j++){
			dp[i][j] = 0;
			s[i][j] = 0;
		}
	}
	dp[0][0] = 1;
	for(j = 0; j <= K; j++){
		s[0][j] = 1;
	}
	for(i = 1; i <= N; i++){
		for(j = 0; j <= K; j++){
			if(j <= a[i]){
				dp[i][j] = s[i - 1][j];
			}
			else{
				dp[i][j] = MOD(s[i - 1][j] - s[i - 1][j - a[i] - 1]);
			}
		}
		s[i][0] = dp[i][0];
		for(j = 1; j <= K; j++){
			s[i][j] = MOD(s[i][j - 1] + dp[i][j]);
		}
	}
	printf("%lld\n", dp[N][K]);
	return 0;
}