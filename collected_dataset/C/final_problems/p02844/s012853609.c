#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_ON			1										// 汎用フラグ - ON
#define D_OFF			0										// 汎用フラグ - OFF
#define D_CHAR_MAX		30005									// 最大文字数

// 内部変数
static FILE *szpFpI;											// 入力
static char sc1Str[D_CHAR_MAX];									// 文字列
static int siCCnt;												// 文字数
static int si1Num[10];											// １文字
static int si2Num[10][10];										// ２文字
static int si3Num[10][10][10];									// ３文字

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
	fgets(lc1Buf, sizeof(lc1Buf), szpFpA);
	if (strcmp(lc1Buf, pcpLine)) {
		siRes = -1;
	}
#else
	printf("%s", pcpLine);
#endif

	return 0;
}

// 実行メイン
int
fMain(
)
{
	int i, j, k;
	char lc1Buf[1024];

	// 文字数 - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d", &siCCnt);

	// 文字列 - 取得
	fgets(sc1Str, sizeof(sc1Str), szpFpI);

	// 各文字チェック
	for (i = 0; i < siCCnt; i++) {
		int liNum = sc1Str[i] - '0';

		// ２文字 -> ３文字
		for (j = 0; j < 10; j++) {
			for (k = 0; k < 10; k++) {
				if (si2Num[j][k] == D_ON) {
					si3Num[j][k][liNum] = D_ON;
				}
			}
		}

		// １文字 -> ２文字
		for (j = 0; j < 10; j++) {
			if (si1Num[j] == D_ON) {
				si2Num[j][liNum] = D_ON;
			}
		}

		// １文字
		si1Num[liNum] = D_ON;
	}

	// ３文字の種類数 - 取得
	int liCnt = 0;
	for (i = 0; i < 10; i++) {
		for (j = 0; j < 10; j++) {
			for (k = 0; k < 10; k++) {
				if (si3Num[i][j][k] == D_ON) {
					liCnt++;
				}
			}
		}
	}

	return liCnt;
}

// １回実行
int
fOne(
)
{
	int liRet;
	char lc1Buf[1024];

	// データ - 初期化
	memset(si1Num, 0, sizeof(si1Num));							// １文字
	memset(si2Num, 0, sizeof(si2Num));							// ２文字
	memset(si3Num, 0, sizeof(si3Num));							// ３文字

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

