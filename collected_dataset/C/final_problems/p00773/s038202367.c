#include <stdio.h>

int main(void){
	int x;  /* 変更前の消費税率 % */
	int y;  /* 変更後の消費税率 % */
	int s;  /* 消費税率変更前の 2 商品の税込合計価格 */
	int a, b, a0, b0, total, max;

	while(scanf("%d %d %d", &x, &y, &s) == 3){
		if (x == 0 && y == 0 && s == 0){
			break;
		}

		max = 0;

		/* すべての場合について計算し最大価格を求める */
		for (a = 1; a <= s / 2; a++){
			b = s - a;

			a0 = a * 100 / (100 + x);
			b0 = b * 100 / (100 + x);

			/* 個々の商品の税抜価格について、とりうる最大値を求める */
			while ((a0 + 1) * (100 + x) / 100 <= a){
				a0++;
			}
			while ((b0 + 1) * (100 + x) / 100 <= b){
				b0++;
			}

			/* 変更前の合計金額が成立しない組み合わせは棄却する */
			if (a0 * (100 + x) / 100 + b0 * (100 + x) / 100 != s){
				continue;
			}

			/* 変更後の合計金額を計算する */
			total = a0 * (100 + y) / 100 + b0 * (100 + y) / 100;

			/* 最大値を判定する */
			if (total > max){
				max = total;
			}
		}

		/* 出力 */
		printf("%d\n", max);
	}

    return 0;
}