#include <stdio.h>

#define DIM 16

const long long Mod = 1000000007;

long long div_mod(long long x, long long y, long long z)
{
	if (x % y == 0) return x / y;
	else return (div_mod((1 + x / y) * y - x, (z % y), y) * z + x) / y;
}

int solve_mod(long long A[][DIM], long long b[], long long x[])
{
	int i, j, k, tmp;
	for (i = 0; i < DIM; i++) {
		if (A[i][i] == 0) {
			for (j = i + 1; j < DIM; j++) if (A[j][i] != 0) break;
			if (j == DIM) return -1;
			b[i] ^= b[j];
			b[j] ^= b[i];
			b[i] ^= b[j];
			for (k = 0; k < DIM; k++) {
				A[i][k] ^= A[j][k];
				A[j][k] ^= A[i][k];
				A[i][k] ^= A[j][k];
			}
		}
		
		for (j = i + 1; j < DIM; j++) {
			if (A[j][i] == 0) continue;
			tmp = div_mod(A[j][i], A[i][i], Mod);
			b[j] = (b[j] - b[i] * tmp % Mod + Mod) % Mod;
			for (k = i; k < DIM; k++) A[j][k] = (A[j][k] - A[i][k] * tmp % Mod + Mod) % Mod;
		}
	}
	
	for (i--; i >= 0; i--) {
		for (j = DIM - 1; j > i; j--) b[i] = (b[i] - A[i][j] * x[j] % Mod + Mod) % Mod;
		x[i] = div_mod(b[i], A[i][i], Mod);
	}
	return 0;
}

int main()
{
	int t, T, N;
	scanf("%d", &T);
	
	int i, j, k, l, m;
	long long A[DIM][DIM], b[2][DIM] = {}, x[2][DIM] = {}, tmp[DIM * 2];
	for (i = 1; i < DIM * 2; i++) {
		if (i % 2 == 1) tmp[i] = (i + 1) * (i + 1) / 4;
		else tmp[i] = i * (i + 2) / 4;
	}
	for (N = 0; N < DIM * 2; N++) {
		for (i = 1; i <= N - 4; i++) {
			for (j = 1; j <= N - 3 - i; j++) {
				for (k = 1; k <= N - 2 - i - j; k++) {
					for (l = 1; l <= N - 1 - i - j - k; l++) {
						for (m = 1; m <= N - i - j - k - l; m++) b[N%2][N/2] = (b[N%2][N/2] + tmp[i] * tmp[j] * tmp[k] * tmp[l] * tmp[m]) % Mod;
					}
				}
			}
		}
	}
	
	for (i = 0; i < DIM; i++) {
		for (j = 1, A[i][0] = 1; j < DIM; j++) A[i][j] = A[i][j-1] * i % Mod;
	}
	solve_mod(A, b[0], x[0]);
	for (i = 0; i < DIM; i++) {
		for (j = 1, A[i][0] = 1; j < DIM; j++) A[i][j] = A[i][j-1] * i % Mod;
	}
	solve_mod(A, b[1], x[1]);
	
	long long ans;
	for (t = 1; t <= T; t++) {
		scanf("%d", &N);
		for (i = DIM - 1, ans = 0; i >= 0; i--) ans = (ans * (N / 2) + x[N%2][i]) % Mod;
		printf("%lld\n", ans);
	}
	
	fflush(stdout);
	return 0;
}