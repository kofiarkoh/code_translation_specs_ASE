#include <stdio.h>
#include <stdlib.h>

int main()
{
	int i, N, A[3][101];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[1][i]));
	for (i = 1; i <= N; i++) scanf("%d", &(A[2][i]));
	
	int max = 0;
	for (i = 1; i < N; i++) {
		A[1][i+1] += A[1][i];
		A[2][N-i] += A[2][N-i+1];
	}
	for (i = 1; i <= N; i++) {
		if (A[1][i] + A[2][i] > max) max = A[1][i] + A[2][i];
	}
	
	printf("%ld\n", max);
	fflush(stdout);
	return 0;
}