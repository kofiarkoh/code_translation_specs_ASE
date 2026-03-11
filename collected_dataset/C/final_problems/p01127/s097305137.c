// AOJ 2002: X-Ray Screening System
// 2017.11.23 bal4u@uu

#include <stdio.h>
#include <string.h>

char m[51][52];
char tbl['Z'+1]; int sz;
int  rmin[7], rmax[7], cmin[7], cmax[7];
char f[7];

int rec(int k)
{
	int i, r, c, ii;
	char tmp[51][52];

	if (k == sz) return 1;
	for (ii = 1, i = 0; i < sz; i++, ii <<= 1) {
		if (f[i]) continue;
		f[i] = 1;
		for (r = rmin[i]; r <= rmax[i]; r++) for (c = cmin[i]; c <= cmax[i]; c++) {
			if (!(m[r][c] & ii)) goto next;
		}
		memcpy(tmp, m, sizeof(tmp));
		for (r = rmin[i]; r <= rmax[i]; r++) for (c = cmin[i]; c <= cmax[i]; c++) {
			m[r][c] = 0xff;
		}
		if (rec(k+1)) return 1;
		memcpy(m, tmp, sizeof(tmp));
next:	f[i] = 0;
	}
	return 0;
}

int main()
{
	int T, H, W, r, c, i, a;

	scanf("%d", &T);
	while (T--) {
		sz = 0, memset(tbl, -1, sizeof(tbl));
		for (i = 0; i < 7; i++) rmin[i] = cmin[i] = 50, rmax[i] = cmax[i] = 0, 
		scanf("%d%d", &H, &W);
		for (r = 0; r < H; r++) {
			scanf("%s", m[r]);
			for (c = 0; c < W; c++) {
				if (m[r][c] == '.') m[r][c] = 0;
				else {
					a = m[r][c];
					if ((i = tbl[a]) < 0) i = tbl[a] = sz++;
					m[r][c] = 1 << i;
					if (rmin[i] > r) rmin[i] = r;
					if (rmax[i] < r) rmax[i] = r;
					if (cmin[i] > c) cmin[i] = c;
					if (cmax[i] < c) cmax[i] = c;
				}
			}
		}
		memset(f, 0, sz);
		puts(rec(0) ? "SAFE" : "SUSPICIOUS");
	}
	return 0;
}