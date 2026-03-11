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
#define D_HCNT_MAX		55										// 最大人数
#define D_HOUR_MAX		24										// 最大時刻

// 内部変数
static FILE *szpFpI;											// 入力
static int si1DF[D_HCNT_MAX];									// 時差
static int siDCnt;												// 時差数
static char sc1Time[D_HOUR_MAX];								// 時刻
static int siMax;												// 最大時間

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

// ソート関数 - int昇順
int
fSortFncIu(
	const void *pzpVal1			// <I> 値１
	, const void *pzpVal2		// <I> 値２
)
{
	int *lipVal1 = (int *)pzpVal1;
	int *lipVal2 = (int *)pzpVal2;

	// int昇順
	if (*lipVal1 > *lipVal2) {
		return 1;
	}
	else if (*lipVal1 < *lipVal2) {
		return -1;
	}

	return 0;
}

// 時刻 - 検索
int
fSrhTime(
	int piSNo					// <I> 開始位置
)
{
	int i;

	for (i = piSNo; ; i++) {
		if (i >= D_HOUR_MAX) {
			i = 0;
		}
		if (sc1Time[i] == D_ON) {
			return i;
		}
	}

	return 0;
}

// 時刻 - セット
int
fSetTime(
	int piDNo					// <I> 時差
)
{
	// 終了
	if (piDNo >= siDCnt) {

		// 開始位置
		int liStart = fSrhTime(0);
		int liNow = liStart;

		// 最小時間 - 取得
		int liMin = INT_MAX;
		while (1) {

			// 次の位置
			int liNext = fSrhTime(liNow + 1);

			// 時間
			int liTime = abs(liNext - liNow);
			if (liMin > liTime) {
				liMin = liTime;
			}

			// 次回用
			if (liNext == liStart) {
				break;
			}
			liNow = liNext;
		}

		// 最大時間 - 更新
		if (siMax < liMin) {
			siMax = liMin;
		}

		return 0;
	}

	// 時刻１
	int liDF = si1DF[piDNo];
	if (sc1Time[liDF] == D_OFF) {
		sc1Time[liDF] = D_ON;
		fSetTime(piDNo + 1);
		sc1Time[liDF] = D_OFF;
	}

	// 時刻２
	liDF = (24 - si1DF[piDNo]) % 24;
	if (sc1Time[liDF] == D_OFF) {
		sc1Time[liDF] = D_ON;
		fSetTime(piDNo + 1);
		sc1Time[liDF] = D_OFF;
	}

	return 0;
}

// 実行メイン
int
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// データ - 初期化
	memset(sc1Time, D_OFF, sizeof(sc1Time));
	siMax = 0;

	// 時差数 - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d", &siDCnt);

	// 時差 - 取得
	si1DF[0] = 0;
	for (i = 1; i <= siDCnt; i++) {
		fscanf(szpFpI, "%d", &si1DF[i]);
	}
	siDCnt++;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	qsort(si1DF, siDCnt, sizeof(int), fSortFncIu);

	// 時刻 - セット
	fSetTime(0);

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

	// 出力
	sprintf(lc1Buf, "%d\n", siMax);
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

