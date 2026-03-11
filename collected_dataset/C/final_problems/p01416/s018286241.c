// AOJ 2299:   Tiles are Colorful
// 2017.12.10

#include <stdio.h>
#include <string.h>

int m, n;
char map[501][503];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};
int rpos['Z'+1], cpos['Z'+1];
int sr[501], sc[501];

int calc()
{
	int r, c, i, r2, c2, a, ans = 0;
	for (r = 0; r < m; r++) for (c = 0; c < n; c++) 
		if ((sr[c] + sc[r]) && map[r][c] == '.' ) {
			memset(rpos, -1, sizeof(rpos));

		    for (i = 0; i < 4; i++) {
				r2 = r, c2 = c;
				while (1) {
					r2 += mv[i][0], c2 += mv[i][1];
					if (r2 < 0 || r2 >= m || c2 < 0 || c2 >= n) break;
					if (map[r2][c2] == '.') continue;

	                a = map[r2][c2];
					if (rpos[a] >= 0) {
	                    map[r2][c2] = '.', sr[c2]--, sc[r2]--;
						r2 = rpos[a], c2 = cpos[a];
	                    map[r2][c2] = '.', sr[c2]--, sc[r2]--;
	                    ans += 2;
					} else rpos[a] = r2, cpos[a] = c2;
					break;
				}
            }
        }

	return ans;
}

int main()
{
	int r, c, t, ans;
	char buf[10];

    fgets(buf, 10, stdin);
	sscanf(buf, "%d%d", &m, &n);
	for (r = 0; r < m; r++) {
		fgets(map[r], 503, stdin);
		for (c = 0; c < n; c++) {
			if (map[r][c] != '.') sr[c]++, sc[r]++;
		}
	}
	ans = 0; while (t = calc()) ans += t;
	printf("%d\n", ans);
	return 0;
}