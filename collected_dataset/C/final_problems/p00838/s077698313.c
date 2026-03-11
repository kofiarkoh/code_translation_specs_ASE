// AOJ 1259: Colored Cubes
// 2017.10.22 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HASHSIZ 101
typedef struct { char *s; int id; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;			// Hash table
char color[25][25]; int sz;
char dice[4][7], dice2[4][7]; int n;
int ans;

int lookup(char *s, int id)
{
	int w = strlen(s);
	int i = (101 * *s + 103 * *(s+w-1) + 107 * w) % HASHSIZ;
	HASH *tp = hash + i;

	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->id;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->id = id;
	return -1;
}

char rot[24][7] = {
	{ 0,1,2,3,4,5,6 },{ 0,1,3,5,2,4,6 },{ 0,1,4,2,5,3,6 },{ 0,1,5,4,3,2,6 },
	{ 0,2,6,3,4,1,5 },{ 0,2,3,1,6,4,5 },{ 0,2,1,4,3,6,5 },{ 0,2,4,6,1,3,5 },
	{ 0,3,1,2,5,6,4 },{ 0,3,2,6,1,5,4 },{ 0,3,5,1,6,2,4 },{ 0,3,6,5,2,1,4 },
	{ 0,4,1,5,2,6,3 },{ 0,4,2,1,6,5,3 },{ 0,4,5,6,1,2,3 },{ 0,4,6,2,5,1,3 },
	{ 0,5,1,3,4,6,2 },{ 0,5,3,6,1,4,2 },{ 0,5,4,1,6,3,2 },{ 0,5,6,4,3,1,2 },
	{ 0,6,2,4,3,5,1 },{ 0,6,3,2,5,4,1 },{ 0,6,5,3,4,2,1 },{ 0,6,4,5,2,3,1 } };

int combi(int m)
{
	int i, j, k, l, min, max;
	char tmp[7];
	char f[4], c[4];

	if (m == n) {
		for (min = 0, k = 1; k <= 6; k++) {
			c[0] = dice2[0][k], f[0] = 1, l = 1, max = 1;
			for (i = 1; i < n; i++) {
				for (j = 0; j < l && dice2[i][k] != c[j]; j++);
				if (j < l) { if (++f[j] > max) max++; }
				else c[l] = dice2[i][k], f[l++] = 1;
			}
			min += n-max;
		}
		if (min < ans) {
			ans = min;
			if (ans == 0) return 1;
		}
		return 0;
	}
	memcpy(dice2[m], dice[m], 7);
	if (combi(m+1)) return 1;
	for (i = 1; i < 24; i++) {
		for (k = 1; k <= 6; k++) tmp[k] = dice[m][rot[i][k]];
		if (memcmp(tmp+1, dice2[m]+1, 6) == 0) continue;
		memcpy(dice2[m], tmp, 7);
		if (combi(m+1)) return 1;
	}
	return 0;
}

int main()
{
	int i, j, k;

	while (scanf("%d", &n) && n > 0) {
		if (n == 1) {
			for (j = 0; j < 6; j++) scanf("%s", color[0]);
			puts("0"); continue;
		}
		memset(hash, 0, sizeof(hash)); sz = 0;
		for (i = 0; i < n; i++) {
			for (j = 0; j < 6; j++) {
				scanf("%s", color[sz]);
				if ((k = lookup(color[sz], sz)) < 0) k = sz++;
				dice[i][j+1] = k;
			}
		}

		ans = 128;
		memcpy(dice2[0], dice[0], 7);
		combi(1);
		printf("%d\n", ans);
	}
	return 0;
}