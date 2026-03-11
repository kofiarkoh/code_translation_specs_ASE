#include <stdio.h>
#include <inttypes.h>

int N;
int A[512][512];

char kawariwaaru[512][512];

int main(void) {
	int i, j, k;
	int64_t asumikana = 0;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		for (j = 0; j < N; j++) {
			if (scanf("%d", &A[i][j]) != 1) return 1;
		}
	}
	for (k = 0; k < N; k++) {
		for (i = 0; i < N; i++) {
			for (j = 0; j < N; j++) {
				if (A[i][k] + A[k][j] < A[i][j]) {
					puts("-1"); /* 最短じゃねーじゃねーか！ */
					return 0;
				} else if (A[i][k] + A[k][j] == A[i][j] && k != i && k != j) {
					kawariwaaru[i][j] = 1; /* かわりはあるもの */
				}
			}
		}
	}
	for (i = 0; i < N; i++) {
		for (j = i + 1; j < N; j++) {
			if (!kawariwaaru[i][j]) asumikana += A[i][j];
		}
	}
	printf("%" PRId64 "\n", asumikana);
	return 0;
}
