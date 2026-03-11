#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p 998244353

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

int power(int a, int N){
	if(N == 0){
		return 1;
	}
	else if(N % 2 == 0){
		return power(MOD(a * a), N / 2);
	}
	else{
		return MOD(a * power(a, N - 1));
	}
}

int inverse(int a){
	return power(a, p - 2);
}

signed main(){
	int N, i, j;
	char *S = (char *)malloc(sizeof(char) * 2002);
	scanf("%s", &S[1]);
	for(N = 0; S[N + 1] != '\0'; N++);
	int *R = (int *)malloc(sizeof(int) * (N + 1));
	int *B = (int *)malloc(sizeof(int) * (N + 1));
	R[0] = 0;
	B[0] = 0;
	for(i = 1; i <= N; i++){
		R[i] = R[i - 1];
		B[i] = B[i - 1];
		if(S[i] == '0'){
			R[i] += 2;
		}
		else if(S[i] == '1'){
			R[i] += 1;
			B[i] += 1;
		}
		else{
			B[i] += 2;
		}
	}
	int **dp = (int **)malloc(sizeof(int *) * (2 * N + 1));
	for(i = 0; i <= 2 * N; i++){
		dp[i] = (int *)malloc(sizeof(int) * (2 * N + 1));
		for(j = 0; j <= 2 * N; j++){
			dp[i][j] = 0;
		}
	}
	dp[0][0] = 1;
	for(i = 1; i <= 2 * N; i++){
		for(j = 0; j <= i; j++){
			if(i >= N || (j <= R[i] && i - j <= B[i])){
				dp[i][j] = MOD((j > 0 ? dp[i - 1][j - 1] : 0) + dp[i - 1][j]);
			}
		}
	}
	printf("%lld\n", dp[2 * N][R[N]]);
	return 0;
}