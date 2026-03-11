#include <stdio.h>

//問題文より、入力される表の最大の大きさ(100)+1の大きさを要素数とする
#define MAX_ROW 101
#define MAX_COLUMN 101

//関数プロトタイプ宣言
void set_sum(long int body[MAX_ROW][MAX_COLUMN], int row, int column);

//表の構造体
typedef struct{
	int row;
	int column;
	long int body[MAX_ROW][MAX_COLUMN];
}Table;

int main(void)
{
	//メンバーを0で初期化
	Table new_table = {0, 0, 0};

	//ループ変数
	int i, j;

	//表の大きさを入力
	scanf("%d %d", &new_table.row, &new_table.column);

	//表のデータを入力
	for(i = 0; i < new_table.row; i++){
		for(j = 0; j < new_table.column; j++){
			scanf("%ld", &(new_table.body[i][j]));
		}
	}
	//各行列の合計値をセット
	set_sum(new_table.body, new_table.row, new_table.column);

	//表のデータを出力
	for(i = 0; i < (new_table.row + 1); i++){
		for(j = 0; j < new_table.column; j++){
			printf("%ld ", new_table.body[i][j]);
		}
		//最後の数値の後にスペースがある場合はエラーとなる
		printf("%ld\n", new_table.body[i][j]);
	}

	return 0;
}

//表に合計値を挿入する関数
void set_sum(long int body[MAX_ROW][MAX_COLUMN], int row, int column)
{
	//ループ変数
	int i, j;

	for(i = 0; i < row; i++){
		for(j = 0; j < column; j++){
			//行の合計値
			body[i][column] += body[i][j];
			//列の合計値
			body[row][j] += body[i][j];
		}
		//行列全体の合計値
		body[row][column] += body[i][column];
	}
}
