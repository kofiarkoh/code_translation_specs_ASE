#include<stdio.h>
#include<string.h>
int main(void) {
	int N;
	scanf("%d", &N);
	int A[100000];
	for (int i = 0; i < N; i++) {
		scanf("%d", &A[i]);
	}

	int a;

	for (int i = 0; i < N; i++) {
		for (int j = 0; j < N - 1 - i; j++) {
			if (A[j] < A[j + 1]) {
				a = A[j];
				A[j] = A[j + 1];
				A[j + 1] = a;
			}
		}
	}

	int count_1 = 0, count_2 = 0;
	int b, c;

	for (int i = 0; i < N; i++) {
		if (A[i] == A[i + 1]&&count_1!=1) {
			b = A[i];
			A[i] = 0;
			A[i + 1] = 0;
			count_1++;
		}else if (count_1 == 1) {
			if (A[i] == A[i + 1]) {
				c = A[i];
				count_2++;
				break;
			}
		}
	}
	if (count_1 == 0) {
		printf("%d", 0);
	}
	else {
		printf("%d",b*c );
	}

		return 0;
}
