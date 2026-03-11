#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_ROW_MAX		500000									// 最大行数
#define D_COL_CNT		7										// 列数
#define D_STACK_MAX		D_ROW_MAX * D_COL_CNT					// 最大スタック数

// 内部構造体 - スタック情報
typedef struct Stack {
	int miRow, miCol;											// 行・列
} Stack;

// 内部変数
static FILE *szpFpI;											// 入力
static int siRow;												// 行数
static int siDiv;												// 除数
static int si2Mass[D_ROW_MAX][D_COL_CNT];						// マス
static int siRMax;												// 最大行
static int siGCnt;												// グループ数
static Stack sz1Stack[D_STACK_MAX];								// スタック
static int siSCnt;												// スタック数

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

// スタック - 追加
int
fStackAdd(
	int piRow					// <I> 行
	, int piCol					// <I> 列
	, int piGNo					// <I> グループ番号
)
{
	// 範囲チェック
	if (piRow < 0 || siRMax <= piRow) {
		return -1;
	}
	if (piCol < 0 || D_COL_CNT <= piCol) {
		return -1;
	}

	// グループ番号
	if (si2Mass[piRow][piCol] != 0) {
		return -1;
	}
	si2Mass[piRow][piCol] = piGNo;

	// セット
	sz1Stack[siSCnt].miRow = piRow;
	sz1Stack[siSCnt].miCol = piCol;

	// スタック数
	siSCnt++;

	return 0;
}

// グループ化
int
fSetGrp(
	int piSRow					// <I> 開始行
	, int piERow				// <I> 終了行
)
{
	int i, j, liRet;

	for (i = piSRow; i <= piERow; i++) {
		for (j = 0; j < D_COL_CNT; j++) {

			// スタック - 追加
			liRet = fStackAdd(i, j, siGCnt + 1);
			if (liRet != 0) {
				continue;
			}
			siGCnt++;

			// 上下左右へ
			while (siSCnt > 0) {
				int liRow = sz1Stack[siSCnt - 1].miRow;
				int liCol = sz1Stack[siSCnt - 1].miCol;
				siSCnt--;
				fStackAdd(liRow + 1, liCol, siGCnt);
				fStackAdd(liRow - 1, liCol, siGCnt);
				fStackAdd(liRow, liCol + 1, siGCnt);
				fStackAdd(liRow, liCol - 1, siGCnt);
			}
		}
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
	memset(si2Mass, 0, sizeof(si2Mass));
	siGCnt = 0;
	siSCnt = 0;

	// 行数・除数・塗る回数 - 取得
	int liDCnt;
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);
	sscanf(lc1Buf, "%d%d%d", &siRow, &siDiv, &liDCnt);

	// 最大行
	if (siRow <= siDiv * 2) {
		siRMax = siRow;
	}
	else {
		siRMax = siDiv * 2;
	}

	// 塗る位置 - 取得
	for (i = 0; i < liDCnt; i++) {
		int liCol;
		fscanf(szpFpI, "%d", &liCol);

		// マス - 黒化
		int liRow = 0;
		while (1) {

			// 行移動
			liRow += liCol / D_COL_CNT;
			liCol %= D_COL_CNT;
			if (liRow >= siRMax) {
				break;
			}

			// 塗る
			si2Mass[liRow][liCol] = -1;

			// 次へ
			liCol += siDiv;
		}
	}
	fgets(lc1Buf, sizeof(lc1Buf), szpFpI);

	// 小規模
	if (siRow <= siDiv * 2) {
		fSetGrp(0, siRow - 1);
		return siGCnt;
	}

	// １つでのグループ数
	fSetGrp(0, siDiv - 1);
	int liGCnt = siGCnt;

	// １つ増分のグループ数
	fSetGrp(siDiv, siDiv * 2 - 1);
	int liGCnt2 = siGCnt - liGCnt;

	// 合計
	return liGCnt + (siRow / siDiv - 1) * liGCnt2;
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

