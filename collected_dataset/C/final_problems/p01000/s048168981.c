// AOJ 1510 Independent Research
// 2018.1.14 bal4u

#include <stdio.h>
#include <string.h>

char map[2][5][5][7];
int a[27], b[27];
int mv[26][3] = {{-1,-1,-1},{-1,-1,0},{-1,-1,1},{-1,0,-1},{-1,0,0},{-1,0,1},{-1,1,-1},{-1,1,0},{-1,1,1},
	             {0,-1,-1},{0,-1,0},{0,-1,1},{0,0,-1},{0,0,1},{0,1,-1},{0,1,0},{0,1,1},
	             {1,-1,-1},{1,-1,0},{1,-1,1},{1,0,-1},{1,0,0},{1,0,1},{1,1,-1},{1,1,0},{1,1,1}};
				 
int main()
{
	int cno, n, m, x, y, z, i;
	int t, xx, yy, zz, s, k1, k2;

	cno = 0;
	while (scanf("%d", &n) && n > 0) {
		for (z = 0; z < 5; z++) {
			for (y = 0; y < 5; y++) scanf("%s", map[0][z][y]);
		}
		memset(a, 0, sizeof(a)), memset(b, 0, sizeof(b));
		
		scanf("%d", &m); while (m--) scanf("%d", &t), a[t] = 1;
		scanf("%d", &m); while (m--) scanf("%d", &t), b[t] = 1;
		for (k1 = 0, k2 = 1; n--; k1 = k2, k2 = !k2) {
			for (z = 0; z < 5; z++) for (y = 0; y < 5; y++) for (x = 0; x < 5; x++) {
				s = 0;
				for (i = 0; i < 26; i++) {
					xx = x + mv[i][0], yy = y + mv[i][1], zz = z + mv[i][2];
					if (xx >= 0 && xx < 5 && yy >= 0 && yy < 5 && zz >= 0 && zz < 5) s += map[k1][zz][yy][xx] & 1;
				}
				if (map[k1][z][y][x] & 1) map[k2][z][y][x] = b[s] + '0';
				else 			          map[k2][z][y][x] = a[s] + '0';
			}
		}
		if (cno) putchar('\n');
		printf("Case %d:\n", ++cno);
		for (z = 0; z < 5; z++) {
			if (z > 0) putchar('\n');
			for (y = 0; y < 5; y++) puts(map[k1][z][y]);
		}
	}
	return 0;
}
		
