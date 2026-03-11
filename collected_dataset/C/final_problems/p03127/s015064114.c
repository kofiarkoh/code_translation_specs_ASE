#include <stdio.h>

int main() {
	int N, A[10^5];
	int i,m,n,buf;

	scanf("%d", &N);

	for (i = 0;i<N; i++) {
		scanf("%d", &A[i]);
	}
	for (i = 0; i < N-1; i++) {
		if (A[i] >= A[i + 1]) {
			m = A[i]; n = A[i + 1];
		}
		else {
			m = A[i + 1]; n = A[i];
		}
		while (n != 0) {
				buf = n;
				n = m % n;
				m = buf;
		}
		A[i + 1] = m;
	}
	printf("%d",A[N-1]);


	return 0;
}