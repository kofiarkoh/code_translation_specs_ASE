#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_MOD			1000000007								// 除数(10の9乗+7)
#define D_ITM_MAX		200000									// 最大アイテム数
#define D_ITM_PC		0										// アイテム - PC
#define D_ITM_POW		1										// アイテム - 電源
#define D_FACT_MAX		D_ITM_MAX								// 最大階乗数

// 内部構造体 - アイテム情報
typedef struct Itm {
	int miType;													// PC or 電源
	int miPos;													// 位置
} Itm;

// 内部変数
static FILE *szpFpI;											// 入力
static Itm sz1Itm[D_ITM_MAX];									// アイテム
static int siICnt;												// アイテム数

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

// ソート関数 - 位置昇順
int
fSortFnc(
	const void *pzpVal1			// <I> 値１
	, const void *pzpVal2		// <I> 値２
)
{
	Itm *lzpVal1 = (Itm *)pzpVal1;
	Itm *lzpVal2 = (Itm *)pzpVal2;

	// 位置昇順
	if (lzpVal1->miPos > lzpVal2->miPos) {
		return 1;
	}
	else if (lzpVal1->miPos < lzpVal2->miPos) {
		return -1;
	}

	return 0;
}

// 積 - 取得
int
fGetModMulti(
	int piVal1					// <I> 値１
	, int piVal2				// <I> 値２
)
{
	return (int)((long long)piVal1 * (long long)piVal2 % D_MOD);
}

// 実行メイン
int
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// アイテム数 - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d", &siICnt);

	// PC - 取得
	for (i = 0; i < siICnt; i++) {
		sz1Itm[i].miType = D_ITM_PC;
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
		sscanf(lc1Buf, "%d", &sz1Itm[i].miPos);
	}

	// 電源 - 取得
	for (i = 0; i < siICnt; i++) {
		sz1Itm[siICnt + i].miType = D_ITM_POW;
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
		sscanf(lc1Buf, "%d", &sz1Itm[siICnt + i].miPos);
	}

	// アイテム - ソート
	siICnt *= 2;
	qsort(sz1Itm, siICnt, sizeof(Itm), fSortFnc);

	// パターン数 - 取得
	int liPCnt = 1;
	int liICnt = 0;
	int liType;
	for (i = 0; i < siICnt; i++) {

		// 個数 - 加算
		if (liICnt == 0) {
			liICnt = 1;
			liType = sz1Itm[i].miType;
		}
		else if (sz1Itm[i].miType == liType) {
			liICnt++;
		}
		else {
			liPCnt = fGetModMulti(liPCnt, liICnt);
			liICnt--;
		}
	}

	return liPCnt;
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

