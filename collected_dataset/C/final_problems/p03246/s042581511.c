#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int i;
	int num;
}pair;

signed compair(const void *a, const void *b){
	return ((pair *)a)->num - ((pair *)b)->num;
}

int max(int a, int b){
	return a >= b ? a : b;
}

int main(){
	int N, i, M = 100000, ans;
	scanf("%d", &N);
	int *v = (int *)malloc(sizeof(int) * N);
	pair *A = (pair *)malloc(sizeof(pair) * (M + 1));
	pair *B = (pair *)malloc(sizeof(pair) * (M + 1));
	for(i = 0; i <= M; i++){
		A[i].num = 0;
		B[i].num = 0;
		A[i].i = i;
		B[i].i = i;
	}
	for(i = 0; i < N; i++){
		scanf("%d", &v[i]);
		if(i % 2 == 0){
			A[v[i]].num++;
		}
		else{
			B[v[i]].num++;
		}
	}
	qsort(A, M + 1, sizeof(pair), compair);
	qsort(B, M + 1, sizeof(pair), compair);
	if(A[M].i != B[M].i){
		ans = A[M].num + B[M].num;
	}
	else{
		ans = A[M].num + max(A[M - 1].num, B[M - 1].num);
	}
	printf("%d\n", N - ans);
	return 0;
}