#include <stdio.h>
#include <string.h>

int card[3][10];

void format_card(void);
int check(void);

int main(void) {
	char color[10];
	int value[9];
	int i, j, num, flag = 0;
	scanf("%d", &num);										//データセット数の取得
	getchar();												//改行の読み飛ばし
	if (num < 0) { num = 0; }								//下限値のチェック
	if (num > 50) { num = 50; }								//上限値のチェック
	for (i = 0; i < num; i++) {
		format_card();										//配列の初期化
		scanf("%d%d%d%d%d%d%d%d%d", &value[0], &value[1], &value[2], &value[3], &value[4], &value[5], &value[6], &value[7], &value[8]);
		getchar();
		scanf("%c %c %c %c %c %c %c %c %c", &color[0], &color[1], &color[2], &color[3], &color[4], &color[5], &color[6], &color[7], &color[8]);
		getchar();
		for (j = 0; j < 9; j++) {							//配列へ格納
			switch (color[j]) {
			case 'R':
				card[0][value[j] - 1] ++;
				break;
			case 'G':
				card[1][value[j] - 1] ++;
				break;
			case 'B':
				card[2][value[j] - 1] ++;
				break;
			default:
				break;
			}
		}
		flag = check();
		if (flag == 3) {
			printf("1\n");
		}
		else {
			printf("0\n");
		}
	}
	return 0;
}

int check(void) {
	int i, j, flag = 0;
	for (i = 0; i < 3; i++) {							//オーバーのチェック
		for (j = 0; j < 9; j++) {
			if (card[i][j] > 4) {
				card[i][j] = 4;
				flag++;
			}
		}
	}
	for (i = 0; i < 3; i++) {							//連番のチェック
		j = 0;
		do {
			if (card[i][j] > 0 && card[i][j + 1] > 0 && card[i][j + 2] > 0) {
				card[i][j]--, card[i][j + 1]--, card[i][j + 2]--;
				flag++;
			}
			else {
				j++;									//連番が無い場合は次へ
			}
		} while (j < 9 - 2);
	}
	for (i = 0; i < 3; i++) {							//同数のチェック
		for (j = 0; j < 9; j++) {
			if (card[i][j] > 2) {
				card[i][j] -= 3;
				flag++;
			}
		}
	}
	return flag;
}
void format_card(void) {
	int i, j;
	for (i = 0; i < 3; i++) {
		for (j = 0; j < 9; j++) {
			card[i][j] = 0;
		}
	}
}

