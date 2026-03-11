#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_TV_MAX		100000									// 最大テレビ番組数
#define D_CH_MAX		30										// 最大チャンネル数

// 内部構造体 - 時刻情報
typedef struct Time {
	int miTime;													// 時刻
	int miType;													// タイプ [0]:開始 [1]:終了
} Time;

// 内部変数
static int si2Time[D_CH_MAX][D_TV_MAX * 2];						// チャンネル別時刻
static int si1TCnt[D_CH_MAX];									// チャンネル別時刻数
static Time sz1Time[D_TV_MAX * 2];								// 全時刻
static int siTCnt;												// 全時刻数

// 内部変数 - テスト用
#ifdef D_TEST
	static int siRes;
	static FILE *szpFpT, *szpFpA;
#endif

// ソート関数 - int昇順
int
fSortFncInt(
	const void *pzpVal1			// <I> 値１
	, const void *pzpVal2		// <I> 値２
)
{
	int *lipVal1 = (int *)pzpVal1;
	int *lipVal2 = (int *)pzpVal2;

	// 昇順
	if (*lipVal1 >  *lipVal2) {
		return(1);
	}
	else if (*lipVal1 < *lipVal2) {
		return(-1);
	}

	return 0;
}

// ソート関数 - 時刻昇順 - タイプ降順
int
fSortFncTime(
	const void *pzpVal1			// <I> 値１
	, const void *pzpVal2		// <I> 値２
)
{
	Time *lzpVal1 = (Time *)pzpVal1;
	Time *lzpVal2 = (Time *)pzpVal2;

	// 時刻昇順
	if (lzpVal1->miTime > lzpVal2->miTime) {
		return(1);
	}
	else if (lzpVal1->miTime < lzpVal2->miTime) {
		return(-1);
	}

	// タイプ降順
	if (lzpVal1->miType > lzpVal2->miType) {
		return(-1);
	}
	else if (lzpVal1->miType < lzpVal2->miType) {
		return(1);
	}

	return 0;
}

// 実行メイン
int
fMain(
	int piTNo					// <I> テスト番号 1～
)
{
	int i, j;
	char lc1Buf[1024], lc1Out[1024];

	// データ初期化
	memset(si1TCnt, 0, sizeof(si1TCnt));						// チャンネル別時刻数
	siTCnt = 0;													// 全時刻数

	// テストファイルオープン
#ifdef D_TEST
	sprintf(lc1Buf, ".\\Test\\T%d.txt", piTNo);
	szpFpT = fopen(lc1Buf, "r");
	sprintf(lc1Buf, ".\\Test\\A%d.txt", piTNo);
	szpFpA = fopen(lc1Buf, "r");
	siRes = 0;
#endif

	// テレビ番組数・チャンネル数取得
	int liTCnt, liCCnt;
#ifdef D_TEST
	fgets(lc1Buf, sizeof(lc1Buf), szpFpT);
#else
	fgets(lc1Buf, sizeof(lc1Buf), stdin);
#endif
	sscanf(lc1Buf, "%d%d", &liTCnt, &liCCnt);

	// テレビ番組取得
	for (i = 0; i < liTCnt; i++) {
		int liStart, liEnd, liCh;
#ifdef D_TEST
		fgets(lc1Buf, sizeof(lc1Buf), szpFpT);
#else
		fgets(lc1Buf, sizeof(lc1Buf), stdin);
#endif
		sscanf(lc1Buf, "%d%d%d", &liStart, &liEnd, &liCh);
		liCh--;

		// チャンネル別時刻 - セット
		si2Time[liCh][si1TCnt[liCh]] = liStart;
		si1TCnt[liCh]++;
		si2Time[liCh][si1TCnt[liCh]] = liEnd;
		si1TCnt[liCh]++;
	}

	// チャンネル別時刻 - ソート
	for (i = 0; i < liCCnt; i++) {
		qsort(si2Time[i], si1TCnt[i], sizeof(int), fSortFncInt);
	}

	// 全時刻 - セット
	for (i = 0; i < liCCnt; i++) {

		// 時刻タイプ
		int liType = 0;										// 開始時刻

		// テレビ番組数でループ
		for (j = 0; j < si1TCnt[i]; j++) {
			if (liType == 0) {									// 開始時刻
				sz1Time[siTCnt].miType = 0;
				sz1Time[siTCnt].miTime = si2Time[i][j] * 2 - 1;
			}
			else {												// 終了時刻

				// 重複チェック
				if (j < si1TCnt[i] - 1) {
					if (si2Time[i][j] == si2Time[i][j + 1]) {			// 次の開始時刻と同一
						j++;												// 次の開始時刻もスキップ
						continue;
					}
				}

				sz1Time[siTCnt].miType = 1;
				sz1Time[siTCnt].miTime = si2Time[i][j] * 2;
			}
			siTCnt++;

			// 時刻タイプ変更
			liType = (liType + 1) % 2;
		}
	}

	// 全時刻 - ソート
	qsort(sz1Time, siTCnt, sizeof(Time), fSortFncTime);

	// 最大個数 - 取得
	int liMax = 0;
	int liCnt = 0;
	for (i = 0; i < siTCnt; i++) {
		if (sz1Time[i].miType == 0) {						// 開始時刻
			liCnt++;

			// 最大個数更新
			if (liMax < liCnt) {
				liMax = liCnt;
			}
		}
		else {												// 終了時刻
			liCnt--;
		}
	}

	// 結果セット
	sprintf(lc1Out, "%d\n", liMax);

	// 結果表示
#ifdef D_TEST
	fgets(lc1Buf, sizeof(lc1Buf), szpFpA);
	if (strcmp(lc1Buf, lc1Out)) {
		siRes = -1;
	}
#else
	printf("%s", lc1Out);
#endif

	// テストファイルクローズ
#ifdef D_TEST
	fclose(szpFpT);
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

