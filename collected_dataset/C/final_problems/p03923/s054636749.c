#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部変数
static FILE *szpFpI;											// 入力
static long long slCCnt;										// クッキー数
static long long slETime;										// 食事時間

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
long long
fGetMin(
	long long plVal1			// <I> 値１
	, long long plVal2			// <I> 値２
)
{
	if (plVal1 < plVal2) {
		return plVal1;
	}
	else {
		return plVal2;
	}
}

// 時間 - 取得
long long
fGetTime(
	int piECnt					// <I> 食事回数
	, long long plBCnt			// <I> 食事前の焼く枚数
)
{
	int i;

	// 食事
	long long llTime = 0;
	long long llMCnt = 1;
	for (i = 0; i < piECnt; i++) {
		llTime += plBCnt + slETime;
		if ((double)llMCnt * (double)plBCnt < LLONG_MAX) {
			llMCnt *= plBCnt;
		}
	}

	// 用意
	llTime += (slCCnt + llMCnt - 1) / llMCnt;

	return llTime;
}

// 最小時間 - 取得
long long
fGetMTime(
	int piECnt					// <I> 食事回数
)
{
	// 食事前の焼く枚数で絞る
	long long llBMin = 2;
	long long llBMax = slCCnt;
	long long llTMin = fGetTime(piECnt, llBMin);
	long long llTime = fGetTime(piECnt, llBMax);
	llTMin = fGetMin(llTMin, llTime);
	while (llBMin <= llBMax) {
		long long llDF = (llBMax - llBMin) / 3;
		if (llDF < 1) {
			llDF = 1;
		}

		// 枚数
		long long llBCnt1 = llBMin + llDF;
		long long llBCnt2 = llBMax - llDF;

		// 時間 - 取得
		long long llTime1 = fGetTime(piECnt, llBCnt1);
		long long llTime2 = fGetTime(piECnt, llBCnt2);

		// 結果
		if (llTime1 < llTime2) {
			llBMax = llBCnt2;
			llTMin = fGetMin(llTMin, llTime1);
		}
		else {
			llBMin = llBCnt1;
			llTMin = fGetMin(llTMin, llTime2);
		}
	}

	return llTMin;
}

// 実行メイン
int
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// クッキー数・食事時間 - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%lld%lld", &slCCnt, &slETime);

	// 食事回数でループ
	long long llTMin = fGetTime(0, 0);
	for (i = 1; ; i++) {
		long long llTime = fGetMTime(i);
		if (llTMin < llTime) {
			break;
		}
		else {
			llTMin = llTime;
		}
	}

	// 出力
	sprintf(lc1Buf, "%lld\n", llTMin);
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

