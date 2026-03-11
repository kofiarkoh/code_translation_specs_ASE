// Aizu Vol-1 0141:   Spiral Pattern
// 2017.8.4

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *gets(char *);
char buf[100];

char a[105][105];
#define U 1
#define R 2
#define D 3
#define L 4

int main()
{
	int r, c, n, d, stop;
	int ci, cn;

	gets(buf);
	cn = atoi(buf);
	for (ci = 0; ci < cn; ci++) {
		gets(buf);
		n = atoi(buf);
		memset(a, ' ', sizeof(a));
		for (r = 0; r < n + 4; r += n + 3) {
			for (c = 0; c < n + 4; c++) a[r][c] = '#';
		}
		for (c = 0; c < n + 4; c += n + 3) {
			for (r = 0; r < n + 4; r++) a[r][c] = '#';
		}
		r = n + 1, c = 2, d = U, stop = 0, a[r][c] = '#';
		while (1) {
			if (stop >= 4) break;
			if (d == U) {
				if (a[r-2][c] == '#' || a[r-1][c] == '#' || a[r-1][c+1] == '#') d = R, stop++;
				else a[--r][c] = '#', stop = 0;
			}
			if (d == R) {
				if (a[r][c+2] == '#' || a[r][c+1] == '#' || a[r+1][c+1] == '#') d = D, stop++;
				else a[r][++c] = '#', stop = 0;
			}
			if (d == D) {
				if (a[r+2][c] == '#' || a[r+1][c] == '#' || a[r+1][c-1] == '#') d = L, stop++;
				else a[++r][c] = '#', stop = 0;
			}
			if (d == L){
				if (a[r][c-2] == '#' || a[r][c-1] == '#' || a[r-1][c-1] == '#') d = U, stop++;
				else a[r][--c] = '#', stop = 0;
			}
		}
		if (ci > 0) putchar('\n');
		for (r = 2; r < n + 2; r++) {
			a[r][n + 2] = 0;
			puts(a[r]+2);
		}
	}
	return 0;
}