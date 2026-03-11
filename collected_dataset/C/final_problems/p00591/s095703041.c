// Aizu Vol-10 1005: Advanced Algorithm Class
// 2017.8.17 bal4u@uu

#include <stdio.h>

#define N 100
int a[N][N], n;
int row[N][2], col[N][2];

int main()
{
	int r, c, f, t, min, max, h;
	
	while (scanf("%d", &n) == 1 && n) {
		for (r = 0; r < n; r++) for (c = 0; c < n; c++) scanf("%d", &a[r][c]);
		h = 0;
		for (r = 0; r < n; r++) {
			min = a[r][0], t = 0, f = 1;
			for (c = 1; c < n; c++) {
				if (a[r][c] < min) min = a[r][c], t = c, f = 1;
				else if (a[r][c] == min) f++;
			}
			if (f > 1) goto DONE;
			row[r][0] = min, row[r][1] = t;
		}

		for (c = 0; c < n; c++) {
			max = a[0][c], t = 0, f = 1;
			for (r = 1; r < n; r++) {
				if (a[r][c] > max) max = a[r][c], t = r, f = 1;
				else if (a[r][c] == max) f++;
			}
			if (f > 1) goto DONE;
			col[c][0] = max, col[c][1] = t;
		}

		for (f = 0, r = 0; r < n; r++) for (c = 0; c < n; c++) {
			if (row[r][0] == col[c][0] && row[r][1] == c && r == col[c][1]) {
				if (f == 0) h = row[r][0];
				else { h = 0; goto DONE; }
			}
		}
DONE:	printf("%d\n", h);
	}
	return 0;
}