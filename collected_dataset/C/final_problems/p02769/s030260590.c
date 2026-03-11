#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_MOD			1000000007								// 除数(10の9乗+7)
#define D_FACT_MAX		200005									// 最大階乗数

// 内部変数
static FILE *szpFpI;											// 入力
static int si1ModFact[D_FACT_MAX];								// 階乗リスト
static int si1ModFactR[D_FACT_MAX];								// 階乗リスト(逆元)

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

// 和 - 取得
int
fGetModSum(
	int piVal1					// <I> 値１
	, int piVal2				// <I> 値２
)
{
	return (piVal1 + piVal2) % D_MOD;
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
		li1Val[liCnt] = fGetModMulti(li1Val[liCnt - 1], li1Val[liCnt - 1]);
		liCnt++;
		liIdx += liIdx;
	}

	// べき乗 - 取得
	int liVal = 1;
	while (piIdx > 0) {
		if (piIdx >= liIdx) {
			piIdx -= liIdx;
			liVal = fGetModMulti(liVal, li1Val[liCnt - 1]);
		}
		liCnt--;
		liIdx /= 2;
	}

	return liVal;
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
		si1ModFact[i] = fGetModMulti(si1ModFact[i - 1], i);
	}

	return 0;
}

// 階乗リスト(逆元) - 作成
int
fMakeModFactR(
	int piMax					// <I> 最大値
)
{
	int i;

	for (i = 0; i <= piMax; i++) {
		si1ModFactR[i] = fGetModPower(si1ModFact[i], D_MOD - 2);
	}

	return 0;
}

// nCk - 取得
int
fGetModnCk(
	int piN						// <I> N
	, int piK					// <I> K
)
{
	if (piN < piK) {
		return 0;
	}
	if (piK < 0) {
		return 0;
	}
	int liCnt = fGetModMulti(si1ModFact[piN], si1ModFactR[piN - piK]);
	return fGetModMulti(liCnt, si1ModFactR[piK]);
}

// 実行メイン
int
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// 部屋数・移動回数 - 取得
	int liRCnt, liMCnt;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &liRCnt, &liMCnt);

	// 階乗リスト - 作成
	fMakeModFact(liRCnt);
	fMakeModFactR(liRCnt);

	// 無人の部屋数 - 最小
	int liMin;
	if (liMCnt % 2 == 0) {
		liMin = 0;
	}
	else {
		liMin = 1;
	}

	// 無人の部屋数 - 最大
	int liMax;
	if (liMCnt < liRCnt) {
		liMax = liMCnt;
	}
	else {
		liMax = liRCnt - 1;
	}

	// パターン数 - 取得
	int liSum = 0;
	for (i = liMin; i <= liMax; i++) {

		// 無人の部屋のパターン数
		int liPCnt = fGetModnCk(liRCnt, i);

		// 有人の部屋のパターン数
		int liPCnt2 = si1ModFact[liRCnt - 1];
		liPCnt2 = fGetModMulti(liPCnt2, si1ModFactR[i]);
		liPCnt2 = fGetModMulti(liPCnt2, si1ModFactR[liRCnt - i - 1]);

		// パターン数 - 加算
		liPCnt = fGetModMulti(liPCnt, liPCnt2);
		liSum = fGetModSum(liSum, liPCnt);
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

