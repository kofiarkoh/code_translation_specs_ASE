#include <stdio.h>

long long A[100][100] = {0};
long long B[100][100] = {0};
long long C[100][100] = {0};

int main(void) {
	int n, m, l;
	int i, j, k;
	scanf(" %d %d %d", &n, &m, &l);

	for (i = 0; i < n; ++i) {
		for (j = 0; j < m; ++j) {
			scanf(" %lld", &A[i][j]);
		}
	}

	for (i = 0; i < m; ++i) {
		for (j = 0; j < l; ++j) {
			scanf(" %lld", &B[i][j]);
		}
	}

	for (i = 0; i < n; ++i) {
		for (j = 0; j < m; ++j) {
			for (k = 0; k < l; ++k) {
				C[i][k] += A[i][j] * B[j][k];
			}
		}
	}

	for (i = 0; i < n; ++i) {
		for (j = 0; j < l; ++j) {
			printf("%lld%c", C[i][j], (j == l - 1)?'\n':' ');
		}
	}
	
	return 0;
}