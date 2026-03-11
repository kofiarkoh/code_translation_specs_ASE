#include<stdio.h>
#define MAX 200000

int main() {
	int N, A[MAX];
	int cnt = 0;
	scanf("%d", &N);
	for (int i = 0; i < N; i++) {
		scanf("%d", &A[i]);
	}

	for (int i = 0; i < N; i++) {
		if (i == 0) {
			if (A[0] != 0) {
				cnt = -1;
				break;
			}
		}
		else if (A[i] > A[i - 1] + 1) {
			cnt = -1;
			break;
		}
		else if (A[i] == A[i - 1] + 1) {
			cnt++;
		}
		else if (A[i] <= A[i - 1]) {
			cnt += A[i];
		}
	}

	printf("%d\n", cnt);


	return 0;
}
