#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p 998244353

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

int fact(int N){
	if(N == 0){
		return 1;
	}
	else{
		return MOD(N * fact(N - 1));
	}
}

signed main(){
	int N, i, j, k, f0 = 0;
	scanf("%lld", &N);
	int *A = (int *)malloc(sizeof(int) * 2 * N);
	int *L = (int *)malloc(sizeof(int) * N);
	int *R = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%lld", &A[i]);
		A[i]--;
		if(A[i] == 0){
			f0 = 1;
		}
		A[N + i] = A[i];
		L[i] = -1;
		R[i] = -1;
	}
	if(f0 == 0){
		printf("0\n");
		return 0;
	}
	for(i = 0; i < N && A[i] == 0; i++);
	if(i == N){
		printf("%lld\n", fact(N));
		return 0;
	}
	for(i++; i < 2 * N && A[i] > 0; i++);
	for(j = 0; j < N; j++){
		A[j] = A[i + j];
	}
/*	printf("A:");
	for(i = 0; i < N; i++){
		printf("%lld ", A[i]);
	}
	printf("\n");
*/	for(i = 0; A[i] == 0; i++);
	k = i;
//	printf("k = %lld\n", k);
	for(i = i; i < N; i++){
		if(A[i] == 0){
			printf("0\n");
			return 0;
		}
		else if(L[A[i]] == -1){
			L[A[i]] = i;
			R[A[i]] = i;
		}
		else if(A[i] == A[i - 1]){
			R[A[i]]++;
		}
		else{
			printf("0\n");
			return 0;
		}
	}
	int *B = (int *)malloc(sizeof(int) * (N + 1));
	for(i = 0; i < N; i++){
		B[i] = 0;
	}
	B[N] = 1;
	for(i = 0; i < k; i++){
		B[i] = 1;
	}
	int ans = 1, lest = 0;
	for(i = 1; i < N; i++){
//		printf("(L, R)[%lld] = (%lld, %lld)\n", i, L[i], R[i]);
		if(L[i] == -1){
			if(lest == 0){
				printf("0\n");
				return 0;
			}
			else{
				ans = MOD(ans * lest);
				lest--;
			}
		}
		else if(R[i] - L[i] + 1 > k){
			printf("0\n");
			return 0;
		}
		else{
			if(B[L[i] - 1] == 0 && B[R[i] + 1] == 0){
				if(R[i] - L[i] + 1 < k){
					printf("0\n");
					return 0;
				}
			}
			else if(B[L[i] - 1] == 1 && B[R[i] + 1] == 1){
				ans = MOD(ans * (k - (R[i] - L[i])));
				lest += k + R[i] - L[i] - 1;
			}
			else{
				lest += R[i] - L[i];
			}
		}
		for(j = L[i]; j <= R[i]; j++){
			B[j] = 1;
		}
	}
	printf("%lld\n", ans);
	return 0;
}