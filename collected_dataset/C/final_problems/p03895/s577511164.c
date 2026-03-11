#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_DAY_SEC		86400									// １日の秒
#define D_3H_SEC		10800									// ３時間の秒

// 内部変数
static FILE *szpFpI;											// 入力
static int si1Cnt[D_DAY_SEC * 2];								// 起床回数[秒]

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
	int i;
	char lc1Buf[1024];

	// 起床回数 - 取得
	int liCnt;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d", &liCnt);

	// 秒 - 取得
	int liTime = 0;
	for (i = 0; i < liCnt; i++) {
		int liTime1, liTime2;
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
		sscanf(lc1Buf, "%d%d", &liTime1, &liTime2);

		// 起床秒
		liTime = (liTime + liTime1) % D_DAY_SEC;

		// 起床回数 - 加算
		si1Cnt[liTime]++;

		// 時間経過
		liTime += liTime2;
	}

	// 起床回数 - 拡張
	memcpy(&si1Cnt[D_DAY_SEC], si1Cnt, sizeof(int) * D_DAY_SEC);

	// 起床回数 - 累積和
	for (i = 1; i < D_DAY_SEC * 2; i++) {
		si1Cnt[i] += si1Cnt[i - 1];
	}

	// 最大値 - 取得
	int liMax = si1Cnt[D_3H_SEC];
	for (i = 1; i < D_DAY_SEC; i++) {
		liCnt = si1Cnt[i + D_3H_SEC] - si1Cnt[i - 1];
		if (liMax < liCnt) {
			liMax = liCnt;
		}
	}

	return liMax;
}

// １回実行
int
fOne(
)
{
	int liRet;
	char lc1Buf[1024];

	// データ - 初期化
	memset(si1Cnt, 0, sizeof(si1Cnt));							// 起床回数

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
	if (liRet == 0) {
		sprintf(lc1Buf, "Yes\n");
	}
	else {
		sprintf(lc1Buf, "No\n");
	}
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

