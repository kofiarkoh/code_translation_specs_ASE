// Aizu 1140: Cleaning Robot
// 2017.9.14 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef struct { int r, c, d; } Q;
Q que[1003]; int top, end;

typedef struct { int r, c; } PP;
PP pp[12]; int n;
char map[22][22], mk[22][22]; int w, h;
int dist[12][12];
int m[4][2] = { {-1,0}, {0,1}, {1,0}, {0,-1} }; 
char f[12];
int ans;

int distance()
{
	int i, j, k, d, r, c, r2, c2;

	for (i = 0; i < n; i++) for (j = i+1; j < n; j++) {
		memset(mk, 0, sizeof(mk));
		que[0].r = pp[i].r, que[0].c = pp[i].c, que[0].d = 0, top = 0, end = 1;
		while (1) {
			if (top >= end) return 0;
			r = que[top].r, c = que[top].c, d = que[top++].d;
			if (r == pp[j].r && c == pp[j].c) {	dist[i][j] = dist[j][i] = d; break; }
			for (k = 0; k < 4; k++) {
				r2 = r + m[k][0], c2 = c + m[k][1];
				if (r2 < 0 || r2 >= h || c2 < 0 || c2 >= w || map[r2][c2] == 'x') continue;
				if (mk[r2][c2]) continue;
				mk[r2][c2] = 1;
				que[end].r = r2, que[end].c = c2, que[end++].d = d+1;
			}
		}
	}
	return 1;
}

void combi(int k, int a, int w)
{
	int b;

	if (k == n) { if (w < ans) ans = w; return; }
	for (b = 1; b < n; b++) {
		if (f[b]) continue;
		if (w + dist[a][b] < ans) f[b] = 1, combi(k+1, b, w+dist[a][b]), f[b] = 0;
	}
}

int main()
{
	int i, r, c;

	while(scanf("%d%d", &w, &h) && w) {
		for (n = 1, r = 0; r < h; r++) {
			scanf("%s", map[r]);
			for (c = 0; c < w; c++) {
				if      (map[r][c] == 'o') pp[0].r = r, pp[0].c = c;
				else if (map[r][c] == '*') pp[n].r = r, pp[n++].c = c;
			}
		}
		if (n <= 1) { puts("0"); continue; }
		if (!distance()) { puts("-1"); continue; }
		memset(f, 0, sizeof(f)), ans = 1000;
		for (i = 1; i < n; i++) f[i] = 1, combi(2, i, dist[0][i]), f[i] = 0;
		printf("%d\n", ans);
	}
	return 0;
}