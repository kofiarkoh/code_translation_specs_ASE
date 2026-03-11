#include <stdio.h>
#include <stdlib.h>

long long int *sort_sub(long long int *T, int N){
	long long int *ans = (long long int *)malloc(sizeof(long long int) * N);
	if(N == 1){
		ans[0] = T[0];
	}
	else{
		int N1 = N / 2;
		int N2 = (N + 1) / 2;
		long long int *sub1 = sort_sub(&T[0] , N1);
		long long int *sub2 = sort_sub(&T[N1], N2);
		int i, j, k;
		for(i = 0, j = 0, k = 0; i < N; i++){
			if((sub1[j] < sub2[k] && j != N1) || (k == N2)){
				ans[i] = sub1[j];
				j++;
			}
			else{
				ans[i] = sub2[k];
				k++;
			}
		}
		free(sub1);
		free(sub2);
	}
	return ans;
}

long long int *sort(long long int *T, int N){
	long long int *ans = sort_sub(T, N);
	free(T);
	return ans;
}

long long int mod(long long int a, long long int p){
	if(a < 0){
		return p + a % p;
	}
	else{
		return a % p;
	}
}

int main(){
	 long long int N, L, T, i, j;
	scanf("%lld%lld%lld", &N, &L, &T);
	long long int *X = (long long int *)malloc(sizeof(long long int) * N);
	int *W = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%lld %d", &X[i], &W[i]);
	}
	for(i = 0; i < N; i++){
		if(W[i] == 1){
			break;
		}
	}
	if(i == N){
		for(i = 0; i < N; i++){
			printf("%lld\n", mod(X[i] - T, L));
		}
	}
	else{
		int count = 0, piv = i;
		for(i = 0; i < N; i++){
			if(W[i] == 2){
				count++;
			}
		}
		long long int enc = ((2 * T + X[piv]) / L) * count;
		count = 0;
		for(i = 0; X[i] < (2 * T + X[piv]) % L; i++){
			if(W[i] == 2){
				count++;
			}
		}
		enc += count;
		count = 0;
		for(i = 0; i < piv; i++){
			if(W[i] == 2){
				count++;
			}
		}
		enc += count;
		for(i = 0; i < N; i++){
			if(W[i] == 1){
				X[i] = mod(X[i] + T, L);
			}
			else{
				X[i] = mod(X[i] - T, L);
			}
		}
		long long int Xpiv = X[piv];
		X = sort(X, N);
		for(i = 0;; i++){
			if(X[i] == Xpiv){
				break;
			}
		}
		for(j = 0; j < N; j++){
			printf("%lld\n", X[mod(i - enc + piv + j, N)]);
		}
	}
	return 0;
}