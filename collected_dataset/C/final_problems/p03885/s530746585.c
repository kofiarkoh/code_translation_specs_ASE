#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p (int)(1e9 + 7)

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

int rank(int **A, int N){
	int i, j, k, l, *tmp;
	for(i = 0, j = 0; j < N; j++){
//		printf("(i, j) = (%lld, %lld)\n", i, j);
		for(k = i; k < N && A[k][j] == 0; k++);
//		printf("k = %lld\n", k);
		if(k == N){
			continue;
		}
		tmp = A[i];
		A[i] = A[k];
		A[k] = tmp;
		for(k = 0; k < N; k++){
			if(i == k || A[k][j] == 0){
				continue;
			}
			for(l = j; l < N; l++){
				A[k][l] ^= A[i][l];
			}
		}
		i++;
	}
	return i;
}

signed main(){
	int N, R, i, j, k;
	scanf("%lld", &N);
	int **C = (int **)malloc(sizeof(int *) * N);
	for(i = 0; i < N; i++){
		C[i] = (int *)malloc(sizeof(int) * N);
		for(j = 0; j < N; j++){
			scanf("%lld", &C[i][j]);
		}
	}
	R = rank(C, N);
//	printf("R = %lld\n", R);

	int *pow2 = (int *)malloc(sizeof(int) * (N * N + 1));
	pow2[0] = 1;
	for(i = 1; i <= N * N; i++){
		pow2[i] = MOD(pow2[i - 1] * 2);
	}

	int ***dp = (int ***)malloc(sizeof(int **) * (N + 1));
	for(i = 0; i <= N; i++){
		dp[i] = (int **)malloc(sizeof(int *) * (N + 2));
		for(j = 0; j <= N + 1; j++){
			dp[i][j] = (int *)malloc(sizeof(int) * (N + 2));
			for(k = 0; k <= N + 1; k++){
				dp[i][j][k] = 0;
			}
		}
	}
	dp[0][0][0] = 1;
	for(i = 0; i < N; i++){
		for(j = 0; j <= N; j++){
			for(k = 0; k <= N; k++){
				if(k > R || j < k){
					dp[i][j][k] = 0;
				}
				else{
					dp[i + 1][j][k] = MOD(dp[i + 1][j][k] + pow2[j] * dp[i][j][k]);
					dp[i + 1][j + 1][k] = MOD(dp[i + 1][j + 1][k] + MOD(pow2[N] - pow2[R - k + j]) * dp[i][j][k]);
					dp[i + 1][j + 1][k + 1] = MOD(dp[i + 1][j + 1][k + 1] + MOD(pow2[R - k + j] - pow2[j]) * dp[i][j][k]);
				}
			}
		}
	}
/*	for(i = 0; i <= N; i++){
		for(j = 0; j <= N; j++){
			for(k = 0; k <= N; k++){
				printf("dp[%lld][%lld][%lld] = %lld\n", i, j, k, dp[i][j][k]);
			}
		}
		printf("\n");
	}
*/	int ans = 0;
	for(j = 0; j <= N; j++){
		ans = MOD(ans + pow2[N * (N - j)] * dp[N][j][R]);
	}
	printf("%lld\n", ans);
	return 0;
}