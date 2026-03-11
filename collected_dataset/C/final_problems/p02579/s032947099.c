#include <stdio.h>
#include <stdlib.h>

#define INF (1000 * 1000 * 99)

struct node_t {
	int y, x;
	int cost;
};

int mofu = 0;
struct node_t nyan[1000 * 1000 * 5];

void qa(int idx) {
	for (;;) {
		int midx = idx;
		int i;
		for (i = 1; i <= 2; i++) {
			int cidx = idx * 2 + i;
			if (cidx < mofu && nyan[midx].cost > nyan[cidx].cost) midx = cidx;
		}
		if (midx == idx) {
			if (idx == 0) break;
			idx = (idx - 1) / 2;
		} else {
			struct node_t temp = nyan[idx];
			nyan[idx] = nyan[midx];
			nyan[midx] = temp;
			idx = midx;
		}
	}
}

void qpush(int y, int x, int cost) {
	nyan[mofu].y = y;
	nyan[mofu].x = x;
	nyan[mofu].cost = cost;
	mofu++;
	qa(mofu - 1);
}

struct node_t qpop(void) {
	struct node_t res;
	if (mofu <= 0) {
		puts("empty");
		exit(2);
	}
	res = nyan[0];
	mofu--;
	if (mofu > 0) {
		nyan[0] = nyan[mofu];
		qa(0);
	}
	return res;
}

int H, W;
int Ch, Cw;
int Dh, Dw;
char S[1024][1024];

int mincost[1024][1024];
char kakutei[1024][1024];

int main(void) {
	int i, j;
	if (scanf("%d%d", &H, &W) != 2) return 1;
	if (scanf("%d%d", &Ch, &Cw) != 2) return 1;
	if (scanf("%d%d", &Dh, &Dw) != 2) return 1;
	for (i = 0; i < H; i++) {
		if (scanf("%1023s", S[i]) != 1) return 1;
		for (j = 0; j < W; j++) mincost[i][j] = INF;
	}
	qpush(Ch - 1, Cw - 1, 0);
	mincost[Ch - 1][Cw - 1] = 0;
	while (mofu > 0) {
		struct node_t cur = qpop();
		int dy, dx;
		if (kakutei[cur.y][cur.x]) continue;
		for (dy = -2; dy <= 2; dy++) {
			for (dx = -2; dx <= 2; dx++) {
				int ny = cur.y + dy, nx = cur.x + dx;
				int ax = abs(dy), ay = abs(dx);
				int waapu = !(ax + ay <= 1);
				int ncost = mincost[cur.y][cur.x] + waapu;
				if (0 <= ny && ny < H && 0 <= nx && nx < W && S[ny][nx] == '.' && ncost < mincost[ny][nx]) {
					mincost[ny][nx] = ncost;
					qpush(ny, nx, ncost);
				}
			}
		}
	}
	printf("%d\n", mincost[Dh - 1][Dw - 1] < INF ? mincost[Dh - 1][Dw - 1] : -1);
	return 0;
}
