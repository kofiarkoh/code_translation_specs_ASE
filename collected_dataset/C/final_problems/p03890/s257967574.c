#include <stdio.h>
#include <stdlib.h>

int f(int X, int Y){
	if(X > Y){
		return X - Y;
	}
	else if(X < Y){
		return Y - X;
	}
	else{
		return X;
	}
}

int main(){
	int N, i, j;
	scanf("%d", &N);
	int **A = (int **)malloc(sizeof(int) * (N + 1));
	for(i = 0; i <= N; i++){
		A[i] = (int *)malloc(sizeof(int) * (1 << i));
	}
	for(j = 0; j < (1 << N); j++){
		scanf("%d", &A[N][j]);
	}
	for(i = N - 1; i >= 0; i--){
		for(j = 0; j < (1 << i); j++){
			A[i][j] = f(A[i + 1][2 * j], A[i + 1][2 * j + 1]);
		}
	}
	printf("%d\n", A[0][0]);
	return 0;
}