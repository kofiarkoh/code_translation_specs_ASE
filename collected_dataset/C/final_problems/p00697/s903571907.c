// Aizu 1116: Jigsaw Puzzles for Computers
// 2017.9.16 bal4u@uu

#include <stdio.h>
#include <string.h>

char dice[9][6];
char map[3][3][4];
char tr['z'];
int f[10];
int ans;
int rot[4][4] = {{0,1,2,3},{1,2,3,0},{2,3,0,1},{3,0,1,2}};

void combi(int k)
{
	int r, c, i, j, m;

	if (k == 9) { ans++; return; }
	r = k / 3, c = k % 3;
	for (i = 0; i < 9; i++) {
		if (f[i]) continue;
		for (j = 0; j < 4; j++) {
			for (m = 0; m < 4; m++) map[r][c][m] = dice[i][rot[j][m]];
			if (r > 0 && map[r][c][0] + map[r-1][c][2] != 0) continue;
			if (c > 0 && map[r][c][3] + map[r][c-1][1] != 0) continue;
			f[i] = 1, combi(k+1), f[i] = 0;
		}
	}
}

int main()
{
	int n, i, j;

	tr['R'] =  1, tr['G'] =  2, tr['B'] =  3, tr['W'] =  4;
	tr['r'] = -1, tr['g'] = -2, tr['b'] = -3, tr['w'] = -4;
	scanf("%d", &n); while (n-- > 0) {
		for (i = 0; i < 9; i++) {
			scanf("%s", dice[i]);
			for (j = 0; j < 4; j++) dice[i][j] = tr[dice[i][j]];
		}
		memset(f, 0, sizeof(f)), ans = 0;
		combi(0);
		printf("%d\n", ans);
	}
	return 0;
}