// AOJ 1308 Awkward Lights
// 2018.3.8 bal4u

#include <stdio.h>
#include <string.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int inbit()
{
	int c = getchar_unlocked();
	getchar_unlocked();
	return c & 1;
}

#define N 630

// solve A X = C
//char A[N][N], X[N], C[N];
char B[N][N];				// working

int gauss_jordan(int n)
{
	int i, j, k, t, pivot;

//	for (i = 0; i < n; i++) for (j = 0; j < n; j++) B[i][j] = A[i][j];
//	for (i = 0; i < n; i++) B[i][n] = C[i];

	for (i = 0; i < n; i++) {
		pivot = -1;
		for (j = i; j < n; j++) if (B[j][i]) { pivot = j; break; }
		if (pivot < 0) continue;

		for (j = 0; j <= n; j++) {					// swap(B[i],B[pivot]);
			t = B[i][j], B[i][j] = B[pivot][j], B[pivot][j] = t;
		}

		for (j = 0; j < n; j++) if (i != j && B[j][i]) {
	        for (k = i; k <= n; k++) B[j][k] = (B[i][k] + B[j][k]) & 1;
		}
	}

	for (i = 0; i < n; i++) {
//		X[i] = B[i][n];
		if (!B[i][i] && B[i][n]) return 0;
	}
	return 1;
}

#define ABS(a)	((a)>=0?(a):-(a))
#define ID(r,c)	((r)*m+(c))

int main()
{
	int m, n, mn, d, i, j, r, c;

	while (m = in()) {
		n = in(), d = in(), mn = m*n;
		memset(B, 0, sizeof(B));

		for (i = 0; i < n; i++) for (j = 0; j < m; j++) {
			if (inbit()) B[ID(i,j)][mn] = 1;
			for (r = 0; r < n; r++) for (c = 0; c < m; c++) {
				if (r == i && c == j || ABS(r-i)+ABS(c-j) == d)
					B[ID(i,j)][ID(r,c)] = 1;
			}
		}
		putchar('0' + gauss_jordan(mn)), putchar('\n');
	}
	return 0;
}
