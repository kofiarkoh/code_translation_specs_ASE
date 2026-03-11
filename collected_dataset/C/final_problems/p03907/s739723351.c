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
#define D_VTX_MAX		100000									// 最大頂点数
#define D_TREE_MAX		D_VTX_MAX								// 木の最大データ数
#define D_TREE_WCNT		2										// 木の方向数
#define D_TREE_LEFT		0										// 木の方向 - 左側
#define D_TREE_RIGHT	1										// 木の方向 - 右側

// 内部構造体 - 頂点情報
typedef struct Vtx {
	int miLen1;													// 頂点１への距離
	int miLen2;													// 頂点２への距離
	int miSum;													// 合計距離
} Vtx;

// 内部構造体 - 木構造
typedef struct Tree {
	Vtx *mzpVtx;												// 頂点
	int mi1Height[D_TREE_WCNT];									// 木の高さ
	struct Tree *mzp1Child[D_TREE_WCNT];						// 子
} Tree;

// 内部変数
static FILE *szpFpI;											// 入力
static Vtx sz1Vtx[D_VTX_MAX];									// 頂点
static int siVCnt;												// 頂点数
static int siVLen;												// 頂点１・２間の距離
static Tree sz1Tree[D_TREE_MAX];								// 木の実データ
static int siTCnt;												// 木の実データ数
static Tree *szpTop;											// 先頭の木データ

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

// ソート関数 - 合計距離昇順
int
fSortFnc(
	const void *pzpVal1			// <I> 値１
	, const void *pzpVal2		// <I> 値２
)
{
	Vtx *lzpVal1 = (Vtx *)pzpVal1;
	Vtx *lzpVal2 = (Vtx *)pzpVal2;

	// 合計距離昇順
	if (lzpVal1->miSum > lzpVal2->miSum) {
		return 1;
	}
	else if (lzpVal1->miSum < lzpVal2->miSum) {
		return -1;
	}

	return 0;
}

// 木データ - 作成
Tree *
fTreeMake(
	Vtx *pzpVtx					// <I> 頂点
)
{
	// 対象の木データ
	Tree *lzpTree = &(sz1Tree[siTCnt]);
	siTCnt++;

	// データセット
	memset(lzpTree, 0, sizeof(Tree));					// 初期化
	lzpTree->mzpVtx = pzpVtx;

	return lzpTree;
}

// 木データ - 比較 - 距離昇順
int
fTreeCmp(
	Tree *pzpTree				// <I> 木データ
	, int piLen1				// <I> 頂点１への距離
	, int piLen2				// <I> 頂点２への距離
)
{
	// 頂点
	Vtx *lzpVtx = pzpTree->mzpVtx;

	// 距離昇順
	if (piLen1 < lzpVtx->miLen1) {			// 左側
		return -1;
	}
	else if (piLen1 > lzpVtx->miLen1) {		// 右側
		return 1;
	}
	if (piLen2 < lzpVtx->miLen2) {			// 左側
		return -1;
	}
	else if (piLen2 > lzpVtx->miLen2) {		// 右側
		return 1;
	}

	return 0;
}

// 木データ - 検索
Tree *
fTreeSrh(
	int piLen1					// <I> 頂点１への距離
	, int piLen2				// <I> 頂点２への距離
)
{
	// 先頭の木データ
	Tree *lzpNow = szpTop;

	// 検索
	while (1) {

		// データ有無
		if (lzpNow == NULL) {
			return NULL;
		}

		// 比較
		int liRet = fTreeCmp(lzpNow, piLen1, piLen2);
		if (liRet == 0) {								// 一致
			return lzpNow;
		}

		// 子へ移動
		if (liRet < 0) {								// 左側
			lzpNow = lzpNow->mzp1Child[D_TREE_LEFT];
		}
		else {											// 右側
			lzpNow = lzpNow->mzp1Child[D_TREE_RIGHT];
		}
	}

	return NULL;
}

// 木データ - 高さ取得
int
fTreeGetHeight(
	Tree *pzpTree				// <I> 対象の木情報
)
{
	// データ有無
	if (pzpTree == NULL) {
		return 0;
	}

	if (pzpTree->mi1Height[D_TREE_LEFT] >= pzpTree->mi1Height[D_TREE_RIGHT]) {
		return pzpTree->mi1Height[D_TREE_LEFT] + 1;
	}
	else {
		return pzpTree->mi1Height[D_TREE_RIGHT] + 1;
	}
}

// 木データ - 右回転(親は子[左]の右下へ・子[左]は親の左上へ)
int
fTreeRttR(
	Tree **pzppTree				// <I> 回転対象
)
{
	// 現在の子[左]
	Tree *lzpChild = (*pzppTree)->mzp1Child[D_TREE_LEFT];

	// 右回転
	(*pzppTree)->mzp1Child[D_TREE_LEFT] = lzpChild->mzp1Child[D_TREE_RIGHT];	// 親の左側 = 子[左]の右側
	(*pzppTree)->mi1Height[D_TREE_LEFT] = lzpChild->mi1Height[D_TREE_RIGHT];	// 親の高さ(左) = 子[左]の高さ(右)
	lzpChild->mzp1Child[D_TREE_RIGHT] = *pzppTree;								// 子[左]の右側 = 親
	lzpChild->mi1Height[D_TREE_RIGHT] = fTreeGetHeight(*pzppTree);				// 子[左]の高さ(右) - 親の高さ
	*pzppTree = lzpChild;														// 親 = 子[左]

	return 0;
}

// 木データ - 左回転(親は子[右]の左下へ・子[右]は親の右上へ)
int
fTreeRttL(
	Tree **pzppTree				// <I> 回転対象
)
{
	// 現在の子[右]
	Tree *lzpChild = (*pzppTree)->mzp1Child[D_TREE_RIGHT];

	// 左回転
	(*pzppTree)->mzp1Child[D_TREE_RIGHT] = lzpChild->mzp1Child[D_TREE_LEFT];	// 親の右側 = 子[右]の左側
	(*pzppTree)->mi1Height[D_TREE_RIGHT] = lzpChild->mi1Height[D_TREE_LEFT];	// 親の高さ(右) = 子[右]の高さ(左)
	lzpChild->mzp1Child[D_TREE_LEFT] = *pzppTree;								// 子[右]の左側 = 親
	lzpChild->mi1Height[D_TREE_LEFT] = fTreeGetHeight(*pzppTree);				// 子[右]の高さ(左) - 親の高さ
	*pzppTree = lzpChild;														// 親 = 子[右]

	return 0;
}

// 木データ - 追加・削除の共通処理
// 戻り値：[2]高さの変更あり [1]高さの変更なし
int
fTreeComAddDel(
	Tree **pzppNow				// <I> 現在の木情報
	, int piWay					// <I> 対象の方向
)
{
	// 高さの変更があるかチェック
	int liNew = fTreeGetHeight((*pzppNow)->mzp1Child[piWay]);
	if ((*pzppNow)->mi1Height[piWay] == liNew) {												// 変化なし
		return 1;
	}
	(*pzppNow)->mi1Height[piWay] = liNew;														// 更新

	// 高さが離れている場合、回転
	if ((*pzppNow)->mi1Height[D_TREE_LEFT] - (*pzppNow)->mi1Height[D_TREE_RIGHT] > 1) {			// 左側が高い
		fTreeRttR(pzppNow);																			// 右回転
	}
	else if ((*pzppNow)->mi1Height[D_TREE_RIGHT] - (*pzppNow)->mi1Height[D_TREE_LEFT] > 1) {	// 右側が高い
		fTreeRttL(pzppNow);																			// 左回転
	}

	return 2;
}

// 木データ - 追加
// 戻り値：[2]高さの変更あり [1]高さの変更なし [0]追加なし
int
fTreeAdd(
	Tree **pzppNow				// <I> 現在の木情報
	, Vtx *pzpVtx				// <I> 頂点
)
{
	// 作成
	if (*pzppNow == NULL) {
		*pzppNow = fTreeMake(pzpVtx);
		return 2;
	}

	// 比較
	int liRet = fTreeCmp(*pzppNow, pzpVtx->miLen1, pzpVtx->miLen2);

	// 一致
	if (liRet == 0) {
		return 0;
	}

	// 方向の判別
	int liWay;
	if (liRet < 0) {															// 左側
		liWay = D_TREE_LEFT;
	}
	else {																		// 右側
		liWay = D_TREE_RIGHT;
	}

	// 下位へ
	liRet = fTreeAdd(&((*pzppNow)->mzp1Child[liWay]), pzpVtx);
	if (liRet < 2) {															// 高さの変更あり以外
		return liRet;
	}

	// 追加・削除の共通処理
	return fTreeComAddDel(pzppNow, liWay);
}

// 実行メイン
int
fMain(
)
{
	int i, j, liRet;
	char lc1Buf[1024];

	// データ - 初期化
	siTCnt = 0;
	szpTop = NULL;

	// 頂点数 - 取得
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d", &siVCnt);

	// 頂点 - 取得
	for (i = 0; i < siVCnt; i++) {
		fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
		sscanf(lc1Buf, "%d%d", &sz1Vtx[i].miLen1, &sz1Vtx[i].miLen2);
		sz1Vtx[i].miSum = sz1Vtx[i].miLen1 + sz1Vtx[i].miLen2;
	}

	// 頂点１・２の距離
	if (sz1Vtx[0].miLen1 != 0) {
		return -1;
	}
	if (sz1Vtx[1].miLen2 != 0) {
		return -1;
	}
	if (sz1Vtx[0].miLen1 != sz1Vtx[1].miLen2) {
		return -1;
	}
	siVLen = sz1Vtx[0].miLen2;

	// 頂点 - ソート
	qsort(sz1Vtx, siVCnt, sizeof(Vtx), fSortFnc);

	// 頂点１・２間の距離より短い距離
	if (sz1Vtx[0].miSum < siVLen) {
		return -1;
	}

	// 距離でループ
	int liVNo = 0;
	sz1Vtx[siVCnt].miSum = INT_MAX;
	int liECnt = -2;
	for (i = siVLen; i <= siVCnt * 2; i++) {

		// 対象頂点 - 取得
		if (sz1Vtx[liVNo].miSum != i) {
			continue;
		}
		int liSNo = liVNo;
		int liENo;
		while (1) {
			liENo = liVNo;
			liVNo++;
			if (sz1Vtx[liVNo].miSum != i) {
				break;
			}
		}

		// 頂点でループ
		char mcDown = D_ON;
		int liACntLine = 0;
		int liACntAll = 0;
		for (j = liSNo; j <= liENo; j++) {

			// 頂点 - 追加
			liRet = fTreeAdd(&szpTop, &sz1Vtx[j]);
			if (liRet > 0) {
				liACntLine++;
			}
			liACntAll++;

			// 下位頂点の有無
			Tree *lzpTree = fTreeSrh(sz1Vtx[j].miLen1 - 1, sz1Vtx[j].miLen2 - 1);
			if (lzpTree == NULL) {
				mcDown = D_OFF;
			}
		}

		// 辺の本数 - 加算
		if (mcDown == D_ON) {				
			liECnt += liACntAll;
		}
		else {

			// 下位頂点のない頂点あり
			if (i == siVLen) {
				if (liACntLine != i + 1) {
					return -1;
				}
			}
			else {
				if (liACntLine != i - 1) {
					return -1;
				}
			}

			liECnt += liACntAll + 1;
		}
	}

	return liECnt;
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

