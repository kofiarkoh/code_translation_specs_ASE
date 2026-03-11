#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_VTX_MAX		100005									// 最大頂点数
#define D_EDGE_MAX		100000									// 最大辺数
#define D_GRP_CNT		3										// グループ数
#define D_GRP_NONE		0										// グループ番号 - なし
#define D_GRP_1			1										// グループ番号 - １
#define D_GRP_2			2										// グループ番号 - ２

// 内部構造体 - 辺情報
typedef struct Edge {
	int miVNo;													// 接続先頂点
	struct Edge *mzpNext;										// 次の辺情報
} Edge;

// 内部構造体 - 頂点情報
typedef struct Vtx {
	Edge *mzpEdge;												// 辺
	int miECnt;													// 辺数
	int miGNo;													// グループ番号
} Vtx;

// 内部変数
static FILE *szpFpI;											// 入力
static Vtx sz1Vtx[D_VTX_MAX];									// 頂点
static int siVCnt;												// 頂点数
static Edge sz1Edge[D_EDGE_MAX * 2];							// 辺
static int siECnt;												// 辺数

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

// 辺 - 追加
int
fAddEdge(
	int piVFNo					// <I> 頂点 - 元
	, int piVTNo				// <I> 頂点 - 先
)
{
	sz1Edge[siECnt].miVNo = piVTNo;
	sz1Edge[siECnt].mzpNext = sz1Vtx[piVFNo].mzpEdge;
	sz1Vtx[piVFNo].mzpEdge = &sz1Edge[siECnt];
	sz1Vtx[piVFNo].miECnt++;
	siECnt++;

	return 0;
}

// グループ番号 - セット
int
fSetGNo(
	int piVNo					// <I> 頂点
	, int piGNo					// <I> グループ番号
)
{
	// グループ番号
	if (sz1Vtx[piVNo].miGNo == piGNo) {
		return 0;
	}
	sz1Vtx[piVNo].miGNo = piGNo;

	// 辺でループ
	Edge *lzpEdge = sz1Vtx[piVNo].mzpEdge;
	while (lzpEdge != NULL) {

		// 下位へ
		fSetGNo(lzpEdge->miVNo, piGNo);

		// 次の辺へ
		lzpEdge = lzpEdge->mzpNext;
	}

	return 0;
}

// 実行メイン
long long
fMain(
)
{
	int i;
	char lc1Buf[1024];

	// 頂点数・辺数 - 取得
	int liECnt;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d", &siVCnt, &liECnt);

	// 辺 - 取得
	for (i = 0; i < liECnt; i++) {
		int liVNo1, liVNo2;
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
		sscanf(lc1Buf, "%d%d", &liVNo1, &liVNo2);

		// 辺 - 追加
		fAddEdge(liVNo1, liVNo2);
		fAddEdge(liVNo2, liVNo1);
	}

	// グループ番号 - セット
	fSetGNo(1, D_GRP_1);
	fSetGNo(2, D_GRP_2);

	// 各グループ数 - 取得
	int li1GCnt[D_GRP_CNT];
	memset(li1GCnt, 0, sizeof(li1GCnt));
	for (i = 1; i <= siVCnt; i++) {
		li1GCnt[sz1Vtx[i].miGNo]++;
	}

	// グループ内の加算辺
	long long llSum = 0;
	for (i = 1; i <= siVCnt; i++) {
		llSum += li1GCnt[sz1Vtx[i].miGNo] - 1 - sz1Vtx[i].miECnt;
	}
	llSum /= 2;

	// グループなし
	if (li1GCnt[D_GRP_1] > li1GCnt[D_GRP_2]) {
		llSum += (long long)li1GCnt[D_GRP_1] * (long long)li1GCnt[D_GRP_NONE];
	}
	else {
		llSum += (long long)li1GCnt[D_GRP_2] * (long long)li1GCnt[D_GRP_NONE];
	}

	return llSum;
}

// １回実行
int
fOne(
)
{
	long long llRet;
	char lc1Buf[1024];

	// データ - 初期化
	memset(sz1Vtx, 0, sizeof(sz1Vtx));							// 頂点
	siECnt = 0;													// 辺数

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
	llRet = fMain();

	// 出力
	sprintf(lc1Buf, "%lld\n", llRet);
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

