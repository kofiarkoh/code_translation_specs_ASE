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

int rank(int N, int M, int **a){
	int i, j, k, l;
	int **sub = (int **)malloc(sizeof(int *) * N);
	for(i = 0; i < N; i++){
		sub[i] = (int *)malloc(sizeof(int) * M);
		for(j = 0; j < M; j++){
			sub[i][j] = a[i][j];
		}
	}
	for(i = 0, j = 0; i < N && j < M; i++, j++){
//		printf("(i, j) = (%lld, %lld)\n", i, j);
		for(k = i; k < N; k++){
			if(sub[k][j] == 1){
				break;
			}
		}
		if(k == N){
			i--;
			continue;
		}
		int *tmp = sub[i];
		sub[i] = sub[k];
		sub[k] = tmp;
		for(k = 0; k < N; k++){
			if(i != k && sub[k][j] == 1){
				for(l = j; l < M; l++){
					sub[k][l] ^= sub[i][l];
				}
			}
		}
	}
	return i;
}

signed main(){
	int N, M, i, j;
	scanf("%lld%lld", &N, &M);
	int **a = (int **)malloc(sizeof(int *) * N);
	for(i = 0; i < N; i++){
		a[i] = (int *)malloc(sizeof(int) * N);
		for(j = 0; j < M; j++){
			scanf("%lld", &a[i][j]);
		}
	}
	printf("%lld\n", MOD((power(2, N) - power(2, N - rank(N, M, a))) * power(2, M - 1)));
	return 0;
}