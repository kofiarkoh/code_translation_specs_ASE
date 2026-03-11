#include <stdio.h>

char map[20][21];											//高さ * 幅
int h, w;													//マップの高さ、幅
int px, py, dir = 0;										//自分の座標、方向
int dirx[4] = { 0,0,-1,1 };
int diry[4] = { -1,1,0,0 };
char my[4] = { '^', 'v', '<', '>' };

void act(char *n);
void move(int x);
void shot(int x);
void format(int x, int y);
void view(void);

int main(void) {
	format(20, 20);
	int i, j, k;
	int data;												//繰り返し回数
	int nmax;												//操作列の長さ
	char n[101];											//操作列の格納用
	scanf("%d\n", &data);									//繰り返し回数の取得
	for (k = 0; k < data; k++) {
		scanf("%d %d\n", &h, &w);							//高さと幅の取得
		for (i = 0; i < h; i++) {
			for (j = 0; j < w; j++) {
				map[i][j] = getchar();
				if (map[i][j] == '^') { px = j, py = i, dir = 0; }
				if (map[i][j] == 'v') { px = j, py = i, dir = 1; }
				if (map[i][j] == '<') { px = j, py = i, dir = 2; }
				if (map[i][j] == '>') { px = j, py = i, dir = 3; }
			}
			getchar();
		}
		scanf("%d\n", &nmax);								//操作列の長さの取得
		if (nmax < 0) { nmax = 0; }
		if (nmax > 100) { nmax = 100; }
		for (i = 0; i < nmax; i++) {						//操作列の取得
			n[i] = getchar();
		}
		n[i] = '\0';
		act(&n[0]);											//行動処理
		view();												//結果の表示
		if (k + 1 != data) { printf("\n"); }
	}
	return 0;
}
void act(char *str) {										//行動処理
	while (*str != '\0') {
		switch (*str) {
		case 'U':
			dir = 0;
			move(dir);
			break;
		case 'D':
			dir = 1;
			move(dir);
			break;
		case 'L':
			dir = 2;
			move(dir);
			break;
		case 'R':
			dir = 3;
			move(dir);
			break;
		case 'S':
			shot(dir);
			break;
		default:
			break;
		}
		str++;
	}
}
void move(int x) {											//移動処理
	int sx = px + dirx[x], sy = py + diry[x], flag = 0;
	if (sx < 0 || sx >= w) { flag = 1; }
	if (sy < 0 || sy >= h) { flag = 1; }
	if (map[sy][sx] == '.' && flag == 0) {
		map[py][px] = '.';									//自分のいた場所を平地へ変更する
		px = sx, py = sy;									//座標を移動する
	}
	map[py][px] = my[dir];
}
void shot(int x) {											//砲弾処理
	int sx = px + dirx[x], sy = py + diry[x] , flag;
	do {													//移動可能な限り繰り返す
		flag = 0;
		if (sx < 0 || sx >= w) { break; }					//幅の範囲外は見ない
		if (sy < 0 || sy >= h) { break; }					//高さの範囲外は見ない
		switch (map[sy][sx]) {
			case '*':										//レンガの壁の場合
				map[sy][sx] = '.';							//平地に変更する
				break;
			case '#':										//鉄の壁の場合
				break;
			default:										//障害物以外の場合
				flag = 1;									//繰り返す
				break;
		}
		sx += dirx[x];
		sy += diry[x];
	} while (flag != 0);
}
void format(int x, int y) {
	int i, j;
	for (i = 0; i < x; i++) {
		for (j = 0; j < y; j++) {
			map[i][j] = '0';
		}
	}
}
void view(void) {
	int i, j;
	for (i = 0; i < h; i++) {
		for (j = 0; j < w; j++) {
			printf("%c", map[i][j]);
		}
		printf("\n");
	}
}

