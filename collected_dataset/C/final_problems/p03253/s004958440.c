#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_MOD			1000000007								// 除数(10の9乗+7)
#define D_PRM_MAX		100										// 最大素数数
#define D_FACT_MAX		100100									// 最大階乗数

// 内部構造体 - 素数情報
typedef struct Prm {
	int miVal;													// 値
	int miCnt;													// 個数
} Prm;

// 内部変数
static FILE *szpFpI;											// 入力
static int siN;													// N
static int siM;													// M
static Prm sz1Prm[D_PRM_MAX];									// 素数
static int siPCnt;												// 素数数
static int si1ModFact[D_FACT_MAX];								// 階乗リスト

// 内部変数 - テスト用
#ifdef D_TEST
	static int siRes;
	static FILE *szpFpA;
#endif

// 素因数分解
int
fDivPrm(
)
{
	int i;

	// 上限
	int liLimit = (int)sqrt((double)siM);

	// 素因数分解
	for (i = 2; i <= liLimit; i++) {

		// 分解終了
		if (siM == 1) {
			break;
		}

		// 分解
		while (siM % i == 0) {
			siM /= i;
			sz1Prm[siPCnt].miCnt++;
		}

		// 分解あり
		if (sz1Prm[siPCnt].miCnt > 0) {
			sz1Prm[siPCnt].miVal = i;
			siPCnt++;
		}
	}

	// 上限を超える素数
	if (siM != 1) {
		sz1Prm[siPCnt].miCnt = 1;
		sz1Prm[siPCnt].miVal = siM;
		siPCnt++;
	}

	return 0;
}

// 階乗リスト - 作成
int
fMakeModFact(
	int piMax					// <I> 最大値
)
{
	int i;

	si1ModFact[0] = 1;
	si1ModFact[1] = 1;
	for (i = 2; i <= piMax; i++) {
		si1ModFact[i] = (int)((long long)si1ModFact[i - 1] * (long long)i % (long long)D_MOD);
	}

	return 0;
}

// べき乗 - 取得
int
fGetModPower(
	int piBase					// <I> 基数
	, int piIdx					// <I> 指数
)
{
	// 平方リスト - 作成
	int li1Val[100];
	li1Val[0] = piBase;
	int liCnt = 1;
	int liIdx = 1;
	while (piIdx > liIdx) {
		li1Val[liCnt] = (int)((long long)li1Val[liCnt - 1] * (long long)li1Val[liCnt - 1] % (long long)D_MOD);
		liCnt++;
		liIdx += liIdx;
	}

	// べき乗 - 取得
	int liVal = 1;
	while (piIdx > 0) {
		if (piIdx >= liIdx) {
			piIdx -= liIdx;
			liVal = (int)((long long)liVal * (long long)li1Val[liCnt - 1] % (long long)D_MOD);
		}
		liCnt--;
		liIdx /= 2;
	}

	return liVal;
}

// 実行メイン
int
fMain(
	int piTNo					// <I> テスト番号 1～
)
{
	int i;
	char lc1Buf[1024], lc1Out[1024];

	// データ - 初期化
	memset(sz1Prm, 0, sizeof(sz1Prm));							// 素数
	siPCnt = 0;													// 素数数

	// 入力 - セット
#ifdef D_TEST
	sprintf(lc1Buf, ".\\Test\\T%d.txt", piTNo);
	szpFpI = fopen(lc1Buf, "r");
	sprintf(lc1Buf, ".\\Test\\A%d.txt", piTNo);
	szpFpA = fopen(lc1Buf, "r");
	siRes = 0;
#else
	szpFpI = stdin;
#endif

	// N, M - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &siN, &siM);

	// 素因数分解
	fDivPrm();

	// 素数の最大個数 - 取得
	int liMax = 0;
	for (i = 0; i < siPCnt; i++) {
		if (liMax < sz1Prm[i].miCnt) {
			liMax = sz1Prm[i].miCnt;
		}
	}

	// 階乗リスト - 作成
	fMakeModFact(siN + liMax - 1);

	// パターン数 - 取得
	int liPtn = 1;
	for (i = 0; i < siPCnt; i++) {

		// 総数
		int liSum = siN + sz1Prm[i].miCnt - 1;

		// *n!
		int liVal = si1ModFact[liSum];

		// /r!
		int liDiv = fGetModPower(si1ModFact[sz1Prm[i].miCnt], D_MOD - 2);
		liVal = (int)((long long)liVal * (long long)liDiv % (long long)D_MOD);

		// /(n-r)!
		liDiv = fGetModPower(si1ModFact[liSum - sz1Prm[i].miCnt], D_MOD - 2);
		liVal = (int)((long long)liVal * (long long)liDiv % (long long)D_MOD);

		// パターン数 - 乗算
		liPtn = (long long)liPtn * (long long)liVal % (long long)D_MOD;
	}

	// 結果 - セット
	sprintf(lc1Out, "%d\n", liPtn);

	// 結果 - 表示
#ifdef D_TEST
	fgets(lc1Buf, sizeof(lc1Buf), szpFpA);
	if (strcmp(lc1Buf, lc1Out)) {
		siRes = -1;
	}
#else
	printf("%s", lc1Out);
#endif

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
		printf("OK %d\n", piTNo);
	}
	else {
		printf("NG %d\n", piTNo);
	}
#endif

	return 0;
}

int
main()
{

#ifdef D_TEST
	int i;
	for (i = D_TEST_SNO; i <= D_TEST_ENO; i++) {
		fMain(i);
	}
#else
	fMain(0);
#endif

	return 0;
}

