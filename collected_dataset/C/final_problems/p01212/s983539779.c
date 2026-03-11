// AOJ 2089 Mysterious Dungeons
// 2018.3.24 bal4u

#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define QMAX  230500
typedef unsigned char uchar;
typedef struct { char r, c; uchar s; int t; } Q;
Q q[QMAX+2]; int top, end;
short mk[31][31][1<<8];
char map[33][33];
char f['z'+1], id['z'+1]; int sz;
int  mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

int main()
{
	int W, H, i, r, c, nr, nc, t, s, ns, a, ans;
	int sr, sc, gr, gc;
	char buf[10], *p;
	
	while (fgets(p=buf, 10, stdin) && *p != '0') {
		sscanf(p, "%d%d", &W, &H);
		sz = 0; memset(f, 0, sizeof(f)), memset(id, 0, sizeof(id));
		for (r = 0; r < H; r++) {
			fgets(p=map[r], 33, stdin);
			for (c = 0; c < W; c++) {
				if      (*p == '@') sr = r, sc = c, *p = '.';
				else if (*p == '<') gr = r, gc = c, *p = '.';
				else if (islower(*p)) f[*p] |= 1, f[toupper(*p)] |= 1;
				else if (isupper(*p)) f[*p] |= 2, f[tolower(*p)] |= 2;
				p++;
			}
		}
		for (r = 0; r < H; r++) {
			for (p = map[r], c = 0; c < W; c++) {
				if (islower(*p)) {
					if (f[*p] != 3) *p = '.';
					else {
						if (!id[*p]) id[*p] = (sz<<2)|1, id[toupper(*p)] = ((sz++)<<2)|3;
						*p = id[*p]>>1;
					}
				} else if (isupper(*p)) {
					if (f[*p] != 3) *p = '#';
					else {
						if (!id[*p]) id[*p] = (sz<<2)|3, id[tolower(*p)] = ((sz++)<<2)|1;
						*p = id[*p]>>1;
					}
				}
				p++;
			}
		}

		memset(mk, 0x55, sizeof(mk));
		q[0].r = sr, q[0].c = sc, q[0].t = 0, q[0].s = 0, mk[sr][sc][0] = 0;
		top = 0, end = 1, ans = -1;
		while (top != end) {
			r = q[top].r, c = q[top].c, t = q[top].t, s = q[top++].s;

			if (r == gr && c == gc) { ans = t; break; }
			if (mk[r][c][s] < t) continue;

			for (i = 0; i < 4; i++) {
				nr = r + mv[i][0], nc = c + mv[i][1], ns = s;
				if (map[nr][nc] == '#') continue;
				if ((a = map[nr][nc]) < 32) {
					if (a & 1) { if (!(s & (1<<(a>>1)))) continue; }
					else ns = s ^ (1<<(a>>1));
				}
				if (mk[nr][nc][ns] > t+1) {
					mk[nr][nc][ns] = t+1;
					q[end].r = nr, q[end].c = nc, q[end].t = t+1, q[end++].s = ns;
				}
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}

