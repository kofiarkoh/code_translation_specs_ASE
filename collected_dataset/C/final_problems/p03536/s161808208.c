#include <stdio.h>
#include <stdlib.h>
#define int long long
#define ms_valtype pair
#define inf (int)(1e17)

typedef struct {
	int H;
	int P;
}pair;

//比較関数
int compare_ms(ms_valtype a, ms_valtype b){
	return (a.H + a.P) - (b.H + b.P);
}

//昇順
void sort_sub(ms_valtype *origin, int left, int right, ms_valtype *tmp){
	if(right - left > 1){
		int i, j, k, half = (left + right) / 2;
		sort_sub(origin, left, half, tmp);
		sort_sub(origin, half, right, tmp);
		for(i = left; i < right; i++){
			tmp[i] = origin[i];
		}
		for(i = left, j = left, k = half; i < right; i++){
			if(k == right){
				origin[i] = tmp[j];
				j++;
			}
			else if(compare_ms(tmp[j], tmp[k]) <= 0 && j < half){
				origin[i] = tmp[j];
				j++;
			}
			else{
				origin[i] = tmp[k];
				k++;
			}
		}
	}
}

void sort(ms_valtype *origin, int N){
	ms_valtype *tmp = (ms_valtype *)malloc(sizeof(ms_valtype) * N);
	sort_sub(origin, 0, N, tmp);
	free(tmp);
}

int min(int a, int b){
	return a <= b ? a : b;
}

signed main(){
	int N, i, j;
	scanf("%lld", &N);
	pair *A = (pair *)malloc(sizeof(pair) * N);
	for(i = 0; i < N; i++){
		scanf("%lld%lld", &A[i].H, &A[i].P);
	}
	sort(A, N);
	int **dp = (int **)malloc(sizeof(int *) * (N + 1));
	for(i = 0; i <= N; i++){
		dp[i] = (int *)malloc(sizeof(int) * (N + 1));
		dp[i][0] = 0;
	}
	for(j = 1; j <= N; j++){
		dp[0][j] = inf;
	}
	for(i = 1; i <= N; i++){
		for(j = 1; j <= N; j++){
			dp[i][j] = dp[i - 1][j];
			if(dp[i - 1][j - 1] <= A[i - 1].H){
				dp[i][j] = min(dp[i][j], dp[i - 1][j - 1] + A[i - 1].P);
			}
		}
	}
	for(j = 1; j <= N; j++){
		if(dp[N][j] >= inf){
			printf("%lld\n", j - 1);
			return 0;
		}
	}
	printf("%lld\n", N);
	return 0;
}