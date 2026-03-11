// AOJ 3042: Gridgedge
// 2019.2.22 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007
#define QMAX 502*502
typedef struct { int r, c, cost; } Q;
Q q[QMAX+2];
int top, end;
int R, C;
int MAP[502][502], COST[502][502];
int d[4][2] = { {-1,0},{1,0},{0,-1},{0,1} };

void job(int r, int c, int cost, int combi)
{
	if (COST[r][c] > 0 && COST[r][c] < cost) return;
	if (COST[r][c] == 0) {
		q[end].r = r, q[end].c = c, q[end].cost = cost;
		if (++end == QMAX) end = 0;
		COST[r][c] = cost;
	}
	MAP[r][c] += combi;
	if (MAP[r][c] >= MOD) MAP[r][c] -= MOD;
}

int main()
{
	int sr, sc, gr, gc;
	int i, r, rr, c, cc, cost, combi;

	scanf("%d%d%d%d%d%d", &R, &C, &sr, &sc, &gr, &gc);
	top = 0, end = 1;
	q[0].r = sr, q[0].c = sc, q[0].cost = 1, MAP[sr][sc] = 1, COST[sr][sc] = 1;
	while (top != end) {
		r = q[top].r, c = q[top].c, cost = q[top].cost, combi = MAP[r][c];
		if (r == gr && c == gc) {
			printf("%d %d\n", cost-1, combi);
			break;
		}
		cost++;
		if (++top == QMAX) top = 0;
		for (i = 0; i < 4; i++) {
			rr = r + d[i][0], cc = c + d[i][1];
			if (rr >= 0 && cc >= 0 && rr < R && cc < C)
				job(rr, cc, cost, combi);
		}
		if (r > 0  ) job(0,   c,   cost, combi);
		if (r < R-1) job(R-1, c,   cost, combi);
		if (c > 0  ) job(r,   0,   cost, combi);
		if (c < C-1) job(r,   C-1, cost, combi);
	}
	return 0;
}

