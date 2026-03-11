#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_SIZE_MAX		305										// 最大サイズ

// 内部変数
static FILE *szpFpI;											// 入力
static int siW, siH;											// 幅・高さ
static char sc2Block[D_SIZE_MAX][D_SIZE_MAX];					// ブロック
static int si2Cost[D_SIZE_MAX][D_SIZE_MAX];						// コスト[列１の高さ][列２の高さ]
static int si2Sum[D_SIZE_MAX][D_SIZE_MAX];						// 合計コスト[列１の高さ][列２の高さ]

// 内部変数 - テスト用
#ifdef D_TEST
	static int siRes;
	static FILE *szpFpA;
	static int siTNo;
#endif

// １行出力
int
fOutLine(
	char *pcpLine				// <I> １行
)
{
	char lc1Buf[1024];

#ifdef D_TEST
	lc1Buf[0] = '\0';
	fgets(lc1Buf, sizeof(lc1Buf), szpFpA);
	if (strcmp(lc1Buf, pcpLine)) {
		siRes = -1;
	}
#else
	printf("%s", pcpLine);
#endif

	return 0;
}

// 最小値 - 取得
int
fGetMin(
	int piVal1					// <I> 値１
	, int piVal2				// <I> 値２
)
{
	if (piVal1 < piVal2) {
		return piVal1;
	}
	else {
		return piVal2;
	}
}

// コスト - セット
int
fSetCost(
	int piCol					// <I> 列１
	, int piRow1				// <I> 列１高さ
	, int piRow2				// <I> 列２高さ
)
{
	// 比較
	si2Cost[piRow1][piRow2] = 0;
	while (1) {
		piRow1++;
		if (piRow1 > siH) {
			break;
		}
		piRow2++;
		if (piRow2 > siH) {
			break;
		}

		// 比較
		si2Cost[piRow1][piRow2] = si2Cost[piRow1 - 1][piRow2 - 1];
		if (sc2Block[piRow1 - 1][piCol] == sc2Block[piRow2 - 1][piCol + 1]) {
			si2Cost[piRow1][piRow2]++;
		}
	}

	return 0;
}

// 実行メイン
int
fMain(
)
{
	int i, j, k;
	char lc1Buf[1024];

	// 幅・高さ - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &siH, &siW);

	// ブロック - 取得
	for (i = 0; i < siH; i++) {
		fgets(sc2Block[i], sizeof(sc2Block[0]), szpFpI);
	}

	// 総和 - 取得
	int liSum = 0;
	for (i = 0; i < siW - 1; i++) {

		// コスト - セット
		for (j = 0; j <= siH; j++) {
			fSetCost(i, j, 0);
		}
		for (j = 1; j <= siH; j++) {
			fSetCost(i, 0, j);
		}

		// 合計コスト - 初期化
		for (j = 0; j <= siH; j++) {
			for (k = 0; k <= siH; k++) {
				si2Sum[j][k] = INT_MAX;
			}
		}

		// 作業数でループ
		si2Sum[siH][siH] = 0;
		for (j = 0; j < siH * 2; j++) {

			// 開始高さ
			int liH1, liH2;
			if (j <= siH) {
				liH1 = siH;
				liH2 = liH1 - j;
			}
			else {
				liH1 = siH * 2 - j;
				liH2 = 0;
			}

			// 開始高さの種類でループ
			while (1) {

				// コスト
				int liCost = si2Sum[liH1][liH2] + si2Cost[liH1][liH2];

				// 列１
				if (liH1 > 0) {
					si2Sum[liH1 - 1][liH2] = fGetMin(si2Sum[liH1 - 1][liH2], liCost);
				}

				// 列２
				if (liH2 > 0) {
					si2Sum[liH1][liH2 - 1] = fGetMin(si2Sum[liH1][liH2 - 1], liCost);
				}

				// 次回用
				liH1--;
				if (liH1 < 0) {
					break;
				}
				liH2++;
				if (liH2 > siH) {
					break;
				}
			}
		}

		// 総和 - 加算
		liSum += si2Sum[0][0];
	}

	return liSum;
}

// １回実行
int
fOne(
)
{
	int liRet;
	char lc1Buf[1024];

	// 入力 - セット
#ifdef D_TEST
	sprintf(lc1Buf, ".\\Test\\T%d.txt", siTNo);
	szpFpI = fopen(lc1Buf, "r");
	sprintf(lc1Buf, ".\\Test\\A%d.txt", siTNo);
	szpFpA = fopen(lc1Buf, "r");
	siRes = 0;
#else
	szpFpI = stdin;
#endif

	// 実行メイン
	liRet = fMain();

	// 出力
	sprintf(lc1Buf, "%d\n", liRet);
	fOutLine(lc1Buf);

	// 残データ有無
#ifdef D_TEST
	lc1Buf[0] = '\0';
	fgets(lc1Buf, sizeof(lc1Buf), szpFpA);
	if (strcmp(lc1Buf, "")) {
		siRes = -1;
	}
#endif

	// テストファイルクローズ
#ifdef D_TEST
	fclose(szpFpI);
	fclose(szpFpA);
#endif

	// テスト結果
#ifdef D_TEST
	if (siRes == 0) {
		printf("OK %d\n", siTNo);
	}
	else {
		printf("NG %d\n", siTNo);
	}
#endif

	return 0;
}

// プログラム開始
int
main()
{

#ifdef D_TEST
	int i;
	for (i = D_TEST_SNO; i <= D_TEST_ENO; i++) {
		siTNo = i;
		fOne();
	}
#else
	fOne();
#endif

	return 0;
}

