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
#define D_LIGHT_MAX		10										// 最大電球数
#define D_SW_MAX		10										// 最大スイッチ数

// 内部構造体 - 電球情報
typedef struct Light {
	int mi1Sw[D_SW_MAX];										// スイッチ
	int miSCnt;													// スイッチ数
	int miP;													// 点灯条件
} Light;

// 内部変数
static FILE *szpFpI;											// 入力
static int siSCnt;												// スイッチ数
static Light sz1Light[D_LIGHT_MAX];								// 電球
static int siLCnt;												// 電球数
static int siPCnt;												// パターン数

// 内部変数 - テスト用
#ifdef D_TEST
	static int siRes;
	static FILE *szpFpA;
	static int siTNo;
#endif

// 出力
int
fOut(
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

// スイッチ - 選択
int
fSelSw(
	int piSNo					// <I> スイッチ 0～
	, int *pipSw				// <I> 選択スイッチ
)
{
	int i, j;

	// 選択中
	if (piSNo < siSCnt) {

		// スイッチ - ON
		pipSw[piSNo] = D_ON;
		fSelSw(piSNo + 1, pipSw);

		// スイッチ - OFF
		pipSw[piSNo] = D_OFF;
		fSelSw(piSNo + 1, pipSw);

		return 0;
	}

	// 点灯 - チェック
	for (i = 0; i < siLCnt; i++) {
		int liCnt = 0;
		for (j = 0; j < sz1Light[i].miSCnt; j++) {
			if (pipSw[sz1Light[i].mi1Sw[j]] == D_ON) {
				liCnt++;
			}
		}
		if (liCnt % 2 != sz1Light[i].miP) {
			return 0;
		}
	}

	// パターン数
	siPCnt++;

	return 0;
}

// 実行メイン
int
fMain(
)
{
	int i, j;
	char lc1Buf[1024];

	// スイッチ数・電球数 - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &siSCnt, &siLCnt);

	// 電球 - スイッチ - 取得
	for (i = 0; i < siLCnt; i++) {
		fscanf(szpFpI, "%d", &sz1Light[i].miSCnt);
		for (j = 0; j < sz1Light[i].miSCnt; j++) {
			fscanf(szpFpI, "%d", &sz1Light[i].mi1Sw[j]);
			sz1Light[i].mi1Sw[j]--;
		}
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	}

	// 電球 - 点灯条件 - 取得
	for (i = 0; i < siLCnt; i++) {
		fscanf(szpFpI, "%d", &sz1Light[i].miP);
	}
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);

	// スイッチ - 選択
	int li1Sw[D_SW_MAX];
	fSelSw(0, li1Sw);

	return 0;
}

// １回実行
int
fOne(
)
{
	int liRet;
	char lc1Buf[1024];

	// データ - 初期化
	siPCnt = 0;													// パターン数

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
	sprintf(lc1Buf, "%d\n", siPCnt);
	fOut(lc1Buf);

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

