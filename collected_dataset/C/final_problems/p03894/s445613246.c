#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_CUP_MAX		100005									// 最大カップ数

// 内部変数
static FILE *szpFpI;											// 入力
static int si1Cup[D_CUP_MAX];									// カップ
static int siCCnt;												// カップ数

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

// 値 - 入れ替え
int
fSwap(
	int *pipVal1				// <IO> 値１
	, int *pipVal2				// <IO> 値２
)
{
	int liWork = *pipVal1;
	*pipVal1 = *pipVal2;
	*pipVal2 = liWork;

	return 0;
}

// 実行メイン
int
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// カップ数・操作数 - 取得
	int liOCnt;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &siCCnt, &liOCnt);

	// ボール位置
	int liBNo = 1;
	si1Cup[liBNo] = 1;

	for (i = 0; ; i++) {

		// 超能力 - 実行
		si1Cup[liBNo - 1] = 1;
		si1Cup[liBNo + 1] = 1;

		// 操作 - 取得
		if (i >= liOCnt) {
			break;
		}
		int liNo1, liNo2;
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
		sscanf(lc1Buf, "%d%d", &liNo1, &liNo2);
		fSwap(&si1Cup[liNo1], &si1Cup[liNo2]);

		// ボール位置
		if (liBNo == liNo1) {
			liBNo = liNo2;
		}
		else if (liBNo == liNo2) {
			liBNo = liNo1;
		}
	}

	// ボール数 - 取得
	int liSum = 0;
	for (i = 1; i <= siCCnt; i++) {
		liSum += si1Cup[i];
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

	// データ - 初期化
	memset(si1Cup, 0, sizeof(si1Cup));							// カップ

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

