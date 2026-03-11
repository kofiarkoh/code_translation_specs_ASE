// Aizu Vol-2 0209: Scene in a Picture
// 2017.8.21 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef signed char schar;

#define N 53
typedef struct { int s, e; } T;
schar mado[100+2*N][100+N]; int n;
schar mono[4][N][N]; int m;
T info[4][N];

void rot90(int d, int s)
{
	int r, c;
	for (r = 0; r < m; r++) for (c = 0; c < m; c++) mono[d][c][m-1-r] = mono[s][r][c];
}

void ginfo(int k)
{
	int r, c;
	for (r = 0; r < m; r++) {
		info[k][r].s = info[k][r].e = 0;
		for (c = 0; c < m; c++) {
			if (mono[k][r][c] >= 0) { info[k][r].s = c; break; }
		}
		for (c = m-1; c >= 0; c--) {
			if (mono[k][r][c] >= 0) { info[k][r].e = c; break; }
		}
	}
}

int check(int i, int r0, int c0)
{
	int s, e, r, c, cc;

	cc = info[i][0].s;
	for (r = 0; r < m; r++) {
		s = info[i][r].s, e = info[i][r].e;
		for (c = s; c <= e; c++) {
			if (mono[i][r][c] >= 0 && mono[i][r][c] != mado[r0+r][c0+c-cc]) return 0;
		}
	}
	return 1;
}

int main()
{
	int i, r, c;

	while (scanf("%d%d", &n, &m) && n) {
		memset(mado, 0, sizeof(mado));
		for (r = 0; r < n; r++) for (c = 0; c < n; c++) scanf("%d", &mado[r][c+N]);
		for (r = 0; r < m; r++) for (c = 0; c < m; c++) scanf("%d", &mono[0][r][c]);
		for (i = 0; i < 3; i++) rot90(i+1, i);
		for (i = 0; i < 4; i++) ginfo(i);
		for (r = 0; r < n; r++) for (c = 0; c < n; c++) {
			for (i = 0; i < 4; i++)
				if (check(i, r, c+N)) { printf("%d %d\n", c+1, r+1); goto Done; }
		}
		puts("NA");
		Done:;
	}
	return 0;
}