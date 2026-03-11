#include <stdio.h>
#include <stdlib.h>
#define int long long
#define inf (int)(1e18)

typedef struct {
	int L;
	int R;
}pair;

signed compair(const void *a, const void *b){
	int sub = (((pair *)a)->L + ((pair *)a)->R) - (((pair *)b)->L + ((pair *)b)->R);
	if(sub > 0){
		return -1;
	}
	else if(sub == 0){
		return 0;
	}
	else{
		return 1;
	}
}

int min(int a, int b){
	return a <= b ? a : b;
}

signed main(){
	int N, i, j, k, l;
	scanf("%lld", &N);
	pair *ps = (pair *)malloc(sizeof(pair) * N);
	for(i = 0; i < N; i++){
		scanf("%lld%lld", &ps[i].L, &ps[i].R);
	}
	qsort(ps, N, sizeof(pair), compair);
	int ***dp = (int ***)malloc(sizeof(int **) * (N / 2 + 2));
	for(i = 0; i <= N / 2 + 1; i++){
		dp[i] = (int **)malloc(sizeof(int *) * (N / 2 + 2));
		for(j = 0; j <= N / 2 + 1; j++){
			dp[i][j] = (int *)malloc(sizeof(int) * (N % 2 + 1));
			for(k = 0; k <= N % 2; k++){
				dp[i][j][k] = inf;
			}
		}
	}
	dp[0][0][0] = 0;
	for(i = 0; i <= N / 2; i++){
		for(j = 0; j <= N / 2; j++){
			for(k = 0; k <= N % 2; k++){
				l = i + j + k;
				if(l >= N){
					continue;
				}
				dp[i + 1][j][k] = min(dp[i + 1][j][k], dp[i][j][k] + i * (ps[l].L + ps[l].R) + ps[l].R);
				dp[i][j + 1][k] = min(dp[i][j + 1][k], dp[i][j][k] + j * (ps[l].L + ps[l].R) + ps[l].L);
				if(k == 0 && N % 2 == 1 && l < N){
					dp[i][j][1] = min(dp[i][j][1], dp[i][j][0] + (N / 2) * (ps[l].L + ps[l].R));
				}
			}
		}
	}
/*	for(i = 0; i < N; i++){
		for(j = 0; j < N; j++){
			for(k = 0; k <= N % 2; k++){
				printf("dp[%lld][%lld][%lld] = %lld\n", i, j, k, dp[i][j][k]);
			}
		}
	}
*/	printf("%lld\n", dp[N / 2][N / 2][N % 2]);
	return 0;
}