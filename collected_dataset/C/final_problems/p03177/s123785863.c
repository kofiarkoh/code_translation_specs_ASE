#include <stdio.h>

const int Mod = 1000000007;

void prod_matrix(int N, long long a[][51], long long b[][51], long long c[][51])
{
	int i, j, k;
	for (i = 1; i <= N; i++) {
		for (j = 1; j <= N; j++) {
			for (k = 1, c[i][j] = 0; k <= N; k++) c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % Mod;
		}
	}
}

void pow_matrix(int N, long long a[][51], long long K, long long b[][51])
{
	int i, j;
	long long c[51][51], d[51][51];
	for (i = 1; i <= N; i++) {
		for (j = 1; j <= N; j++) {
			b[i][j] = (i == j)? 1: 0;
			c[i][j] = a[i][j];
		}
	}
	for (; K > 0; K >>= 1) {
		if (K % 2 == 1) {
			prod_matrix(N, b, c, d);
			for (i = 1; i <= N; i++) for (j = 1; j <= N; j++) b[i][j] = d[i][j];
		}
		prod_matrix(N, c, c, d);
		for (i = 1; i <= N; i++) for (j = 1; j <= N; j++) c[i][j] = d[i][j];
	}
}

int main()
{
	int i, j, N;
	long long K, a[51][51];
	scanf("%d %lld", &N, &K);
	for (i = 1; i <= N; i++) for (j = 1; j <= N; j++) scanf("%lld", &(a[i][j]));
	
	long long b[51][51], ans = 0;
	pow_matrix(N, a, K, b);
	for (i = 1; i <= N; i++) for (j = 1; j <= N; j++) ans += b[i][j];
	printf("%lld\n", ans % Mod);
	fflush(stdout);
	return 0;
}