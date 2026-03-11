#include <stdio.h>

int main(void) {
	int N, M;
	while (scanf("%d%d", &N, &M) == 2 && (N != 0 || M != 0)) {
		int i;
		int asumikana = M / N;
		int igutiyuka = 0;
		for (i = 0; i < N; i++) {
			int A;
			if (scanf("%d", &A) != 1) return 0;
			igutiyuka += A >= asumikana ? asumikana : A;
		}
		printf("%d\n", igutiyuka);
	}
	return 0;
}

