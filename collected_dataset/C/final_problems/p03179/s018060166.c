#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p (int)(1e9 + 7)

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

signed main(){
	int N, i, j;
	scanf("%lld", &N);
	char *s = (char *)malloc(sizeof(char) * N);
	scanf("%s", s);
	int **dp = (int **)malloc(sizeof(int *) * N);
	int **sum = (int **)malloc(sizeof(int *) * N);
	for(i = 0; i < N; i++){
		dp[i] = (int *)malloc(sizeof(int) * (i + 1));
		sum[i] = (int *)malloc(sizeof(int) * (i + 1));
	}
	for(j = 0; j < N; j++){
		dp[N - 1][j] = 1;
		sum[N - 1][j] = j + 1;
	}
	for(i = N - 2; i >= 0; i--){
		for(j = 0; j <= i; j++){
			if(s[i] == '<'){
				dp[i][j] = MOD(sum[i + 1][i + 1] - sum[i + 1][j]);
			}
			else{
				dp[i][j] = sum[i + 1][j];
			}
		}
		sum[i][0] = dp[i][0];
		for(j = 1; j <= i; j++){
			sum[i][j] = MOD(sum[i][j - 1] + dp[i][j]);
		}
	}
	printf("%lld\n", dp[0][0]);
	return 0;
}