#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_ARRAY_MAX		100000									// 最大配列数

// 内部構造体 - 数字情報
typedef struct Num {
	int miMod;													// 余り
	int miVal;													// 値
	int miCntM;													// 個数(同一余り)
	int miCntV;													// 個数(同一値)
} Num;

// 内部変数
static FILE *szpFpI;											// 入力
static Num sz1Num[D_ARRAY_MAX];									// 数字
static int siNCnt;												// 数字数

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

// ソート関数 - 余り昇順 - 値昇順
int
fSortFnc(
	const void *pzpVal1			// <I> 値１
	, const void *pzpVal2		// <I> 値２
)
{
	Num *lzpVal1 = (Num *)pzpVal1;
	Num *lzpVal2 = (Num *)pzpVal2;

	// 余り昇順
	if (lzpVal1->miMod > lzpVal2->miMod) {
		return 1;
	}
	else if (lzpVal1->miMod < lzpVal2->miMod) {
		return -1;
	}

	// 値昇順
	if (lzpVal1->miVal > lzpVal2->miVal) {
		return 1;
	}
	else if (lzpVal1->miVal < lzpVal2->miVal) {
		return -1;
	}

	return 0;
}

// カード使用
int
fUseCard(
	int *piCnt1					// <IO> 枚数１
	, int *piCnt2				// <IO> 枚数２
)
{
	int liCnt;
	if (*piCnt1 < *piCnt2) {
		liCnt = *piCnt1;
	}
	else {
		liCnt = *piCnt2;
	}

	// カード使用
	*piCnt1 -= liCnt;
	*piCnt2 -= liCnt;

	return liCnt;
}

// 実行メイン
int
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// 数字数・和 - 取得
	int liSum;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &siNCnt, &liSum);

	// 数字 - 取得
	for (i = 0; i < siNCnt; i++) {
		fscanf(szpFpI, "%d", &sz1Num[i].miVal);
		sz1Num[i].miMod = sz1Num[i].miVal % liSum;
		sz1Num[i].miCntV = 1;
	}
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	qsort(sz1Num, siNCnt, sizeof(Num), fSortFnc);

	// 同一値 - 圧縮
	int liTo = 0;
	for (i = 1; i < siNCnt; i++) {

		// 圧縮
		if (sz1Num[i].miVal == sz1Num[liTo].miVal) {
			sz1Num[liTo].miCntV++;
			continue;
		}

		// 不一致
		liTo++;
		sz1Num[liTo] = sz1Num[i];
	}
	siNCnt = liTo + 1;

	// 余り - 圧縮
	sz1Num[0].miCntM = sz1Num[0].miCntV % 2;
	sz1Num[0].miCntV = sz1Num[0].miCntV / 2 * 2;
	liTo = 0;
	for (i = 1; i < siNCnt; i++) {

		// 圧縮
		if (sz1Num[i].miMod == sz1Num[liTo].miMod) {
			sz1Num[liTo].miCntM += sz1Num[i].miCntV % 2;
			sz1Num[liTo].miCntV += sz1Num[i].miCntV / 2 * 2;
			continue;
		}

		// 不一致
		liTo++;
		sz1Num[liTo] = sz1Num[i];
		sz1Num[liTo].miCntM = sz1Num[liTo].miCntV % 2;
		sz1Num[liTo].miCntV = sz1Num[liTo].miCntV / 2 * 2;
	}
	siNCnt = liTo + 1;

	// 余り - 組み合わせ - ０
	int liCnt = 0;
	int liNo1 = 0;
	if (sz1Num[0].miMod == 0) {
		liCnt = (sz1Num[0].miCntM + sz1Num[0].miCntV) / 2;
	}

	// 余り - 組み合わせ - １～
	int liNo2 = siNCnt - 1;
	int liMax = (liSum - 1) / 2;
	for (i = 1; i <= liMax; i++) {

		// １つ目
		while (sz1Num[liNo1].miMod < i) {
			liNo1++;
		}
		if (sz1Num[liNo1].miMod > i) {
			continue;
		}

		// ２つ目
		int liMod = liSum - i;
		while (sz1Num[liNo2].miMod > liMod) {
			liNo2--;
		}
		if (sz1Num[liNo2].miMod < liMod) {
			continue;
		}

		// ペアのみ同士
		liCnt += fUseCard(&sz1Num[liNo1].miCntM, &sz1Num[liNo2].miCntM);

		// ペアのみと両用
		liCnt += fUseCard(&sz1Num[liNo1].miCntM, &sz1Num[liNo2].miCntV);
		liCnt += fUseCard(&sz1Num[liNo1].miCntV, &sz1Num[liNo2].miCntM);

		// 両用
		liCnt += sz1Num[liNo1].miCntV / 2;
		liCnt += sz1Num[liNo2].miCntV / 2;
	}

	// 余り - 組み合わせ - 和の半分
	if (liSum % 2 == 0) {
		int liMod = liSum / 2;
		while (sz1Num[liNo1].miMod < liMod) {
			liNo1++;
		}
		if (sz1Num[liNo1].miMod == liMod) {
			liCnt += (sz1Num[liNo1].miCntM + sz1Num[liNo1].miCntV) / 2;
		}
	}

	// 出力
	sprintf(lc1Buf, "%d\n", liCnt);
	fOutLine(lc1Buf);

	return 0;
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

