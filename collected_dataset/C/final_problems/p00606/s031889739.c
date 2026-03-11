// Aizu 1020: Cleaning Robot
// 2017.9.17 bal4u@uu

#include <stdio.h>
#include <string.h>

double f[17][3][3];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

int main()
{
	int n, s, t, b, i, j, r, c, r2, c2;
	char t1[5], t2[5], t3[5];

	while (scanf("%d", &n) && n) {
		scanf("%s%s%s", t1, t2, t3), s = *t1-'A', t = *t2-'A', b = *t3-'A';
		memset(f, 0, sizeof(f)), f[0][s/3][s%3] = 1.0;
		for (j = 1; j <= n; j++) {
			for (r = 0; r < 3; r++) for (c = 0; c < 3; c++) {
				for (i = 0; i < 4; i++) {
					r2 = r + mv[i][0], c2 = c + mv[i][1];
					if (r2 < 0 || r2 >= 3 || c2 < 0 || c2 >= 3 || 3*r+c == b)
						 f[j][r][c]   += f[j-1][r][c]/4.0;
					else f[j][r2][c2] += f[j-1][r][c]/4.0;
				}
			}
		}
		printf("%.8lf\n", f[n][t/3][t%3]);
	}
	return 0;
}