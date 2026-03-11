#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_N_MAX			100000									// 最大N
#define D_HEAP_UP		1										// ヒープ - 昇順
#define D_HEAP_DOWN		-1										// ヒープ - 降順

// 内部変数
static int si1Array[D_N_MAX * 3];								// 配列
static int siACnt;												// 配列数(1/3)
static int si1Heap[D_N_MAX + 5];								// ヒープ
static int siHCnt;												// ヒープ数
static long long sl1SumF[D_N_MAX + 5];							// 合計リスト - 前半
static long long sl1SumS[D_N_MAX + 5];							// 合計リスト - 後半

// 内部変数 - テスト用
#ifdef D_TEST
	static int siRes;
	static FILE *szpFpT, *szpFpA;
#endif

// ヒープ - 親子関係チェック
// 戻り値：[>=0]:変更した子の配列番号 [-1]:変更なし
int
fHeapChk(
	int piMode					// <I> 昇順 or 降順
	, int piPNo					// <I> 親の配列番号 0～
)
{
	// 最小値 or 最大値
	int liMNo = piPNo;

	// 左の子と比較
	int liCNo = piPNo * 2 + 1;
	if (liCNo < siHCnt) {
		if (si1Heap[liMNo] * piMode > si1Heap[liCNo] * piMode) {
			liMNo = liCNo;
		}
	}

	// 右の子と比較
	liCNo = piPNo * 2 + 2;
	if (liCNo < siHCnt) {
		if (si1Heap[liMNo] * piMode > si1Heap[liCNo] * piMode) {
			liMNo = liCNo;
		}
	}

	// 自分が最小値(最大値)であるかチェック
	if (piPNo == liMNo) {
		return -1;
	}

	// 値の交換
	int liWork;
	liWork = si1Heap[liMNo];
	si1Heap[liMNo] = si1Heap[piPNo];
	si1Heap[piPNo] = liWork;

	return liMNo;
}

// ヒープ - キュー追加
int
fHeapEnqueue(
	int piMode					// <I> 昇順 or 降順
	, int piVal					// <I> 値
)
{
	int liRet;

	// 末尾に追加
	si1Heap[siHCnt] = piVal;
	siHCnt++;

	// 親子関係チェック
	int liNo = siHCnt - 1;
	while (1) {

		// 親の配列番号
		liNo = (liNo - 1) / 2;

		// 親子関係チェック
		liRet = fHeapChk(piMode, liNo);
		if (liRet < 0) {
			break;
		}
	}

	return 0;
}

// ヒープ - キュー取得
int
fHeapDequeue(
	int piMode					// <I> 昇順 or 降順
	, int *pipRet				// <O> 取得値
)
{
	// データ数
	if (siHCnt < 1) {
		return -1;
	}

	// 取得
	*pipRet = si1Heap[0];
	siHCnt--;

	// データ数
	if (siHCnt < 1) {
		return 0;
	}

	// 末尾を先頭へ
	si1Heap[0] = si1Heap[siHCnt];

	// 親子関係チェック
	int liNo = 0;
	while (liNo >= 0) {
		liNo = fHeapChk(piMode, liNo);
	}

	return 0;
}

// 合計リストの作成
int
fMakeSum(
	int piMode					// <I> 昇順 or 降順
)
{
	int i;

	// ヒープ数
	siHCnt = 0;

	// 配列開始位置・対象配列
	int liNo;
	long long *llpSum;
	if (piMode == D_HEAP_UP) {
		liNo = 0;
		llpSum = sl1SumF;
	}
	else {
		liNo = siACnt * 3 - 1;
		llpSum = sl1SumS;
	}

	// １つ目の合計値作成・ヒープ作成
	llpSum[0] = 0;
	for (i = 0; i < siACnt; i++) {

		// 合計値
		llpSum[0] += (long long)si1Array[liNo];

		// ヒープに追加
		fHeapEnqueue(piMode, si1Array[liNo]);

		// 次の位置
		liNo += piMode;
	}

	// ２つ目以降の合計値作成
	for (i = 1; i <= siACnt; i++) {

		// ヒープに追加
		fHeapEnqueue(piMode, si1Array[liNo]);

		// ヒープ - キュー取得
		int liVal;
		fHeapDequeue(piMode, &liVal);

		// 合計値 = 差を加算
		llpSum[i] = llpSum[i - 1] + (long long)si1Array[liNo] - (long long)liVal;

		// 次の位置
		liNo += piMode;
	}

	return 0;
}

// 実行メイン
int
fMain(
	int piTNo					// <I> テスト番号 1～
)
{
	int i;
	char lc1Buf[1024], lc1Out[1024];

	// テストファイルオープン
#ifdef D_TEST
	sprintf(lc1Buf, ".\\Test\\T%d.txt", piTNo);
	szpFpT = fopen(lc1Buf, "r");
	sprintf(lc1Buf, ".\\Test\\A%d.txt", piTNo);
	szpFpA = fopen(lc1Buf, "r");
	siRes = 0;
#endif

	// 配列数(1/3)取得
#ifdef D_TEST
	fgets(lc1Buf, sizeof(lc1Buf), szpFpT);
#else
	fgets(lc1Buf, sizeof(lc1Buf), stdin);
#endif
	sscanf(lc1Buf, "%d", &siACnt);

	// 配列取得
	for (i = 0; i < siACnt * 3; i++) {
#ifdef D_TEST
		fscanf(szpFpT, "%d", &si1Array[i]);
#else
		fscanf(stdin, "%d", &si1Array[i]);
#endif
	}
#ifdef D_TEST
	fgets(lc1Buf, sizeof(lc1Buf), szpFpT);
#else
	fgets(lc1Buf, sizeof(lc1Buf), stdin);
#endif

	// 合計リストの作成
	fMakeSum(D_HEAP_UP);
	fMakeSum(D_HEAP_DOWN);

	// 最大の差を取得
	long long llMax = LLONG_MIN;
	for (i = 0; i <= siACnt; i++) {
		long long llDF = sl1SumF[i] - sl1SumS[siACnt - i];
		if (llMax < llDF) {
			llMax = llDF;
		}
	}

	// 結果セット
	sprintf(lc1Out, "%lld\n", llMax);

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

