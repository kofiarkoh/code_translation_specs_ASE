// AOJ 1024 Cleaning Robot 2.0
// 2018.2.4 bal4u

#include <stdio.h>
#include <string.h>

char map[65][65];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};
char ch[2] = {'.', 'E' };

int main()
{
    int n, n1, i, r, c, nr, nc;
    long long k;

	while (scanf("%d%lld", &n, &k) && n) {
		k--, n1 = n-1;
        if ((n & 1) || k >= (1LL<<(n/2))) { puts("No\n"); continue; }

		memset(map, -1, sizeof(map));
        for (c = 0; c < n; c++) map[0][c] = (k >> ((n1-c) >> 1)) & 1;
        for (r = 0; r < n1; r++) for (c = 0; c < n; c++) {
			int f = 0, t = map[r][c];
			for (i = 0; i < 4; i++){
				nr = r + mv[i][0], nc = c + mv[i][1];
				if (nr >= 0 && nr < n && nc >= 0 && nc < n && map[nr][nc] == t) f++;
			}
			map[r+1][c] = (f==2)? 1-t: t;
        }
        for (r = 0; r < n; r++) {
			for (c = 0; c < n; c++) putchar(ch[map[r][c]]);
            putchar('\n');
        }
		putchar('\n');
	}
	return 0;
}
