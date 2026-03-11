#include <stdio.h>
#include <limits.h>

/* 命令 */
int STRAIGHT = 0;
int RIGHT = 1; /* 右に90度回転し直進 */
int BACK = 2; /* 右に180度回転し直進 */
int LEFT = 3; /* 右に270度回転し直進 */
int HALT = 4;
int OUT = 5; /* 盤面外 */
/* 各方向を向いているときの移動ベクトル (dx,dy) (北, 東, 南, 西) */
int MOVEMENT[4][2] = { {0, -1}, {1, 0}, {0, 1}, {-1, 0} };
/* 入力された条件 */
int w, h, s[30][30]; /* 盤面 */
int c[4]; /* 定められたコスト */

/* 指定された座標のブロックを返す */
int getCell(int x, int y) {
	if (x < 0 || x >= w || y < 0 || y >= h) return OUT;
	return s[y][x];
}

/* 幅優先探索 */
int memo[30][30][4]; /* 探索のメモ (あるマスにある向きで到達するまでの最小コスト) */
int UNREACHED = INT_MAX;
typedef struct { int cost; int x; int y; int angle; } Log; /* 探索状態 */

/* FIFOキュー */
typedef Log Data;
Data queue[2000]; int queue_size = 2000; int queue_head; int queue_length;
void initqueue() {
	queue_length = 0;
}
void enqueue(Data d) {
	queue[(queue_head + queue_length) % queue_size] = d;
	queue_length++;
}
Data dequeue() {
	Data d = queue[queue_head];
	queue_head = (queue_head + 1) % queue_size;
	queue_length--;
	return d;
}

/* 幅優先探索をしてゴールのマスに到達するまでに掛かったコストを返す */
int search(Log l) {
	int min = UNREACHED;
	int i;
	/* この試行が現在いるブロックまでの最短経路か確認する */
	if (l.cost > memo[l.y][l.x][l.angle]) return UNREACHED;

	/* 次の動作 */
	for (i = 0; i < 4; i++) {
		/* 次の動作を行った場合の状態を作る */
		Log next;
		next.angle = (l.angle + i) % 4;
		next.x = l.x + MOVEMENT[next.angle][0];
		next.y = l.y + MOVEMENT[next.angle][1];
		next.cost = l.cost;
		if (i != getCell(l.x, l.y)) next.cost += c[i]; /* マスの命令に従わない場合のみコストが掛かる */

		int cell = getCell(next.x, next.y); /* 次に踏むマス */

		if (/* cell == HALT && */ next.x == w - 1 && next.y == h - 1) {
			/* ゴールに到達 */
			if (next.cost < min) min = next.cost;
		} else if (cell == OUT) {
			/* 盤面外に前進し失格 */
			continue;
		} else {
			/* 次に探索する状態をキューに入れる */
			if (next.cost < memo[next.y][next.x][next.angle]) {
				memo[next.y][next.x][next.angle] = next.cost;
				enqueue(next);
			}
		}
	}

	return min;
}

int main(void) {
	int min, i, j, k;

	/* データセット毎の処理 */
	while (scanf("%d%d", &w, &h) == 2 && w > 0 && h > 0) {
		/* 各マスに割り当てられた命令を読み込む */
		for (i = 0; i < h; i++) for (j = 0; j < w; j++) {
			scanf("%d", &s[i][j]);

			/* メモを初期化する */
			for (k = 0; k < 4; k++) memo[i][j][k] = UNREACHED;
		}
		/* コストを読み込む */
		for (i = 0; i < 4; i++) scanf("%d", &c[i]);

		/* 探索の始点をキューに入れる */
		Log l = {0, 0, 0, 1}; /* 北西隅のマスで東向き */
		enqueue(l);

		/* 幅優先探索する */
		min = UNREACHED;
		while (queue_length > 0) {
			int cost = search(dequeue());
			if (cost < min) min = cost;
		}

		/* 結果を出力する */
		printf("%d\n", min);
	}

	return 0;
}