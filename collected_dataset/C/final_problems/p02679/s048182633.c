#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_MOD			1000000007								// 除数(10の9乗+7)
#define D_FISH_MAX		200005									// 最大魚数
#define D_VAL_CNT		2										// 値数
#define D_STS_CNT		4										// 状態数
#define D_STS_PU		0										// 状態 - ＋昇順
#define D_STS_MU		1										// 状態 - －昇順
#define D_STS_PD		2										// 状態 - ＋降順
#define D_STS_MD		3										// 状態 - －降順

// 内部構造体 - 魚情報
typedef struct Fish {
	long long ml1Val[D_VAL_CNT];								// 値
	int mi1Cnt[D_STS_CNT];										// 個数[状態]
} Fish;

// 内部変数
static FILE *szpFpI;											// 入力
static Fish sz1Fish[D_FISH_MAX];								// 魚
static int siFCnt;												// 魚数
static int si1Two[D_FISH_MAX];									// ２のべき乗

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

// 値 - 入れ替え
int
fSwap(
	long long *plpVal1			// <IO> 値１
	, long long *plpVal2		// <IO> 値２
)
{
	long long llWork = *plpVal1;
	*plpVal1 = *plpVal2;
	*plpVal2 = llWork;

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

// 最大公約数 - 取得
long long
fGetGcd(
	long long plVal1			// <I> 値１
	, long long plVal2			// <I> 値２
)
{
	long long llWork;

	// ユークリッドの互除法
	if (plVal1 < plVal2) {
		llWork = plVal1;
		plVal1 = plVal2;
		plVal2 = llWork;
	}
	while (plVal2 > 0) {
		llWork = plVal1 % plVal2;
		plVal1 = plVal2;
		plVal2 = llWork;
	}

	return plVal1;
}

// 比較関数 - 値昇順
int
fCmpFnc(
	const void *pzpVal1			// <I> 値１
	, const void *pzpVal2		// <I> 値２
)
{
	Fish *lzpVal1 = (Fish *)pzpVal1;
	Fish *lzpVal2 = (Fish *)pzpVal2;

	// 値昇順
	return memcmp(lzpVal1->ml1Val, lzpVal2->ml1Val, sizeof(lzpVal1->ml1Val));
}

// 魚 - 追加
int
fAddFish(
	long long plVal1			// <I> 値１
	, long long plVal2			// <I> 値２
	, int piSts					// <I> 状態
)
{
	sz1Fish[siFCnt].ml1Val[0] = plVal1;
	sz1Fish[siFCnt].ml1Val[1] = plVal2;
	sz1Fish[siFCnt].mi1Cnt[piSts] = 1;
	siFCnt++;

	return 0;
}

// 魚 - 重複カット
int
fCutFish(
)
{
	int i, j, liRet;

	// 魚 - ソート
	qsort(sz1Fish, siFCnt, sizeof(Fish), fCmpFnc);

	int liTo = 1;
	int liDCnt = 0;
	for (i = 1; i < siFCnt; i++) {

		// 比較
		liRet = fCmpFnc(&sz1Fish[i], &sz1Fish[liTo - 1]);
		if (liRet == 0) {
			for (j = 0; j < D_STS_CNT; j++) {
				sz1Fish[liTo - 1].mi1Cnt[j] += sz1Fish[i].mi1Cnt[j];
			}
			liDCnt++;
		}
		else {
			sz1Fish[liTo] = sz1Fish[i];
			liTo++;
		}
	}
	siFCnt -= liDCnt;

	return liDCnt;
}

// 実行メイン
int
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// データ - 初期化
	memset(sz1Fish, 0, sizeof(sz1Fish));
	siFCnt = 0;

	// 魚数 - 取得
	int liFCnt;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d", &liFCnt);

	// ２のべき乗 - セット
	si1Two[0] = 1;
	for (i = 1; i <= liFCnt; i++) {
		si1Two[i] = fGetModMulti(si1Two[i - 1], 2);
	}

	// 魚 - 取得
	for (i = 0; i < liFCnt; i++) {
		long long llVal1, llVal2;
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
		sscanf(lc1Buf, "%lld%lld", &llVal1, &llVal2);

		// 選択不可
		if (llVal1 == 0 && llVal2 == 0) {
			continue;
		}

		// ０昇順
		if (llVal1 == 0) {
			fAddFish(0, 0, D_STS_PU);
			continue;
		}

		// ０降順
		if (llVal2 == 0) {
			fAddFish(0, 0, D_STS_MU);
			continue;
		}

		// 符号
		int liSts = 0;
		if (llVal1 * llVal2 < 0) {
			liSts += 1;
		}
		llVal1 = llabs(llVal1);
		llVal2 = llabs(llVal2);

		// 降順
		if (llVal1 > llVal2) {
			fSwap(&llVal1, &llVal2);
			liSts += 2;
		}

		// 最大公約数 - 取得
		long long llGcd = fGetGcd(llVal1, llVal2);
		llVal1 /= llGcd;
		llVal2 /= llGcd;

		// 魚 - 追加
		fAddFish(llVal1, llVal2, liSts);
	}

	// 魚 - 重複カット
	fCutFish();

	// パターン数 - 取得
	int liSum = 1;
	for (i = 0; i < siFCnt; i++) {

		// 選択なし
		int liPCnt = 1;

		// ０ or 同値
		if (sz1Fish[i].ml1Val[0] == 0 || sz1Fish[i].ml1Val[0] == sz1Fish[i].ml1Val[1]) {
			liPCnt = fGetModSum(liPCnt, si1Two[sz1Fish[i].mi1Cnt[D_STS_PU]] - 1);
			liPCnt = fGetModSum(liPCnt, si1Two[sz1Fish[i].mi1Cnt[D_STS_MU]] - 1);
		}

		// その他
		else {
			int liVal;

			// ＋昇順のみ
			int liPU = si1Two[sz1Fish[i].mi1Cnt[D_STS_PU]] - 1;
			liPCnt = fGetModSum(liPCnt, liPU);

			// －昇順のみ
			int liMU = si1Two[sz1Fish[i].mi1Cnt[D_STS_MU]] - 1;
			liPCnt = fGetModSum(liPCnt, liMU);

			// ＋降順のみ
			int liPD = si1Two[sz1Fish[i].mi1Cnt[D_STS_PD]] - 1;
			liPCnt = fGetModSum(liPCnt, liPD);

			// －降順のみ
			int liMD = si1Two[sz1Fish[i].mi1Cnt[D_STS_MD]] - 1;
			liPCnt = fGetModSum(liPCnt, liMD);

			// ＋昇順・－昇順
			liVal = fGetModMulti(liPU, liMU);
			liPCnt = fGetModSum(liPCnt, liVal);

			// ＋昇順・＋降順
			liVal = fGetModMulti(liPU, liPD);
			liPCnt = fGetModSum(liPCnt, liVal);

			// －昇順・－降順
			liVal = fGetModMulti(liMU, liMD);
			liPCnt = fGetModSum(liPCnt, liVal);

			// ＋降順・－降順
			liVal = fGetModMulti(liPD, liMD);
			liPCnt = fGetModSum(liPCnt, liVal);
		}

		// 乗算
		liSum = fGetModMulti(liSum, liPCnt);
	}

	return liSum - 1;
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

