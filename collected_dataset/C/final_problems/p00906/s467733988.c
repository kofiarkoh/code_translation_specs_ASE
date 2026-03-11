#include <stdio.h>
#define N_MAX 50

/* 正方行列の積を求める */
void multiply(int (*d)[N_MAX], int (*s1)[N_MAX], int (*s2)[N_MAX], int size, int mod) {
	int b1[N_MAX][N_MAX], b2[N_MAX][N_MAX];
	int i, j, k;
	/* バッファにコピーする */
	for (i = 0; i < size; i++) {
		for (j = 0; j < size; j++) {
			b1[i][j] = s1[i][j];
			b2[i][j] = s2[i][j];
		}
	}
	/* 計算する */
	for (i = 0; i < size; i++) {
		for (j = 0; j < size; j++) {
			d[i][j] = 0;
			for (k = 0; k < size; k++) d[i][j] += b1[i][k] * b2[k][j];
			d[i][j] %= mod;
		}
	}
}

/* 正方行列の累乗を求める */
int memo[30][N_MAX][N_MAX]; /* 2^i 乗した行列を格納するメモ */
void power(int (*d)[N_MAX], int (*s)[N_MAX], int size, int x, int mod) {
	int i, j;
	/* 初期化 */
	for (i = 0; i < size; i++) {
		for (j = 0; j < size; j++) {
			memo[0][i][j] = s[i][j];
			d[i][j] = (i == j) ? 1 : 0;
		}
	}
	/* x を2の累乗の和に分解して計算する */
	for (i = 0; (1 << i) <= x; i++) {
		/* 2^i 乗した行列を計算する */
		if (i >= 1) multiply(memo[i], memo[i-1], memo[i-1], size, mod);
		/* 求めるべき累乗を計算する */
		if ((1 << i) & x) multiply(d, d, memo[i], size, mod);
	}
}

int main(void) {
	int N, M, A, B, C, T, S[N_MAX];
	int matrix[N_MAX][N_MAX];
	int i, j;

	/* データセット毎の処理 */
	while (scanf("%d%d%d%d%d%d", &N, &M, &A, &B, &C, &T) == 6 && N > 0) {
		/* S(*, 0) を入力 */
		for (i = 0; i < N; i++) scanf("%d", S + i);

		/* 初期化 */
		for (i = 0; i < N; i++) {
			for (j = 0; j < N; j++) {
				if (i - j == 1) matrix[i][j] = A;
				else if (i - j == 0) matrix[i][j] = B;
				else if (i - j == -1) matrix[i][j] = C;
				else matrix[i][j] = 0;
			}
		}

		/* 行列の累乗 */
		power(matrix, matrix, N, T, M);

		/* S(*, T) を求めて出力する */
		for (i = 0; i < N; i++) {
			int s = 0;
			for (j = 0; j < N; j++) s += matrix[i][j] * S[j];
			s %= M;
			/* 出力 */
			if (i != 0) printf(" ");
			printf("%d", s);
		}
		printf("\n");
	}
	return 0;
}