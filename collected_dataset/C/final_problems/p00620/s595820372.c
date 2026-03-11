// AOJ 1034: Line Puzzle
// 2017.9.29 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 8
#define N2 66

typedef struct { signed char r, c, s; } T;
T start[N2]; int m;

signed char map[N][N]; int n, n2;
int mv[4][2] = {{-1,0},{1,0},{0,1},{0,-1}};

int cmp(T *a, T *b)
{
	if (b->s - a->s) return b->s - a->s;
	if (b->r - a->r) return b->r - a->r;
	return a->c - b->c;
}

void route(signed char a[N][N], signed char b[N][N][2], int r, int c)
{
	int r2, c2;

	while (a[r][c] > 0) {
		a[r][c] = 0;
		r2 = b[r][c][0], c2 = b[r][c][1];
		r = r2, c = c2;
	}
}

void print(signed char a[N][N])
{
	int r, c;

	for (r = 0; r < n; r++) {
		for (c = 0; c < n; c++) printf("%3d ", a[r][c]);
		printf("\n");
	}
	printf("\n");
}

int combi(signed char a[N][N], int m0, int n0)
{
	int i, id, r, c, s, r2, c2, ss, goal;
	signed char tmp[N][N], prev[N][N][2], mk[N][N][N][N];
	T q[N2]; int top;

	if (m0 >= m && n0 >= n2) return 1;
	memset(mk, 0, sizeof(mk));
	q[0].r = start[m0].r, q[0].c = start[m0].c, q[0].s = 0, top = 1;
	goal = start[m0].s, id = 0;
	while (top > 0) {
		r = q[--top].r, c = q[top].c, s = q[top].s;
		for (i = 0; i < 4; i++) {
			r2 = r + mv[i][0], c2 = c + mv[i][1], id++;
			if (r2 < 0 || r2 >= n || c2 < 0 || c2 >= n ||
				mk[r][c][r2][c2] || a[r2][c2] <= 0) continue;
			ss = s + a[r2][c2];
			mk[r][c][r2][c2] = 1;
			if (ss < goal) {
				prev[r2][c2][0] = r, prev[r2][c2][1] = c;
				q[top].r = r2, q[top].c = c2, q[top++].s = ss;
			} else if (ss == goal) {
				memcpy(tmp, a, sizeof(tmp));
				tmp[r2][c2] = 0;
				route(tmp, prev, r, c);
//printf("m %d, n %d ", m0+1, n0+id+1); print(tmp);
				if (combi(tmp, m0+1, n0+id+1)) return 1;
			}
		}
	}
	return 0;
}

int main()
{
	int r, c, t, s;

	while (scanf("%d", &n) && n > 0) {
		n2 = n*n;
		for (s = m = 0, r = 0; r < n; r++) for (c = 0; c < n; c++) {
			scanf("%d", &t);
			if (t < 0) start[m].r = r, start[m].c = c, start[m++].s = -t;
			map[r][c] = t, s += t;
		}
//		qsort(start, m, sizeof(T), cmp);
		puts(s == 0 && combi(map, 0, 0) ? "YES" : "NO");
	}
	return 0;
}