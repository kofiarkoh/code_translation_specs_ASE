#include <stdio.h>
#define IMPOSSIBLE -1

int main(void) {
	int n, m, sn[100], sm[100]; /* 与えられる値 */
	int i, j;

	/* データセット毎の処理 */
	while (scanf("%d%d", &n, &m) == 2 && n > 0 && m > 0) {
		int tn = 0, tm = 0; /* 現在の合計点数 */
		int an = IMPOSSIBLE, am = IMPOSSIBLE; /* 交換すべきカード */

		/* 各カードの点数を入力する */
		for (i = 0; i < n; i++) {
			scanf("%d", sn + i);
			tn += sn[i];
		}
		for (i = 0; i < m; i++) {
			scanf("%d", sm + i);
			tm += sm[i];
		}

		/* 交換すべきカードを総当たりで探す */
		for (i = 0; i < n; i++) {
			for (j = 0; j < m; j++) {
				/* 交換によって合計点数が等しくなるか */
				if ((sn[i] - sm[j]) * 2 == tn - tm) {
					/* 交換するカードの点数の和が最小か */
					if (an == IMPOSSIBLE || am == IMPOSSIBLE || sn[i] + sm[j] < an + am) {
						an = sn[i];
						am = sm[j];
					}
				}
			}
		}

		/* 結果を出力する */
		if (an == IMPOSSIBLE || am == IMPOSSIBLE) printf("%d\n", IMPOSSIBLE);
		else printf("%d %d\n", an, am);
	}

	return 0;
}