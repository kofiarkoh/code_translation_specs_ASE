#include <stdio.h>

int min(int a[], int g) {
	int min = 1000;
	for (int i = 0; i < g; i++) {
		if (a[i] < min)min = a[i];
	}
	return min;
}

int main()
{
	int N, T, A[100000], max = -100, i, count = 0;
	scanf("%d%d", &N, &T);
	for (i = 0; i < N; i++) {
		scanf("%d", &A[i]);
	}
	for (i = 0; i < N; i++) {
		if (A[i] - min(A, i) > max) {
			max = A[i] - min(A, i);
		}
	}
	for (i = 0; i < N; i++) {
		if (A[i] - min(A, i) == max)count++;
	}
	printf("%d", count);
}
