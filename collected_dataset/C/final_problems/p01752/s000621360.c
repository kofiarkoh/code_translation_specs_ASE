// AOJ 2640 Prowler
// 2018.3.17 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define QMAX 1000000
typedef struct { char r, c, d; int n; } Q;
Q q[QMAX + 4]; int top, end;
int H, W, SR, SC, GR, GC, D;
char vis[52][52];
char sta[52][52][5];
char map[52][52];
char cd[128];
char pat[5][4][3][4] = { {0},
   {{"   ",	 " ^>",	 "  #"},
	{" ^ ",	 " ^#",	 "   "},
	{" # ",	 "<^#",	 "   "},
	{" # ",	 "#^#",	 " v "}},

   {{"   ",  " > ",  "#v "},
	{"   ",  " >>",  " # "},
	{" ^ ",  " >#",  " # "},
	{" # ",  "<>#",  " # "}},

   {{"#  ",  "<v ",  "   "},
	{"   ",  "#v ",  " v "},
	{"   ",  "#v>",  " # "},
	{" ^ ",  "#v#",  " # "}},

   {{" ^#",  " < ",  "   "},
	{" # ",  "<< ",  "   "},
	{" # ",  "#< ",  " v "},
	{" # ",  "#<>",  " # "}} };

int next(int *r0, int *c0, int *d0)
{
	int i, r, c, d, x;
	int rr, cc, dd;

	dd = 0;
	for (i = 0; i < 4; i++) {
		for (r = -1; r <= 1; r++) for (c = -1; c <= 1; c++) if (r != c) {
			x = pat[*d0][i][r + 1][c + 1];
			if ((d = cd[x]) & 0xf) {
				if (map[*r0 + r][*c0 + c] == '#') { dd = 0; goto NT; }
				dd = d, rr = *r0 + r, cc = *c0 + c;
			}
			else if (x == '#' && map[*r0 + r][*c0 + c] != '#') { dd = 0; goto NT; }
		}
		if (dd) break;
NT:     ;
	}
	if (!dd) return 0;
	*r0 = rr, *c0 = cc, *d0 = dd;
	return 1;
}

int bfs()
{
	int r, c, d, n;

	top = 0, end = 1;
	q[0].r = SR, q[0].c = SC, q[0].d = D, q[0].n = 1;
	sta[SR][SC][D] = 1, vis[SR][SC] = 1;
	while (top != end) {
		r = q[top].r, c = q[top].c, d = q[top].d, n = q[top].n;
		++top;

		if (!next(&r, &c, &d)) continue;
		if (r == GR && c == GC) return n + 1;
		if (sta[r][c][d]) continue;
		sta[r][c][d] = 1;
		if (!vis[r][c]) n++, vis[r][c] = 1;

		q[end].r = r, q[end].c = c, q[end].d = d, q[end].n = n;
		++end;
	}
	return -1;
}

int main()
{
	int r, c, d;

	cd['^'] = 1, cd['>'] = 2, cd['v'] = 3, cd['<'] = 4, cd['#'] = 16;
	scanf("%d%d", &H, &W);
	for (r = 1; r <= H; r++) {
		scanf("%s", map[r]+1);
		for (c = 1; c <= W; c++) {
			if (map[r][c] == 'G') GR = r, GC = c;
			else if ((d = cd[map[r][c]]) & 0xf) SR = r, SC = c, D = d;
		}
	}
	// 周りを壁"#"で囲んでおく
	memset(map[0], '#', W+2), memset(map[H + 1], '#', W+2);
	for (r = 1; r <= H; r++) map[r][0] = map[r][W + 1] = '#';

	d = 0;
	for (r = -1; r <= 1; r++) for (c = -1; c <= 1; c++) {
		if (map[GR + r][GC + c] == '#') { d = 1, r = 2; break; }
	}
	if (!d) { puts("-1"); return 0; }

	printf("%d\n", bfs());
	return 0;
}
