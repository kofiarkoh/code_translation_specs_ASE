#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_SIZE_MAX		2005									// 最大サイズ
#define D_WAY_CNT		2										// 方向数
#define D_WAY_RIGHT		0										// 方向 - 右
#define D_WAY_DOWN		1										// 方向 - 下

// 内部変数
static FILE *szpFpI;											// 入力
static int siW, siH;											// 幅・高さ
static char sc2Mass[D_SIZE_MAX][D_SIZE_MAX];					// マス[高さ][幅]
static int si3LCnt[D_SIZE_MAX][D_SIZE_MAX][D_WAY_CNT];			// 光数[高さ][幅][方向]

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

// 最大値 - 取得
int
fGetMax(
	int piVal1					// <I> 値１
	, int piVal2				// <I> 値２
)
{
	if (piVal1 > piVal2) {
		return piVal1;
	}
	else {
		return piVal2;
	}
}

// 光数 - セット
int
fSetLCnt(
	int piPx					// <I> 位置X
	, int piPy					// <I> 位置Y
	, int piWay					// <I> 方向
	, int piMx					// <I> 移動方向X
	, int piMy					// <I> 移動方向Y
	, int piLCnt				// <I> 光数
)
{
	// 範囲チェック
	if (piPx >= siW) {
		return piLCnt;
	}
	if (piPy >= siH) {
		return piLCnt;
	}

	// セット済
	if (si3LCnt[piPy][piPx][piWay] > 0) {
		return piLCnt;
	}

	// マス
	if (sc2Mass[piPy][piPx] != '.') {
		return piLCnt;
	}

	// 光数 - セット
	si3LCnt[piPy][piPx][piWay] = fSetLCnt(piPx + piMx, piPy + piMy, piWay, piMx, piMy, piLCnt + 1);

	return si3LCnt[piPy][piPx][piWay];
}

// 実行メイン
int
fMain(
)
{
	int i, j;
	char lc1Buf[1024];

	// 幅・高さ - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &siH, &siW);

	// マス - 取得
	for (i = 0; i < siH; i++) {
		fgets(sc2Mass[i], sizeof(sc2Mass[0]), szpFpI);
	}

	// 光数 - セット
	int liMax = 0;
	for (i = 0; i < siH; i++) {
		for (j = 0; j < siW; j++) {
			fSetLCnt(j, i, D_WAY_RIGHT, 1, 0, 0);
			fSetLCnt(j, i, D_WAY_DOWN, 0, 1, 0);
			liMax = fGetMax(liMax, si3LCnt[j][i][D_WAY_RIGHT] + si3LCnt[j][i][D_WAY_DOWN]);
		}
	}

	return liMax - 1;
}

// １回実行
int
fOne(
)
{
	int liRet;
	char lc1Buf[1024];

	// データ - 初期化
	memset(si3LCnt, 0, sizeof(si3LCnt));						// 光数

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

