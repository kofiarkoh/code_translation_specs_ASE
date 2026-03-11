#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main() {


	int i, j, N, *A, *B, tmp, tmp2, time=0;


	scanf("%d", &N);

	A = (int*)malloc(N*sizeof(int));
	B = (int*)malloc(N * sizeof(int));

	for (i = 0; i < N; i++) {
		scanf("%d %d",&A[i], &B[i]);
	}

	for (i = 0; i < N-1; i++) {
		for (j = 0; j < N-1; j++) {
			if (B[j] > B[j + 1]) {

				tmp = A[j];
				A[j] = A[j + 1];
				A[j + 1] = tmp;


				tmp2 = B[j];
				B[j] = B[j + 1];
				B[j + 1] = tmp2;

			}
		}
}

	for (i = 0; i < N; i++) {
		time += A[i];
		if (time > B[i]) {
			printf("No\n");
			return 0;
		}

	}

	printf("Yes\n");
	return 0;
}