// AOJ 1253: Dice Puzzle
// 2017.10.21 bal4u@uu

#include <stdio.h>
#include <string.h>

char rot[24][7] = {
	{ 0,1,2,3,4,5,6 },{ 0,1,3,5,2,4,6 },{ 0,1,4,2,5,3,6 },{ 0,1,5,4,3,2,6 },
	{ 0,2,6,3,4,1,5 },{ 0,2,3,1,6,4,5 },{ 0,2,1,4,3,6,5 },{ 0,2,4,6,1,3,5 },
	{ 0,3,1,2,5,6,4 },{ 0,3,2,6,1,5,4 },{ 0,3,5,1,6,2,4 },{ 0,3,6,5,2,1,4 },
	{ 0,4,1,5,2,6,3 },{ 0,4,2,1,6,5,3 },{ 0,4,5,6,1,2,3 },{ 0,4,6,2,5,1,3 },
	{ 0,5,1,3,4,6,2 },{ 0,5,3,6,1,4,2 },{ 0,5,4,1,6,3,2 },{ 0,5,6,4,3,1,2 },
	{ 0,6,2,4,3,5,1 },{ 0,6,3,2,5,4,1 },{ 0,6,5,3,4,2,1 },{ 0,6,4,5,2,3,1 } };

typedef struct { int x, y, z; } T;
T n2xyz[28];

char dice[3][3][3][7];
int top[3][3], front[3][3];
char ans[55], sz;

void combi(char dice[3][3][3][7], int n)
{
	int s, x, y, z, i;

	if (n == 27) {
		for (s = 0, y = 0; y < 3; y++) for (z = 0; z < 3; z++) s += dice[z][y][2][3];
		sz = 1;
		ans[s] = 1;
		return;
	}
	x = n2xyz[n].x, y = n2xyz[n].y, z = n2xyz[n].z;
	for (i = 0; i < 24; i++) {
		memcpy(dice[z][y][x], rot[i], 7);
		if (x > 0 && dice[z][y][x][4] + dice[z][y][x-1][3] != 7) continue;
		if (y == 0) { if (front[z][x] > 0 && front[z][x] != dice[z][0][x][2]) continue; }
		else { if (dice[z][y][x][2] + dice[z][y-1][x][5] != 7) continue; }
		if (z == 0) { if (top[y][x] > 0 && top[y][x] != dice[0][y][x][1]) continue; }
		else { if (dice[z][y][x][1] + dice[z-1][y][x][6] != 7) continue; }
		combi(dice, n + 1);
	}
}

int main()
{
	int n, i, x, y, z;

	for (x = 0; x < 3; x++) for (y = 0; y < 3; y++) for (z = 0; z < 3; z++) {
		n = 9 * z + 3 * y + x;
		n2xyz[n].x = x, n2xyz[n].y = y, n2xyz[n].z = z;
	}

	scanf("%d", &n);
	while (n-- > 0) {
		memset(ans, 0, sizeof(ans));
		for (y = 2; y >= 0; y--) for (x = 0; x < 3; x++) scanf("%d", &top[y][x]);
		for (z = 0; z < 3; z++) for (x = 0; x < 3; x++) scanf("%d", &front[z][x]);

		sz = 0;
		combi(dice, 0);
		if (sz == 0) puts("0");
		else {
			for (i = 9; i <= 54; i++) if (ans[i]) {
				if (sz > 0) sz = 0;
				else putchar(' ');
				printf("%d", i);
			}
			putchar('\n');
		}
	}
	return 0;
}