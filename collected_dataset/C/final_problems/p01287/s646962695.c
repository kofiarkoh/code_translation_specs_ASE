// AOJ 2169 Colored Octahedra
// 2018.3.20 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char hash[16777218];

int tbl[24][8] = {
{0,1,2,3,4,5,6,7},{0,3,7,4,1,2,6,5},{0,4,5,1,3,7,6,2},
{1,0,4,5,2,3,7,6},{1,2,3,0,5,6,7,4},{1,5,6,2,0,4,7,3},
{2,1,5,6,3,0,4,7},{2,3,0,1,6,7,4,5},{2,6,7,3,1,5,4,0},
{3,0,1,2,7,4,5,6},{3,2,6,7,0,1,5,4},{3,7,4,0,2,6,5,1},
{4,0,3,7,5,1,2,6},{4,5,1,0,7,6,2,3},{4,7,6,5,0,3,2,1},
{5,1,0,4,6,2,3,7},{5,4,7,6,1,0,3,2},{5,6,2,1,4,7,3,0},
{6,2,1,5,7,3,0,4},{6,5,4,7,2,1,0,3},{6,7,3,2,5,4,0,1},
{7,3,2,6,4,0,1,5},{7,4,0,3,6,5,1,2},{7,6,5,4,3,2,1,0}};

char name[8][22];
char id[8]; int n;
char used[8];
char color[8];
int  ans;

int check()
{
	int i, j, id;
	char t[8];

	id = 0; for (i = 0; i < 8; i++) id = (id << 3) | color[i];
	if (hash[id]) return 0;
	hash[id] = 1;
	for (j = 1; j < 24; j++) {
		for (i = 0; i < 8; i++) t[i] = color[tbl[j][i]];
		id = 0; for (i = 0; i < 8; i++) id = (id << 3) | t[i];
		hash[id] = 1;
	}
	return 1;
}

void rec(int k)
{
	int  i;
	char f[8];
	
	if (k == 8) {
		if (check()) ans++;
		return;
	}
	memset(f, 0, 8);
	for (i = 0; i < 8; i++) {
		if (used[i] || f[id[i]]) continue;
		color[k] = id[i], f[id[i]] = 1;
		used[i] = 1, rec(k+1), used[i] = 0;
	}
}

int main()
{
	int i, j, max;

    while (1) {
		memset(hash, 0, sizeof(hash));
		n = 0;
		for (i = 0; i < 8; i++) {
			if (scanf("%s", name[i]) != 1) return 0;
			for (j = 0; ; j++) {
				if (j == i) { id[i] = n++; break; }
				if (!strcmp(name[j], name[i])) { id[i] = id[j]; break; }
			}
		}
		if (n == 1) { puts("1"); continue; }
		if (n == 8) { puts("1680"); continue; }

		for (i = 0; i < 8; i++) used[id[i]]++;
		max = 0;
		for (i = 1; i < 8; i++) if (used[i] > max) max = i;
		for (i = 0; i < 8; i++) if (id[i] == max) break;

		ans = 0;
		memset(used, 0, 8);
		used[i] = 1, color[0] = id[i];
		rec(1), used[i] = 0;
		printf("%d\n", ans);
	}
	return 0;
}
