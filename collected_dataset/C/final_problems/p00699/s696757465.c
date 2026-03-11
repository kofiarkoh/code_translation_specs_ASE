// Aizu 1118: Nets of Dice
// 2017.9.16 bal4u@uu

#include <stdio.h>
#include <string.h>

char net[5][5];
int f[7];
char pat[11][3][5] = { 
	{{1,0,0,0},{2,3,2,3},{1,0,0,0}}, {{1,0,0,0},{2,3,2,3},{0,1,0,0}}, {{1,0,0,0},{2,3,2,3},{0,0,1,0}},
	{{1,0,0,0},{2,3,2,3},{0,0,0,1}}, {{0,1,0,0},{2,3,2,3},{0,1,0,0}}, {{0,1,0,0},{2,3,2,3},{0,0,1,0}},
	{{1,2,1,0,0},{0,0,3,2,3},{0 }},  {{1,2,0,0},{0,3,1,3},{0,2,0,0}}, {{1,3,0,0},{0,2,1,2},{0,0,3,0}},
	{{1,3,0,0},{0,2,1,2},{0,0,0,3}}, {{1,2,0,0},{0,3,1,0},{0,0,2,3}}};
int spec[11][3] = { {0,3,4}, {0,3,4}, {0,3,4}, {0,3,4},	{1,3,4}, 
					{1,3,4}, {0,2,5}, {0,3,4}, {0,3,4}, {0,3,4}, {0,3,4} };

void rot90()
{
	int r, c; char t[5][5];
	for (r = 0; r < 5; r++) for (c = 0; c < 5; c++) t[r][c] = net[c][4-r];
	memcpy(net, t, sizeof(net));
}

void rev()
{
	int r, c; char t[5][5];
	for (r = 0; r < 5; r++) for (c = 0; c < 5; c++) t[r][c] = net[r][4-c];
	memcpy(net, t, sizeof(net));
}

int match()
{
	int i, j, r, c, r2, c2, rr, cc, x, y;

	for (r = 0; r < 4; r++) for (c = 0; c < 3; c++) {
		if (!net[r][c]) continue;
		for (i = 0; i < 11; i++) {
			rr = r, cc = c;	if (spec[i][0]) if (cc == 0) continue; else cc--;
			if (rr + spec[i][1] > 5 || cc + spec[i][2] > 5) continue;
			memset(f, 0, sizeof(f));
			for (y = 0; y < spec[i][1]; y++) for (x = 0; x < spec[i][2]; x++) {
				r2 = rr + y, c2 = cc + x;
				if (!net[r2][c2] && pat[i][y][x] > 0 ||
					net[r2][c2] > 0 && !pat[i][y][x]) goto Next;
				f[pat[i][y][x]] += net[r2][c2];
			}
			for (j = 1; j <= 3; j++) if (f[j] != 7) break;
			if (j > 3) return 1;
			Next:;
		}
	}
	return 0;
}

int check()
{
	int i;
	if (match()) return 1;
	for (i = 0; i < 3; i++) { rot90(); if (match()) return 1; }
	rot90(), rev();
	if (match()) return 1;
	for (i = 0; i < 3; i++) { rot90(); if (match()) return 1; }
	return 0;
}

int main()
{
	int n, r, c, x;

	scanf("%d", &n); while (n-- > 0) {
		memset(f, 0, sizeof(f));
		for (r = 0; r < 5; r++) for (c = 0; c < 5; c++) {
			scanf("%d", &x), net[r][c] = x, f[x]++;
		}
		for (x = 1; x <= 6; x++) if (f[x] != 1) break;
		if (x <= 6) puts("false");
		else puts(check() ? "true" : "false");
	}
	return 0;
}