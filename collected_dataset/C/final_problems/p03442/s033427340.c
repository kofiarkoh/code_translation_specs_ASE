#include <stdio.h>
#include <stdlib.h>
#define inf (int)(1e9)

int min(int a, int b){
	return a <= b ? a : b;
}

int main(){
	int N, x, y, a, i, j, K = 16, ans = 0, remain_num, xor, i_num;
	scanf("%d", &N);
	int *val = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		val[i] = 0;
	}
	for(i = 1; i < N; i++){
		scanf("%d%d%d", &x, &y, &a);
		val[x] ^= a;
		val[y] ^= a;
	}
	int *remain = (int *)malloc(sizeof(int) * K);
	for(i = 0; i < K; i++){
		remain[i] = 0;
	}
	for(i = 0; i < N; i++){
		if(val[i] > 0 && remain[val[i]] == 1){
			ans++;
		}
		remain[val[i]] ^= 1;
	}
	remain[0] = 0;
	remain_num = 0;
	for(i = 0; i < K; i++){
		remain_num ^= (remain[i] << i);
	}
//	printf("ans = %d\n", ans);
//	printf("remain_num = %d\n", remain_num);
	int *dp = (int *)malloc(sizeof(int) * (1 << K));
	dp[0] = 0;
	for(i = 0; i < (1 << K); i++){
//		printf("dp[%d] = %d\n", i, dp[i]);
		xor = 0;
		i_num = 0;
		for(j = 0; j < K; j++){
			if((i & (1 << j)) != 0){
//				printf("j = %d\n", j);
				xor ^= j;
				i_num++;
			}
		}
		if(xor != 0){
			dp[i] = inf;
		}
		else{
			dp[i] = i_num - 1;
//			printf("dp[%d] = %d\n", i, dp[i]);
			for(j = i; j > 0; j = ((j - 1) & i)){
				dp[i] = min(dp[i], dp[j] + dp[i ^ j]);
			}
		}
		if(i == 0){
			dp[i] = 0;
		}
//		printf("dp[%d] = %d\n", i, dp[i]);
		if(i == remain_num){
			printf("%d\n", ans + dp[remain_num]);
			return 0;
		}
	}
	return 0;
}