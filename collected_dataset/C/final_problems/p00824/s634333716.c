// AOJ 1245: Gap
// 2017.10.18 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { char r, c; } T;
#define QMAX 70000
typedef struct { char tbl[4][8]; int s; } Q;
Q q[QMAX+3]; int top, end;
char tbl[4][8];

#define HASHSIZ 120011
unsigned hash[HASHSIZ+2], *hashend = hash + HASHSIZ;

int insert(unsigned x)
{
	unsigned *p = hash + x % HASHSIZ;
	while (*p) {
		if (*p == x) return 1;
		if (++p == hashend) p = hash;
	}
	*p = x;
	return 0;
}

unsigned check(char tbl[4][8])
{
	int i, j, k, f;
	unsigned x;

	for (x = 0, f = 0, k = 11, i = 0; i < 4; i++, k += 10) {
		for (j = 1; j < 7; j++) {
			x <<= 1, x += tbl[i][j];
			if (tbl[i][j] != k+j) f = 1;
		}
		x <<= 1, x += tbl[i][7];
	}
	if (f) return x;
	return 0;
}

int main()
{
	int t, i, j, r, c, r2, c2, v, s, ans;
	char tmp[4][8];
	unsigned x;

	scanf("%d", &t);
	while (t-- > 0) {
		for (r = 0; r < 4; r++) for (c = 1; c < 8; c++) scanf("%d", &v), tbl[r][c] = v;
		// initial layout
		for (r = 0; r < 4; r++) {
			tbl[r][0] = 10*r + 11;
			for (c = 1; c < 8; c++)	if (tbl[r][c] % 10 == 1) tbl[r][c] = 0;
		}

		memset(hash, 0, sizeof(hash));
		if ((x = check(tbl)) == 0) { puts("0"); continue; }
		insert(x);
		ans = -1, top = 0, end = 1;
		memcpy(q[0].tbl, tbl, sizeof(tbl)), q[0].s = 0;

		while (top < end) {
			for (i = 0; i < 4; i++) {
				memcpy(tmp, q[top].tbl, sizeof(tbl)), s = q[top].s + 1;
				for (j = 0, r = 0; r < 4; r++) for (c = 1; c < 8; c++) {
					if (tmp[r][c] == 0) { if (j++ == i) goto next1; }
				}
		next1:
				v = tmp[r][c-1];
				if (v == 0 || v % 10 == 7) continue;
				v++;
				for (r2 = 0; r2 < 4; r2++) for (c2 = 1; c2 < 8; c2++) {
					if (tmp[r2][c2] == v) {
						tmp[r2][c2] = 0, tmp[r][c] = v;
						goto next2;
					}
				}
		next2:
				if ((x = check(tmp)) == 0) { ans = s; goto done; }
				if (insert(x)) continue;
				memcpy(q[end].tbl, tmp, sizeof(tbl)), q[end].s = s, end++;
			}
			++top;
		}
done:	printf("%d\n", ans);
	}
 	return 0;
}